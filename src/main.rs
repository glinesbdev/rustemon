use rustemon::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", request().await?);

    Ok(())
}
