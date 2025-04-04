/// This module is dedicated
/// for making requests to server
/// whcih is written in python is an independent
/// codebase
pub mod docling;

use std::io::Cursor;

use image::DynamicImage;
use reqwest::{
    Client, Url,
    multipart::{Form, Part},
};
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::OnceCell;

use crate::err::{OCRErrs, OcrResult};

pub(crate) static OCR_CLIENT: OnceCell<OcrClient> = OnceCell::const_new();

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
    /// makes a request to /.../ocr/invoice
    pub async fn invoice_info(&self, img: &DynamicImage) -> OcrResult<()> {
        // Convert DynamicImage to bytes
        let mut img_bytes = Vec::new();

        img.write_to(&mut Cursor::new(&mut img_bytes), image::ImageFormat::Jpeg)?;

        let part = Part::bytes(img_bytes).file_name("[unknown].jpg");
        let form = Form::new().part("file", part);

        let res = self
            .client
            .post(self.base.join("ocr/invoice")?)
            .multipart(form)
            .send()
            .await?;

        let r = res.json::<Value>().await?;

        println!("{r:#?}");

        Ok(())
    }
}

/// Initializes ocr client
/// this will also test if the server is live
/// # returns:
/// - true: if client was initialized
/// - false: either client initialization failed
pub fn init_client(addr: &str) -> OcrResult<()> {
    let base = Url::parse(addr)?;

    let client = Client::builder().use_rustls_tls().build().unwrap();
    let a = OcrClient { client, base };
    let res = OCR_CLIENT.set(a);

    if let Err(e) = res {
        return Err(OCRErrs::Custom(format!(
            "Error initializing OcrClient. Has the client initialized: {}",
            e.is_already_init_err()
        )));
    }

    Ok(())
}

#[tokio::test]
async fn invoice_info() {
    use std::fs::File;
    init_client("http://localhost:8000").unwrap();

    println!(
        "{}",
        File::open("./1.jpg").unwrap().metadata().unwrap().len()
    );

    let f = File::open("./1.jpg").unwrap();

    let img = image::open("./1.jpg").unwrap();
    OCR_CLIENT.get().unwrap().invoice_info(&img).await.unwrap();
}
