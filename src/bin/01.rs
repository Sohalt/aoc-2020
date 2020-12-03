#[tokio::main]
async fn main() -> reqwest::Result<()> {
    println!("{}", advent::get_input(1).await?);
    Ok(())
}
