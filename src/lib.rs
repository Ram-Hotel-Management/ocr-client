use image::DynamicImage;
use pdf::{
    PdfEngine,
    doc::{PdfDoc, PdfInvoiceDoc},
};
use server::{
    OcrClient,
    docling::{OcrDoc, ParsedDoc},
    invoice::InvoiceDetails,
};
mod err;
pub mod pdf;
pub mod server;
pub use err::*;

pub struct OcrEngine {
    pdf_engine: PdfEngine,
    client: OcrClient,
}

impl OcrEngine {
    /// prepends the addr with Http or Https is the address
    /// is on the current computer or remote computer.
    /// if device is on the LAN network, it is caller's reposiblity to
    /// prefix the ip address with 'http://' otherwise this function will
    /// add 'https://' by default
    pub fn new(ocr_server_addr: &str) -> OcrResult<Self> {
        let addr = if ocr_server_addr.starts_with("localhost")
            || ocr_server_addr.starts_with("127.0.0.1")
        {
            format!("http://{ocr_server_addr}")
        } else if !ocr_server_addr.starts_with("https") && !ocr_server_addr.starts_with("http") {
            format!("https://{ocr_server_addr}")
        } else {
            ocr_server_addr.to_owned()
        };

        Ok(Self {
            pdf_engine: PdfEngine::new(),
            client: OcrClient::new(addr)?,
        })
    }

    /// short hand for getting invoice and pdf in one shot
    pub async fn pdf_invoice(&self, bytes: Vec<u8>) -> OcrResult<PdfInvoiceDoc> {
        self.pdf_engine.invoice(&self.client, bytes).await
    }

    /// creates a pdf
    /// from provided bytes
    /// It is assumed that provided bytes are from PDF, otherwise it will return
    /// an error
    pub fn pdf(&self, bytes: Vec<u8>) -> OcrResult<PdfDoc> {
        self.pdf_engine.doc(bytes)
    }

    /// short hand to process invoice details quickly
    pub async fn invoice_details(&self, img: &DynamicImage) -> OcrResult<InvoiceDetails> {
        self.client.invoice(img).await
    }

    /// Short hand function for easy access
    pub async fn ocr(&self, img: &DynamicImage) -> OcrResult<ParsedDoc> {
        let doc = OcrDoc::from_img(img)?;
        self.client.docling(doc).await
    }
}
