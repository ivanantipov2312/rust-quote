use serde::{Deserialize, Serialize};

use crate::quote::Quote;

// Quick struct for Serializing and Deserializing reqwest responses
// TODO: Make use of Serde and come up with a way to parse directly to Quote type
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

        // Work field must be present, so no work = empty string
        let work = if let Some(work) = value.work {
            work
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
