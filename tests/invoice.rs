use ocr_client::OcrEngine;
use tokio::io::AsyncReadExt;

#[tokio::test]
async fn invoice_test() {
    let target = std::env::var("CARGO_CFG_TARGET_OS").expect("TARGET was not set");
    println!("cargo:warning=Building for target: {}", target);

    let mut file = tokio::fs::File::open("./tests/golden_waffles.pdf")
        .await
        .unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).await.unwrap();

    let ocr_engine = OcrEngine::new("localhost:3000").await.unwrap();
    let res = ocr_engine.pdf(bytes).await;
    assert!(res.is_ok())
}
