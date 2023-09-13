use html2md;
use reqwest;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let url = args.get(1);
    let default_name = String::from("xxx.md");
    let output = args.get(2).unwrap_or(&default_name);

    if let Some(url) = url {
        println!("Fetching url: {}", url);
        let body = reqwest::blocking::get(url)?.text()?;

        println!("Converting html to markdown...");
        let md = html2md::parse_html(&body);

        fs::write(output, md.as_bytes())?;
        println!("Converted markdown has been saved in {}.", output);
    } else {
        println!("Url is invalid!");
    }
    Ok(())
}
