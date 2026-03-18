pub mod options;
pub mod category;
pub mod response;
pub mod get;

use std::str::FromStr;

use category::Category;
use response::QuoteResponse;

pub struct Quote {
    quote: String,
    author: String,
    work: Option<String>,
    categories: Vec<Category>,
}

impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quote: {}\n", self.quote)?;
        write!(f, "By: {}\n", self.author)?;
        if let Some(work) = &self.work {
            write!(f, "From work: {}\n", work)?;
        }

        for c in &self.categories {
            write!(f, "Category: {}\n", c.to_string())?;
        }

        write!(f, "")
    }
}

impl From<QuoteResponse> for Quote {
    fn from(value: QuoteResponse) -> Self {
        let categories: Vec<_> = value.categories
            .iter()
            .map(|c| Category::from_str(c.as_str()))
            .filter_map(Result::ok)
            .collect();

        let work = if !value.work.is_empty() {
            Some(value.work)
        } else {
            None
        };

        Self {
            quote: value.quote,
            author: value.author,
            work,
            categories,
        }
    }
}
