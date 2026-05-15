use anyhow::Result;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {
    let url = std::env::args()
        .nth(1)
        .expect("please provide a url");

    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);

    let selector = Selector::parse("title").unwrap();

    let title = document
        .select(&selector)
        .next()
        .map(|element| element.inner_html())
        .unwrap_or("No title found".to_string());

    println!("{title}");
    
    Ok(())
}