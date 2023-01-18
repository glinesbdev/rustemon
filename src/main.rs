use rustemon::*;

#[tokio::main]
async fn main() -> std::result::Result<(), ResponseError> {
    let data: SetsResponse = Request::new("sets").query("series:base").search().await?;
    println!("{data:#?}");

    Ok(())
}
