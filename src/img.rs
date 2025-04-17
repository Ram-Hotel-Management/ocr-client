use image::DynamicImage;

use crate::{
    err::OcrResult,
    server::docling::{OcrDoc, ParsedDoc},
};

/// Helper trait to perform ocr on an image
pub trait ImgOcr {
    #[allow(async_fn_in_trait)]
    /// performs ocr on a Dynamic Image
    async fn ocr(&self) -> OcrResult<ParsedDoc>;
}

impl ImgOcr for DynamicImage {
    async fn ocr(&self) -> OcrResult<ParsedDoc> {
        OcrDoc::from_img(self)?.parse().await
    }
}
