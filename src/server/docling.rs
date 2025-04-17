use std::io::Cursor;

use image::DynamicImage;
use regex::RegexBuilder;
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
pub struct OcrText {
    pub prov: Vec<Prov>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ParsedDoc {
    pub texts: Vec<OcrText>,
}

impl ParsedDoc {
    // pub fn from_img(img: &DynamicImage) -> OcrResult<Self> {
    //     // super::OcrClient::
    // }

    /// performs a search for text in the parsed image
    /// text.
    /// NOTE: the search is case sensitive
    pub fn contains(&self, needle: &str) -> bool {
        for OcrText { prov: _, text } in &self.texts {
            if text.contains(needle) {
                return true;
            }
        }

        false
    }

    /// perform a case insenitive search
    pub fn contains_insensitive(&self, needle: &str) -> bool {
        let re = RegexBuilder::new(needle)
            .case_insensitive(true)
            .build()
            .unwrap();

        for OcrText { prov: _, text } in &self.texts {
            if re.is_match(text) {
                return true;
            }
        }

        false
    }
}

/// Since Docling can perform ocr
/// on various file formats
/// This enum be used to wrap around files/ bytes
/// to limit so that only certain type of files can be sent
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocType {
    Word,
    Pdf,
    /// Images will be first converted to
    /// Png
    Img,
}

/// Represents the OCR Document
/// This is the document that will be sent to the server
#[derive(Debug, Clone)]
pub struct OcrDoc<'a> {
    /// Type of the document
    pub ty: DocType,
    /// name of the file
    pub name: &'a str,
    /// bytes
    pub bytes: Vec<u8>,
}

impl<'a> OcrDoc<'a> {
    pub fn new(name: &'a str, bytes: Vec<u8>) -> Option<Self> {
        let ty = if name.ends_with(".doc") || name.ends_with(".docx") {
            DocType::Word
        } else if name.ends_with(".pdf") {
            DocType::Pdf
        } else if name.ends_with(".jpg")
            || name.ends_with(".jpeg")
            || name.ends_with(".tif")
            || name.ends_with(".tiff")
            || name.ends_with(".png")
        {
            DocType::Img
        } else {
            return None;
        };

        Some(Self { ty, name, bytes })
    }

    pub fn from_img(img: &DynamicImage) -> OcrResult<Self> {
        // Convert DynamicImage to bytes
        let mut img_bytes = Vec::new();

        img.write_to(&mut Cursor::new(&mut img_bytes), image::ImageFormat::Png)?;

        Ok(Self {
            ty: DocType::Img,
            name: "[Unknown].png",
            bytes: img_bytes,
        })
    }

    /// Perform ocr on the provided document
    /// this will send the request to the server
    pub async fn parse(self) -> OcrResult<ParsedDoc> {
        super::OcrClient::get_doc_info(self).await
    }
}

#[test]
fn a() {
    let p = "Some random Ass text goes hehre";

    dbg!(p.contains("ass"));

    let re = RegexBuilder::new("some")
        .case_insensitive(true)
        .build()
        .unwrap();

    dbg!(re.is_match(p));
}
