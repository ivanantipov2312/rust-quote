use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuoteResponse {
    pub quote: String,
    pub author: String,
    pub work: String,
    pub categories: Vec<String>,
}
