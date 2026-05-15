use anyhow::Result;
use serde_json::Value;
use std::fs;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .expect("please provide a json file");

    let content = fs::read_to_string(path)?;

    let json: Value = serde_json::from_str(&content)?;

    let pretty = serde_json::to_string_pretty(&json)?;

    println!("{pretty}");

    Ok(())
}