use err::OcrResult;
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
pub mod err;
pub mod pdf;
pub mod server;

pub struct OcrEngine {
    pdf_engine: PdfEngine,
    client: OcrClient,
}

impl OcrEngine {
    pub async fn new(ocr_server_addr: &str) -> OcrResult<Self> {
        Ok(Self {
            pdf_engine: PdfEngine::new().await?,
            client: OcrClient::new(ocr_server_addr).await?,
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
    pub async fn pdf(&self, bytes: Vec<u8>) -> OcrResult<PdfDoc> {
        self.pdf_engine.doc(bytes).await
    }

    /// short hand to process invoice details quickly
    pub async fn invoice_details(&self, img: &DynamicImage) -> OcrResult<InvoiceDetails> {
        self.client.get_invoice_info(img).await
    }

    /// Short hand function for easy access
    pub async fn ocr(&self, img: &DynamicImage) -> OcrResult<ParsedDoc> {
        let doc = OcrDoc::from_img(img)?;
        self.client.get_doc_info(doc).await
    }
}
