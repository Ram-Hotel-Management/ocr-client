/// This module is dedicated
/// for making requests to server
/// whcih is written in python is an independent
/// codebase
pub mod docling;
pub mod invoice;
use crate::err::OcrResult;
use docling::{OcrDoc, ParsedDoc};
use image::DynamicImage;
use invoice::{InvoiceDetails, InvoiceResponse};
use reqwest::{
    Client, Url,
    multipart::{Form, Part},
};
use serde::Deserialize;
use serde_json::Value;
use std::io::Cursor;

#[derive(Debug, Deserialize)]
pub struct OCRServerHttpErrInner {
    pub msg: Value,
    pub r#type: String,
    pub ctx: Value,
    pub input: Value,
    pub loc: Value,
}

#[derive(Debug, Deserialize)]
pub struct OCRServerHttpErr {
    pub details: Vec<OCRServerHttpErrInner>,
}

pub struct OcrClient {
    pub client: Client,
    pub base: Url,
}

impl OcrClient {
    /// Initialize ocr client
    pub async fn new(addr: &str) -> OcrResult<Self> {
        let base = Url::parse(addr)?;
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

        img.write_to(&mut Cursor::new(&mut img_bytes), image::ImageFormat::Jpeg)?;
        self.bytes_req(url_path, img_bytes).await
    }

    /// makes a request to given path
    /// the path should not include the base
    async fn bytes_req<T>(&self, url_path: &str, data: Vec<u8>) -> OcrResult<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let part = Part::bytes(data);

        let form = Form::new().part("file", part);

        let res = self
            .client
            .post(self.base.join(url_path)?)
            .multipart(form)
            // .json(&body)
            .send()
            .await?;

        let res = res.json::<T>().await?;

        Ok(res)
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

// #[tokio::test]
// async fn test_invoice_info() {
//     // use std::fs::File;
//     init_ocr_client("http://localhost:8000").unwrap();

//     let img = image::open("./1.jpg").unwrap();
//     let r = OCR_CLIENT
//         .read()
//         .await
//         .as_ref()
//         .unwrap()
//         .invoice_info(&img)
//         .await
//         .unwrap();
//     println!("{r:#?}");
// }

// #[tokio::test]
// async fn test_doc() {
//     // use std::fs::File;
//     init_client("http://localhost:8000").unwrap();

//     let img = image::open("./1.jpg").unwrap();
//     let r = OCR_CLIENT.get().unwrap().invoice_info(&img).await.unwrap();
//     println!("{r:#?}");
// }
