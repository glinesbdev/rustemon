use std::env;

use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client,
};

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn request() -> Result<String> {
    let client = Client::new();
    let result = client
        .get("https://api.pokemontcg.io/v2/cards")
        .header("X-Api-Key", env::var("PTCG_API_KEY")?)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "json")
        .send()
        .await?
        .text()
        .await?;

    Ok(result)
}
