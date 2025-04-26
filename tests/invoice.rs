use std::error::Error;

use ocr_client::OcrEngine;
use tokio::io::AsyncReadExt;

#[tokio::test]
async fn invoice_test() {
    let mut file = tokio::fs::File::open("./tests/golden_waffles.pdf")
        .await
        .unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).await.unwrap();

    let ocr_engine = OcrEngine::new("http://localhost:8000").unwrap();
    let res = ocr_engine.pdf_invoice(bytes).await.unwrap().invoice_details;
    match res {
        Ok(res) => {
            dbg!(res);
        }
        Err(err) => {
            dbg!(err.source().unwrap().to_string());
        }
    };
}

#[tokio::test]
async fn test_invoice_info() {
    let ocr_engine = OcrEngine::new("http://localhost:8000").unwrap();
    let img = image::open("./tests/1.jpg").unwrap();
    let res = ocr_engine.invoice_details(&img).await.unwrap();
    dbg!(res);
}
