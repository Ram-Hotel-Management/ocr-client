/// This module is dedicated
/// for making requests to server
/// whcih is written in python is an independent
/// codebase
pub mod docling;
pub mod invoice;
use crate::{OCRServerErr, err::OcrResult};
use base64::{Engine, prelude::BASE64_STANDARD};
use docling::{OcrDoc, ParsedDoc};
use image::DynamicImage;
use invoice::{InvoiceDetails, InvoiceResponse};
use reqwest::{
    Client, Url,
    multipart::{Form, Part},
};
use serde::Deserialize;
use std::io::Cursor;

pub struct OcrClient {
    pub client: Client,
    pub base: Url,
}

impl OcrClient {
    /// Initialize ocr client
    pub async fn new<S: AsRef<str>>(addr: S) -> OcrResult<Self> {
        let base = Url::parse(addr.as_ref())?;
        let client = Client::builder().use_rustls_tls().build()?;
        Ok(OcrClient { client, base })
    }

    pub async fn get_invoice_info(&self, img: &DynamicImage) -> OcrResult<InvoiceDetails> {
        let r = self._invoice_info(img).await?;
        Ok(r.into())
    }

    pub async fn get_doc_info(&self, doc: OcrDoc<'_>) -> OcrResult<ParsedDoc> {
        self._doc_info(doc.bytes).await
    }

    async fn img_req<T>(&self, url_path: &str, img: &DynamicImage) -> OcrResult<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        // Convert DynamicImage to bytes
        let mut img_bytes = Vec::new();

        img.to_rgb8()
            .write_to(&mut Cursor::new(&mut img_bytes), image::ImageFormat::Jpeg)?;
        self.bytes_req(url_path, img_bytes).await
    }

    /// makes a request to given path
    /// the path should not include the base
    async fn bytes_req<T>(&self, url_path: &str, data: Vec<u8>) -> OcrResult<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let encoded = BASE64_STANDARD.encode(&data);
        // let part = Part::text(encoded);
        let part = Part::bytes(data).file_name("[unknown].jpg");

        let form = Form::new().part("file", part);

        let req = self.client.post(self.base.join(url_path)?).multipart(form);

        // println!("{:#?}", res.build()?.headers());
        // unimplemented!()
        let res = req.send().await?;

        // response mapping
        if res.status().as_u16() >= 200 && res.status().as_u16() < 300 {
            let res = res.json::<T>().await?;
            Ok(res)
        } else {
            Err(res.json::<OCRServerErr>().await?.into())
        }
    }

    /// makes a request to /.../ocr/invoice
    async fn _invoice_info(&self, img: &DynamicImage) -> OcrResult<InvoiceResponse> {
        let res = self.img_req::<InvoiceResponse>("ocr/invoice", img).await?;
        Ok(res)
    }

    /// makes a request to /.../ocr/doc
    async fn _doc_info(&self, bytes: Vec<u8>) -> OcrResult<ParsedDoc> {
        let res = self.bytes_req("ocr/doc", bytes).await?;
        Ok(res)
    }
}
