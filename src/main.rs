#[tokio::main]
async fn main() {
    let response = reqwest::get("https://example.com").await.unwrap();
    let body = response.text().await.unwrap();
    println!("Body: {}", body);
}