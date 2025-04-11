use crate::{
    err::{OcrErrs, OcrResult},
    server::{
        docling::{OcrDoc, ParsedDoc},
        invoice::InvoiceDetails,
    },
};
use image::DynamicImage;
use pdf::prelude::*;

pub struct PdfDoc<'a> {
    /// the underlying document
    pub doc: PdfDocument<'a>,
    /// All embedded images found in this document
    pub imgs: Vec<DynamicImage>,
    pub parsed_doc: Vec<ParsedDoc>,
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

    /// Extract OCR data on the index img
    pub async fn extract_i(&self, i: usize) -> OcrResult<ParsedDoc> {
        if i >= self.imgs.len() {
            return OcrErrs::Custom("Tried to access image greater than the length".into()).into();
        }

        let img = &self.imgs[i];

        let img_doc = OcrDoc::from_img(img)?;

        img_doc.parse().await
    }

    /// Extract relevenat data from images if possible
    /// this will send a request to the ocr server to perform extraction
    /// this will assume the images are upright
    /// if there is orientation, the caller has to fix.
    /// Will return an error as soon as one of the images return an error
    pub async fn extract(&mut self) -> OcrResult<()> {
        for i in self.parsed_doc.len()..self.imgs.len() {
            let parsed = self.extract_i(i).await?;
            self.parsed_doc.push(parsed);
        }

        Ok(())
    }

    /// Get Invoice data
    /// this will make a request ocr server
    /// to retrieve necessary info
    /// if there multiple pages to the invoice
    /// it will fetch info on the first page only
    /// as most of the invoice contain needed info on the first page
    /// in order to convert page image it needs to render into bitmap
    /// and to convert it to pixels for image it needs height and width
    /// By default it will use A4 size paper in pixel @200DPI (w: 1654, h: 2339)
    pub async fn invoice_info(&self) -> OcrResult<InvoiceDetails> {
        self.invoice_info_wh(2339, 1654).await
    }

    /// gets invoice data will from this pdf file
    /// only the first page is sent for processing
    /// takes height and width at which the image will be generated
    /// consider using `Self::invoice_info` which renders the at @200dpi
    /// Beware that this does not perform any rotation on an the page
    pub async fn invoice_info_wh(
        &self,
        height: Pixels,
        width: Pixels,
    ) -> OcrResult<InvoiceDetails> {
        let first_page = self.doc.pages().first()?;

        let first_page_img = first_page.render(width, height, None)?.as_image();

        InvoiceDetails::process(&first_page_img).await
    }
}
