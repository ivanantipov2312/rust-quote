mod quote;

use std::{env, str::FromStr};
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use dotenv;

use quote::{Quote, options::QuoteSearchOptions, response::QuoteResponse, category::Category, get::{get_quote, get_quote_of_the_day}};

pub fn print_menu() {
    println!("1. Choose an author");
    println!("2. Choose category or categories");
    println!("3. Exclude categories");
    println!("4. Choose work");
    println!("5. Discard all options");
    println!("6. Search");
    println!("7. Get quote of the day");
    println!("8. Quit");
}

fn get_option(prompt: &str, s: &mut String) {
    loop {
        println!("{prompt}");
        if let Err(e) = std::io::stdin().read_line(s) {
            println!("{e}");
            continue;
        } else if s == "\n" {
            println!("Empty input is not allowed!");
            s.clear();
            continue;
        } {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found!");
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("X-Api-Key", HeaderValue::from_str(&api_key).unwrap());
    let mut options = QuoteSearchOptions::default();

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
            1 => {
                let mut author = String::new();
                get_option("Enter author's name", &mut author);
                options.author = Some(author);
            },
            2 => {
                let mut categories = String::new();
                get_option("Enter category or categories as a comma separated list", &mut categories);
                let categories: Vec<_> = categories.trim().split(',').collect();
                let categories = categories.iter()
                    .map(|c| Category::from_str(c.trim()))
                    .filter_map(Result::ok)
                    .collect();
                options.categories = Some(categories);
            },
            3 => {
                let mut excluded_categories = String::new();
                get_option("Enter category or categories as a comma separated list", &mut excluded_categories);
                let categories: Vec<_> = excluded_categories.trim().split(',').collect();
                let categories = categories.iter()
                    .map(|c| Category::from_str(c.trim()))
                    .filter_map(Result::ok)
                    .collect();
                options.exclude_categories = Some(categories);
            },
            4 => {
                let mut work = String::new();
                get_option("Enter work's name", &mut work);
                options.work = Some(work);
            },
            5 => {
                println!("Discarding options!");
                options = QuoteSearchOptions::default();
            },
            6 => {
                let quote = match get_quote(client.clone(), headers.clone(), options.clone()).await {
                    Ok(q) => q,
                    Err(e) => {
                        println!("Error! {e}");
                        continue;
                    }
                };
                println!("{quote}");
            },
            7 => {
                println!("Here is the quote of the day!");
                let qotd = match get_quote_of_the_day(client.clone(), headers.clone()).await {
                    Ok(q) => q,
                    Err(e) => {
                        println!("Error! {e}");
                        continue;
                    }
                };
                println!("{qotd}");
            }
            8 => {
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
