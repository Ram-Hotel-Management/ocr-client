use doc::{PdfDoc, PdfInvoiceDoc};
use pdf::prelude::*;
pub mod doc;
use crate::{err::OcrResult, server::OcrClient};

// use std::{
//     env::temp_dir,
//     path::PathBuf,
//     sync::atomic::{AtomicBool, Ordering},
// };

// use tokio::fs::metadata;

// static DYLIB_WRITTEN: AtomicBool = AtomicBool::new(false);

// /// writes the dynamic library at the provided location
// /// based on the platform and returns the path at which it
// /// was written
// async fn write_dylib() -> OcrResult<PathBuf> {
//     #[cfg(target_os = "windows")]
//     const PDFIUM_LIB: &[u8] = include_bytes!("../../include/windows/pdfium.dll");

//     #[cfg(target_os = "macos")]
//     const PDFIUM_LIB: &[u8] = include_bytes!("../../include/macos/libpdfium.dylib");

//     #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
//     const PDFIUM_LIB: &[u8] = include_bytes!("../../include/linux/x86_64/libpdfium.so");

//     #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
//     const PDFIUM_LIB: &[u8] = include_bytes!("../../include/linux/aarch64/libpdfium.so");

//     let temp_lib_path = temp_dir();
//     // let temp_lib_path = temp_lib_path.join(Pdfium::pdfium_platform_library_name());

//     {
//         let exists = metadata(&temp_lib_path);
//         // Write the embedded library to the temporary file
//         // if it doesn't exists
//         if !DYLIB_WRITTEN.load(Ordering::Acquire) || exists.await.is_err() {
//             std::fs::write(&temp_lib_path, PDFIUM_LIB).map_err(|e| {
//                 OcrErrs::Custom(format!(
//                     "Failed to create temp libpdfium dynamic library : {e:?}"
//                 ))
//             })?;

//             DYLIB_WRITTEN.store(true, Ordering::Release);
//         }
//     }

//     Ok(temp_lib_path)
// }

async fn load_lib() -> OcrResult<Pdfium> {
    // let p = write_dylib().await?;
    // let bindings = Pdfium::bind_to_library(p)?;
    let bindings = Pdfium::bind_to_statically_linked_library()?;
    let pdfium = Pdfium::new(bindings);
    Ok(pdfium)
}

// #[derive(Debug)]
pub struct PdfEngine {
    // pdfium: Pdfium,
}

impl PdfEngine {
    pub async fn new() -> OcrResult<Self> {
        load_lib().await?;
        Ok(Self {
            // pdfium: load_lib().await?,
        })
    }

    pub async fn doc(&self, bytes: Vec<u8>) -> OcrResult<PdfDoc> {
        let mut imgs = Vec::new();
        let pdfium = load_lib().await?;
        {
            let doc = pdfium.load_pdf_from_byte_slice(&bytes, None)?;

            let pages = doc.pages().iter();

            for page in pages {
                for obj in page.objects().iter() {
                    if let Some(image) = obj.as_image_object() {
                        if let Ok(image) = image.get_raw_image() {
                            imgs.push(image);
                        }
                    }
                }
            }
        }

        let pdf = PdfDoc {
            pdfium,
            bytes,
            imgs,
            parsed_doc: Vec::new(),
        };
        Ok(pdf)
    }

    pub async fn invoice(&self, client: &OcrClient, bytes: Vec<u8>) -> OcrResult<PdfInvoiceDoc> {
        Ok(self.doc(bytes).await?.into_invoice_doc(client).await)
    }
}
