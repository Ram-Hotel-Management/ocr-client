use image::ImageError;
use serde::Deserialize;
use serde_json::Value;
pub type OcrResult<T> = Result<T, OcrErrs>;

#[derive(Debug, thiserror::Error)]
pub enum OcrErrs {
    #[error("An error occurred while parsing the PDF text (PDF_Extract)")]
    ExtractPdf(
        #[source]
        #[from]
        pdf::prelude::PdfiumError,
    ),

    #[error("Error Parsing URL")]
    URL(
        #[source]
        #[from]
        url::ParseError,
    ),

    #[error("Error while making requsting")]
    Req(
        #[source]
        #[from]
        reqwest::Error,
    ),

    #[error["An error occurred while parsing the data"]]
    Json(
        #[source]
        #[from]
        serde_json::Error,
    ),

    #[error("An Image Error Occurred")]
    Image(
        #[source]
        #[from]
        ImageError,
    ),

    #[error("IO Err: {0}")]
    IO(
        #[source]
        #[from]
        std::io::Error,
    ),

    #[error("An error occurred on the server")]
    Server(
        #[source]
        #[from]
        OCRServerErr,
    ),
}

impl<T> From<OcrErrs> for OcrResult<T> {
    fn from(value: OcrErrs) -> Self {
        Self::Err(value)
    }
}

#[derive(Debug, Deserialize, thiserror::Error)]
pub struct OCRServerErr {
    pub details: Option<Value>,
    pub error: Option<String>,
}

impl std::fmt::Display for OCRServerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(err) = &self.error {
            write!(f, "Server Response: {}", err)
        } else if let Some(inner) = &self.details {
            inner.fmt(f)
        } else {
            write!(f, "An error occurred on the server side")
        }
    }
}
