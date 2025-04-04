use image::DynamicImage;
use pdf::prelude::{PdfDocument, PdfSearchOptions};
use serde::Deserialize;

use crate::err::OcrResult;

#[derive(Debug, Deserialize)]
pub struct BoundingBox {
    pub t: f64,
    pub l: f64,
    pub r: f64,
    pub b: f64,
    pub coord_origin: String,
}

#[derive(Debug, Deserialize)]
pub struct Prov {
    pub page_no: usize,
    pub bbox: BoundingBox,
    pub charspan: [usize; 2],
}

#[derive(Debug, Deserialize)]
pub struct PDFText {
    pub prov: Vec<Prov>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ParsedPDF {
    pub texts: Vec<PDFText>,
}

pub struct PdfDoc<'a> {
    /// the underlying document
    pub doc: PdfDocument<'a>,
    pub imgs: Vec<DynamicImage>,
}

impl PdfDoc<'_> {
    // will search thru the document
    pub fn contains(&self, needle: &str) -> bool {
        let options = PdfSearchOptions::new();
        let pages = self.doc.pages().iter();

        for page in pages {
            for text in page.text().iter() {
                let search_items = text.search(needle, &options);
                if search_items.find_next().is_some() {
                    return true;
                }
            }
        }

        false
    }

    /// Extract relevenat data from images if possible
    /// this will send a request to the ocr server to perform ocr
    /// this will assume the images are upright
    /// if there is orientation the caller has to fix
    pub async fn extract(&self) -> OcrResult<Vec<ParsedPDF>> {
        todo!()
    }

    /// Get Invoice data
    /// this will make a request ocr server
    /// to retrieve necessary info
    /// if there multiple pages to the invoice
    /// it will once perform info fetching on the first one
    pub async fn invoice_info(&self) -> OcrResult<()> {
        todo!()
    }
}
