use err::OcrResult;
use server::OcrClient;
pub mod err;
pub mod img;
pub mod pdf;
pub mod server;

/// this function needs to be called to initialize important
/// parts of the ocr crate. Either call this function or
/// the individual items needs be initialized
/// # Arguments
/// - ocr_addr: The address where the ocr server is running, could be website or ip address with port
pub async fn setup(ocr_addr: &str) -> OcrResult<()> {
    OcrClient::init(ocr_addr).await?;
    Ok(())
}
