use doc::PdfDoc;
use pdf::prelude::*;
pub mod doc;

use crate::err::{OCRErrs, OcrResult};
use std::{env::temp_dir, path::PathBuf};

/// writes the dynamic library at the provided location
/// based on the platform and returns the path at which it
/// was written
fn write_dylib() -> OcrResult<PathBuf> {
    #[cfg(target_os = "windows")]
    const PDFIUM_LIB: &[u8] = include_bytes!("../../include/windows/pdfium.dll");

    #[cfg(target_os = "macos")]
    const PDFIUM_LIB: &[u8] = include_bytes!("../../include/macos/libpdfium.dylib");

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    const PDFIUM_LIB: &[u8] = include_bytes!("../../include/linux/x86_64/libpdfium.so");

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    const PDFIUM_LIB: &[u8] = include_bytes!("../../include/linux/aarch64/libpdfium.so");

    let temp_lib_path = temp_dir().join(Pdfium::pdfium_platform_library_name());

    // Write the embedded library to the temporary file
    if !temp_lib_path.exists() {
        std::fs::write(&temp_lib_path, PDFIUM_LIB).map_err(|e| {
            OCRErrs::Custom(format!(
                "Failed to create temp libpdfium dynamic library : {e:?}"
            ))
        })?;
    }

    Ok(temp_lib_path)
}

#[derive(Debug)]
pub struct PdfEngine {
    pdfium: Pdfium,
}

impl PdfEngine {
    pub fn new() -> OcrResult<Self> {
        let p = write_dylib()?;
        let bindings = Pdfium::bind_to_library(p)?;
        let pdfium = Pdfium::new(bindings);
        Ok(Self { pdfium })
    }

    pub fn parse<'a>(&'a self, bytes: &'a [u8]) -> OcrResult<PdfDoc<'a>> {
        let doc = self.pdfium.load_pdf_from_byte_slice(bytes, None)?;

        let pages = doc.pages().iter();

        let mut imgs = Vec::new();

        for page in pages {
            for obj in page.objects().iter() {
                if let Some(image) = obj.as_image_object() {
                    if let Ok(image) = image.get_raw_image() {
                        imgs.push(image);
                    }
                }
            }
        }

        Ok(PdfDoc { doc, imgs })
    }
}
