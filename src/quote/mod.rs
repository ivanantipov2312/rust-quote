pub mod options;
pub mod category;
pub mod response;
pub mod get;

use std::str::FromStr;

use category::Category;
use response::QuoteResponse;

// Actual quote for storage and type checking reason
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
    
        write!(f, "Categories: ")?;
        for c in &self.categories {
            write!(f, "{} ", c.to_string())?;
        }

        write!(f, "\n")
    }
}

impl From<QuoteResponse> for Quote {
    fn from(value: QuoteResponse) -> Self {
        // Convert from String categories to Category
        let categories: Vec<_> = value.categories
            .iter()
            .map(|c| Category::from_str(c.as_str()))
            .filter_map(Result::ok) // filter out everything that is not ok
            .collect();

        // Work field is always present in the API, so if it's empty set None
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
