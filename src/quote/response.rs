use serde::{Deserialize, Serialize};

use crate::quote::Quote;

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteResponse {
    pub quote: String,
    pub author: String,
    pub work: String,
    pub categories: Vec<String>,
}

impl From<Quote> for QuoteResponse {
    fn from(value: Quote) -> Self {
        let categories: Vec<String> = value.categories.into_iter().map(|q| q.to_string()).collect();
        let work = if let Some(work) = value.work {
            work.clone()
        } else {
            "".to_string()
        };

        Self {
            quote: value.quote,
            author: value.author,
            work,
            categories,
        }
    }
}
