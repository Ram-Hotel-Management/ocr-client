use image::ImageError;
pub type OcrResult<T> = Result<T, OCRErrs>;

#[derive(Debug, thiserror::Error)]
pub enum OCRErrs {
    #[error("An error occurred while parsing the PDF text (PDF_Extract)")]
    ExtractPdf(#[from] pdf::prelude::PdfiumError),

    #[error("Error Parsing URL")]
    URL(#[from] url::ParseError),

    #[error("Error while making requsting")]
    Req(#[from] reqwest::Error),

    #[error["An error occurred while parsing the data"]]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Custom(String),

    #[error("An Image Error Occurred")]
    Image(#[from] ImageError),

    #[error("IO Err: {0}")]
    IO(#[from] std::io::Error),

    #[error("EasyOCR not initialized")]
    EasyOcrInit,
}
