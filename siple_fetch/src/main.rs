use std::error::Error;

use clap::Parser;
use reqwest::blocking::{Client, Response};
use reqwest::header::HeaderMap;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Sends HTTP requests and prints detailed information"
)]
struct Cli {
    #[arg(short, long, help = "Target URL, required = true")]
    url: String,
}

fn main() {
    let cli = Cli::parse();

    let response = send_request(&cli.url).unwrap();

    print_response_details(response).unwrap();
}

fn send_request(url: &str) -> Result<Response, Box<dyn Error>> {
    let client = Client::builder().build()?;
    let response = client.get(url).send()?;

    Ok(response)
}

fn print_response_details(response: Response) -> Result<(), Box<dyn Error>> {
    println!("Status: {}", response.status());
    println!("Headers:");
    print_headers(response.headers());

    let body = response.text()?;
    println!("Body:\n{}", body);

    Ok(())
}

fn print_headers(header: &HeaderMap) {
    for (key, value) in header.iter() {
        println!(" {}: {}", key, value.to_str().unwrap_or("[unprintable!]"));
    }
}
