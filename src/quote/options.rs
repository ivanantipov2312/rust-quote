use crate::quote::category::Category;

#[derive(Clone)]
pub struct QuoteSearchOptions {
    pub categories: Option<Vec<Category>>,
    pub exclude_categories: Option<Vec<Category>>,
    pub author: Option<String>,
    pub work: Option<String>,
}

impl std::default::Default for QuoteSearchOptions {
    fn default() -> Self {
        Self {
            categories: None,
            exclude_categories: None,
            author: None,
            work: None,
        }
    }
}
