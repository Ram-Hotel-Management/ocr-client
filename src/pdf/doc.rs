use crate::{
    err::OcrResult,
    server::{
        OcrClient,
        docling::{OcrDoc, ParsedDoc},
        invoice::InvoiceDetails,
    },
};
use image::DynamicImage;
use pdf::prelude::*;

pub struct PdfDoc {
    pub bytes: Vec<u8>,
    // pub(crate) doc: PdfDocument<'a>,
    pub(crate) pdfium: Pdfium,
    /// All embedded images found in this document
    pub imgs: Vec<DynamicImage>,
    pub parsed_doc: Vec<OcrResult<ParsedDoc>>,
}

impl PdfDoc {
    pub fn load(&self) -> OcrResult<PdfDocument> {
        Ok(self.pdfium.load_pdf_from_byte_slice(&self.bytes, None)?)
    }

    // will search thru the document
    pub fn contains(&self, needle: &str) -> bool {
        // search thru to the ocred items first
        for parsed in self.parsed_doc.iter().flatten() {
            if parsed.contains_insensitive(needle) {
                return true;
            }
        }

        let doc = match self.load() {
            Ok(doc) => doc,
            Err(_) => return false,
        };

        let options = PdfSearchOptions::new();
        let pages = doc.pages().iter();

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

    async fn ocr_img(client: &OcrClient, img: &DynamicImage) -> OcrResult<ParsedDoc> {
        let doc = OcrDoc::from_img(img)?;
        client.docling(doc).await
    }

    /// Extract relevenat data from images if possible
    ///  - this will send a request to the ocr server to perform extraction.
    ///  - this will assume the images are upright
    ///  - if there is orientation, the caller has to fix.
    ///  - if an error occurs while sending the request the value at that index is set to None
    ///  - Only performs OCR on the images that are incompleted/ or have not been done
    pub async fn ocr(&mut self, client: &OcrClient) {
        for i in self.parsed_doc.len()..self.imgs.len() {
            let img = &self.imgs[i];
            let parsed = Self::ocr_img(client, img).await;
            self.parsed_doc.push(parsed);
        }
    }

    /// Get Invoice data
    /// this will make a request ocr server
    /// to retrieve necessary info
    /// if there multiple pages to the invoice
    /// it will fetch info on the first page only
    /// as most of the invoice contain needed info on the first page
    /// in order to convert page image it needs to render into bitmap
    /// and to convert it to pixels for image it needs height and width
    pub(crate) async fn invoice_info(&self, client: &OcrClient) -> OcrResult<InvoiceDetails> {
        self.invoice_info_wh(client).await
    }

    /// gets invoice data will from this pdf file
    /// only the first page is sent for processing
    /// takes height and width at which the image will be generated
    /// consider using `Self::invoice_info` which renders the at @200dpi
    /// Beware that this does not perform any rotation on an the page
    pub(crate) async fn invoice_info_wh(&self, client: &OcrClient) -> OcrResult<InvoiceDetails> {
        let doc = self.load()?;
        let first_page = doc.pages().first()?;
        println!(
            "{}, {}",
            first_page.height().to_mm(),
            first_page.width().to_mm()
        );

        let render_config = PdfRenderConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000);

        let first_page_img = first_page.render_with_config(&render_config)?.as_image();

        client.invoice(&first_page_img).await
    }

    pub async fn into_invoice_doc(self, client: &OcrClient) -> PdfInvoiceDoc {
        let invoice_details = self.invoice_info(client).await;

        PdfInvoiceDoc {
            doc: self,
            invoice_details,
        }
    }
}

/// Pdf document that is possibly an invoice
pub struct PdfInvoiceDoc {
    pub doc: PdfDoc,
    pub invoice_details: OcrResult<InvoiceDetails>,
}
