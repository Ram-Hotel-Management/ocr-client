use err::OcrResult;
use server::OcrClient;

pub mod err;
pub mod pdf;
pub mod server;

/// this function needs to be called to initialize important
/// parts of the ocr crate. Either call this function or
/// the individual items needs be initialized
pub async fn setup(ocr_addr: &str) -> OcrResult<()> {
    OcrClient::init(ocr_addr).await?;

    Ok(())
}
