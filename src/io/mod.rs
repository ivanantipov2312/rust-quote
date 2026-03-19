use std::fs::OpenOptions;
use std::io::{Error as IoError, ErrorKind, Write};
use crate::{Quote, QuoteResponse};

pub fn write_quote_to_json(quote: &Option<Quote>) -> Result<(), Box<dyn std::error::Error>> {
    let quote = if let Some(q) = quote {
        q
    } else {
        return Err(Box::new(IoError::new(ErrorKind::Other, "Nothing to write!")));
    };

    let mut filename = String::new();
    get_option("Enter filename:", &mut filename);
    println!("Saving to {filename}...");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    let formatted = QuoteResponse::from(quote);
    let json = serde_json::to_string_pretty(&formatted)?;
    file.write_all(json.as_bytes())?;
    
    Ok(())
}

pub fn print_menu() {
    println!("1. Choose an author");
    println!("2. Choose category or categories");
    println!("3. Exclude categories");
    println!("4. Choose work");
    println!("5. Discard all options");
    println!("6. Search");
    println!("7. Get quote of the day");
    println!("8. Save current quote to the json file");
    println!("9. Quit");
}

pub fn get_option(prompt: &str, s: &mut String) {
    loop {
        println!("{prompt}");
        if let Err(e) = std::io::stdin().read_line(s) {
            println!("{e}");
            continue;
        } else if s.trim() == "" {
            println!("Empty input is not allowed!");
            s.clear();
            continue;
        } else {
            *s = s.trim().to_string();
            break;
        }
    }
}
