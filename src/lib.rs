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

pub struct OcrEngine {
    pdf_engine: PdfEngine,
}

impl OcrEngine {
    pub async fn new(ocr_server_addr: &str) -> OcrResult<Self> {
        setup(ocr_server_addr).await?;
        Ok(Self {
            pdf_engine: PdfEngine::new().await?,
        })
    }

    /// short hand for getting invoice and pdf in one shot
    pub async fn pdf_invoice(&self, bytes: Vec<u8>) -> OcrResult<PdfInvoiceDoc> {
        self.pdf_engine.invoice(bytes).await
    }

    /// creates a pdf
    /// from provided bytes
    /// It is assumed that provided bytes are from PDF, otherwise it will return
    /// an error
    pub async fn pdf(&self, bytes: Vec<u8>) -> OcrResult<PdfDoc> {
        self.pdf_engine.doc(bytes).await
    }

    /// short hand to process invoice details quickly
    pub async fn invoice_details(img: &DynamicImage) -> OcrResult<InvoiceDetails> {
        InvoiceDetails::process(img).await
    }

    /// Short hand function for easy access
    pub async fn ocr(img: &DynamicImage) -> OcrResult<ParsedDoc> {
        OcrDoc::from_img(img)?.parse().await
    }
}
