mod quote;
mod io;

use std::{env, str::FromStr};
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use dotenv;

use quote::{Quote, options::QuoteSearchOptions, response::QuoteResponse, category::Category, get::{get_quote, get_quote_of_the_day}};
use io::{get_option, print_menu, write_quote_to_json};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found!");
    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", HeaderValue::from_str(&api_key).unwrap());
    let client = Client::builder()
        .default_headers(headers)
        .build().unwrap();
    let mut options = QuoteSearchOptions::default();

    let mut current_quote: Option<Quote> = None;

    loop {
        print_menu();
        let mut opt = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut opt) {
            println!("Error reading the input! {e}");
            continue;
        }

        let opt = match opt.trim().parse::<i32>() {
            Ok(o) => o,
            Err(e) => {
                println!("Error parsing the input! {e}");
                continue;
            }
        };

        match opt {
            1 => { // Author
                let mut author = String::new();
                get_option("Enter author's name", &mut author);
                options.author = Some(author);
            },
            2 => { // Category
                let mut categories = String::new();
                get_option("Enter category or categories as a comma separated list", &mut categories);
                let categories: Vec<_> = categories.trim().split(',').collect();
                let categories = categories.iter()
                    .map(|c| Category::from_str(c.trim()))
                    .filter_map(Result::ok)
                    .collect();
                options.categories = Some(categories);
            },
            3 => { // Exclude category
                let mut excluded_categories = String::new();
                get_option("Enter category or categories as a comma separated list", &mut excluded_categories);
                let categories: Vec<_> = excluded_categories.trim().split(',').collect();
                let categories = categories.iter()
                    .map(|c| Category::from_str(c.trim()))
                    .filter_map(Result::ok)
                    .collect();
                options.exclude_categories = Some(categories);
            },
            4 => { // Work
                let mut work = String::new();
                get_option("Enter work's name", &mut work);
                options.work = Some(work);
            },
            5 => { // Discard
                println!("Discarding options!");
                options = QuoteSearchOptions::default();
            },
            6 => { // Search with options
                let quote = match get_quote(&client, &options).await {
                    Ok(q) => q,
                    Err(e) => {
                        println!("Error! {e}");
                        continue;
                    }
                };
                println!("{quote}");
                current_quote = Some(quote);
            },
            7 => { // QOTD
                println!("Here is the quote of the day!");
                let qotd = match get_quote_of_the_day(&client).await {
                    Ok(q) => q,
                    Err(e) => {
                        println!("Error! {e}");
                        continue;
                    }
                };
                println!("{qotd}");
                current_quote = Some(qotd);
            },
            8 => { // Save quote to json
                let q = match current_quote.take() {
                    Some(q) => q,
                    None => {
                        println!("Error! Nothing to write!");
                        continue;
                    }
                };

                if let Err(e) = write_quote_to_json(q) {
                    println!("Error writing! {e}");
                    continue;
                }
            }
            9 => { // Quit
                println!("Goodbye, user!");
                break;
            }
            _ => {
                println!("Invalid option!");
                continue;
            }
        }
    }
}
