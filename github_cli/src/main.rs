use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
    name: Option<String>,
    followers: u32,
    public_repos: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let username = std::env::args()
        .nth(1)
        .expect("please provide a github username");

    let url = format!("https://api.github.com/users/{username}");

    let client = reqwest::Client::new();

    let user: GitHubUser = client
        .get(&url)
        .header("User-Agent", "rust-github-cli")
        .send()
        .await?
        .json()
        .await?;

    println!("Login: {}", user.login);

    println!(
        "Name: {}",
        user.name.unwrap_or("No name".to_string())
    );

    println!("Followers: {}", user.followers);
    println!("Public repos: {}", user.public_repos);

    Ok(())
}
