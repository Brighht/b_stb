use hyper::{Body, Client, Request};
use b_stb::{StreamConverter, process::process_stream, util::bytes_to_string};
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a converter instance
    let converter = StreamConverter::new();
    
    // Create an HTTPS client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Method 1: Using StreamConverter directly
    let req = Request::builder()
        .uri("https://www.google.com")
        .header("User-Agent", "StreamConverter/1.0")
        .body(Body::empty())?;

    let resp = client.request(req).await?;
    let body = resp.into_body();

    let content = converter.body_to_string(body).await?;
    println!("Content using converter: {}", content);

    // Method 2: Using process module
    let req2 = Request::builder()
        .uri("https://www.google.com")
        .header("User-Agent", "StreamConverter/1.0")
        .body(Body::empty())?;
    let resp2 = client.request(req2).await?;
    let body2 = resp2.into_body();
    
    let bytes = process_stream(body2).await?;
    if let Some(content) = bytes_to_string(&bytes) {
        println!("Content using process module: {}", content);
    }

    Ok(())
} 