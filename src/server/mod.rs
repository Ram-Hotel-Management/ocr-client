/// This module is dedicated
/// for making requests to server
/// whcih is written in python is an independent
/// codebase
pub mod docling;
pub mod invoice;
use crate::{OCRServerErr, err::OcrResult};
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
    pub fn new<S: AsRef<str>>(addr: S) -> OcrResult<Self> {
        let base = Url::parse(addr.as_ref())?;
        let client = Client::builder().use_rustls_tls().build()?;
        Ok(OcrClient { client, base })
    }

    /// makes a request to /.../ocr/invoice
    pub async fn invoice(&self, img: &DynamicImage) -> OcrResult<InvoiceDetails> {
        let res = self.img_req::<InvoiceResponse>("ocr/invoice", img).await?;
        Ok(res.into())
    }

    /// makes a request to /.../ocr/doc
    pub async fn docling(&self, doc: OcrDoc<'_>) -> OcrResult<ParsedDoc> {
        let res = self
            .bytes_req("ocr/doc", doc.bytes, doc.name.into())
            .await?;
        Ok(res)
    }

    async fn img_req<T>(&self, url_path: &str, img: &DynamicImage) -> OcrResult<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        // Convert DynamicImage to bytes
        let mut img_bytes = Vec::new();

        img.write_to(&mut Cursor::new(&mut img_bytes), image::ImageFormat::Png)?;
        self.bytes_req(url_path, img_bytes, "[Unknown].png".into())
            .await
    }

    /// makes a request to given path
    /// the path should not include the base
    async fn bytes_req<T>(&self, url_path: &str, data: Vec<u8>, name: String) -> OcrResult<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        // NOTE: filename has to be attached otherwise it causes
        // issue on the server side
        let part = Part::bytes(data).file_name(name);

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

    // /// makes a request to /.../ocr/invoice
    // async fn _invoice_info(&self, img: &DynamicImage) -> OcrResult<InvoiceResponse> {
    //     let res = self.img_req::<InvoiceResponse>("ocr/invoice", img).await?;
    //     Ok(res)
    // }
}
