use crate::err::OcrResult;

use std::net::SocketAddr;

#[allow(async_fn_in_trait)]
pub trait DoclingOCR {
    /// Extracts data from the server
    async fn extract(&self, addr: SocketAddr) -> OcrResult<()>;
}
