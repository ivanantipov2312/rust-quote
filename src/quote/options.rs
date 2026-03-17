use crate::quote::category::Category;

const BASE_URL: &'static str = "https://api.api-ninjas.com/v2/quotes";

pub struct QuoteSearchOptions {
    pub categories: Option<Vec<Category>>,
    pub exclude_categories: Option<Vec<Category>>,
    pub author: Option<String>,
    pub work: Option<String>
}

impl QuoteSearchOptions {
    pub fn build_url(&self) -> String {
        let mut url = format!("{BASE_URL}?");
        if let Some(categories) = &self.categories {
            url.push_str("categories=");
            for c in categories {
                let s = c.to_string();
                url.push_str(&format!("{s}%2C"));
            }
            url.pop();
            url.pop();
            url.pop();
            url.push('&');
        }

        if let Some(excluded) = &self.exclude_categories {
            url.push_str("exclude_categories=");
            for c in excluded {
                let s = c.to_string();
                url.push_str(&format!("{s}%2C"));
            }
            url.pop();
            url.pop();
            url.pop();
            url.push('&');
        }

        if let Some(mut author) = self.author.clone() {
            author = author.replace(" ", "%20");
            url.push_str(&format!("author={author}&"));
        }

        if let Some(mut work) = self.work.clone() {
            work = work.replace(" ", "%20");
            url.push_str(&format!("work={work}&"));
        }

        url.pop();
        url
    }
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
