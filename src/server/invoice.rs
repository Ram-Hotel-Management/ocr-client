use chrono::NaiveDate;
use either::Either;
use image::DynamicImage;
use regex::Regex;
use serde::Deserialize;

use crate::err::OcrResult;

/// Provides a convience method to deserialize
/// invoice info received from the server
#[derive(Debug, Deserialize)]
pub struct InvoiceResponse {
    pub invoice_no: Option<String>,
    pub vendor: Option<String>,
    pub acct_no: Option<String>,
    pub inv_date: Option<String>,
    pub due_date: Option<String>,
    pub total: Option<String>,
}

#[derive(Debug)]
pub struct InvoiceDetails {
    pub inv_no: Option<String>,
    pub vendor: Option<String>,
    pub acct_no: Option<String>,
    pub inv_date: Either<NaiveDate, String>,
    pub due_date: Either<NaiveDate, String>,
    pub total: Either<f64, String>,
}

impl From<InvoiceResponse> for InvoiceDetails {
    fn from(value: InvoiceResponse) -> Self {
        let InvoiceResponse {
            invoice_no,
            vendor,
            acct_no,
            inv_date,
            due_date,
            total,
        } = value;

        let inv_date = inv_date
            .map(parse_date_from_str)
            .unwrap_or_else(|| Either::Right("No Date Available".into()));

        let due_date = due_date
            .map(parse_date_from_str)
            .unwrap_or_else(|| Either::Right("No Date Available".into()));

        let total = total
            .map(parse_f64_from_str)
            .unwrap_or_else(|| Either::Right("Unavailable".into()));

        Self {
            inv_no: invoice_no,
            vendor,
            acct_no,
            inv_date,
            due_date,
            total,
        }
    }
}

impl InvoiceDetails {
    pub async fn process(img: &DynamicImage) -> OcrResult<Self> {
        super::OcrClient::get_invoice_info(img).await
    }
}

/// Parses the date from a given string
/// if unable to get date
/// returns the string
pub fn parse_date_from_str(s: String) -> Either<NaiveDate, String> {
    let words = s.split_ascii_whitespace();

    for word in words {
        if let Ok((date, _remainder)) = NaiveDate::parse_and_remainder(word, "%m/%d/%Y") {
            return Either::Left(date);
        }
    }

    Either::Right(s)
}

/// Parse the total amount due
pub fn parse_f64_from_str(s: String) -> Either<f64, String> {
    let regex = Regex::new(r"[^0-9.]").unwrap();
    let binding = regex.replace_all(&s, "");
    let trimmed = binding.trim();

    let res = trimmed.parse();

    match res {
        Ok(res) => Either::Left(res),
        Err(_) => Either::Right(s),
    }
}

#[test]
fn t() {
    let a = parse_f64_from_str("**Total Amount:** $28,496.68".into());
    dbg!(a);

    let b = parse_date_from_str("**Due Date:** Not explicitly stated, but based on the payment terms \"Net 30,\" the due date is approximately 02/05/2024.".into());

    dbg!(b);
}
