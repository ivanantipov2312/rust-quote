use crate::{QuoteSearchOptions, QuoteResponse, Quote};
use reqwest::Client;

const BASE_URL: &'static str = "https://api.api-ninjas.com/v2/randomquotes";
const QOTD_URL: &'static str = "https://api.api-ninjas.com/v2/quoteoftheday";

pub fn build_url(options: &QuoteSearchOptions) -> String {
    let mut url = format!("{BASE_URL}?");
    if let Some(categories) = &options.categories {
        url.push_str("categories=");
        for c in categories {
            let s = c.to_string();
            url.push_str(&format!("{s}%2C")); // %2C = ;
        }
        url.pop();
        url.pop();
        url.pop();
        url.push('&');
    }

    if let Some(excluded) = &options.exclude_categories {
        url.push_str("exclude_categories=");
        for c in excluded {
            let s = c.to_string();
            url.push_str(&format!("{s}%2C")); // %2C = ;
        }
        url.pop();
        url.pop();
        url.pop();
        url.push('&');
    }

    if let Some(author) = &options.author {
        let author = author.replace(" ", "%20"); // space = %20, multiple words might be present
        url.push_str(&format!("author={author}&"));
    }

    if let Some(work) = &options.work {
        let work = work.replace(" ", "%20"); // space = %20, multiple words might be present
        url.push_str(&format!("work={work}&"));
    }

    url.pop(); // Remove trailing &
    url
}

// API always returns exactly one quote, but for some reason it still returns an array
pub async fn get_quote(client: &Client, options: &QuoteSearchOptions) -> Result<Quote, String> {
    let url = build_url(options);

    let resp: Vec<QuoteResponse> = client.get(url)
        .send()
        .await
        .map_err(|_| "Failed to send".to_string())?
        .json()
        .await
        .map_err(|_| "Failed to get response".to_string())?;

    let first = resp.into_iter()
        .next()
        .ok_or_else(|| "No quotes found!".to_string())?;
    Ok(Quote::from(first))
}

// API for some reason returns an array, even though it must be exactly one quote
pub async fn get_quote_of_the_day(client: &Client) -> Result<Quote, String> {
    let resp: Vec<QuoteResponse> = client.get(QOTD_URL)
        .send()
        .await
        .map_err(|_| "Failed to send".to_string())?
        .json()
        .await
        .map_err(|_| "Failed to get response".to_string())?;

    let first = resp.into_iter().next().ok_or_else(|| "No quotes found!".to_string())?;
    Ok(Quote::from(first))
}
