use anyhow::Result;
use futures::future::join_all;
use scraper::{Html, Selector};

async fn fetch_title(url: String) -> Result<()> {
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

    println!("{url} -> {title}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let urls: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    if urls.is_empty() {
        anyhow::bail!("please provide urls");
    }

    let tasks = urls.into_iter()
        .map(|url| fetch_title(url));

    let results = join_all(tasks).await;

    for result in results {
        result?;
    }
    
    Ok(())
}