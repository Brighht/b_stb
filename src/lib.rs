//! # StreamConverter
//! 
//! A Rust library that efficiently converts Hyper response bodies and async streams into strings or bytes.
//! Built for modern web applications, StreamConverter provides a simple, reliable way to handle streaming data.
//! 
//! ## Features
//! 
//! - **Hyper Integration**: Seamlessly works with Hyper's `Body` type
//! - **Async Support**: Built for async/await with Tokio
//! - **Flexible Conversion**: Convert streams to both String and bytes
//! - **Chunked Transfer**: Handles chunked transfer encoding efficiently
//! - **Memory Efficient**: Streams data in chunks to manage memory usage
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use b_stb::StreamConverter;
//! use hyper::{Body, Client, Request};
//! use hyper_tls::HttpsConnector;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let converter = StreamConverter::new();
//!     
//!     // Create an HTTPS client
//!     let https = HttpsConnector::new();
//!     let client = Client::builder().build::<_, hyper::Body>(https);
//! 
//!     // Make a request
//!     let req = Request::builder()
//!         .uri("https://www.example.com")
//!         .header("User-Agent", "StreamConverter/1.0")
//!         .body(Body::empty())?;
//! 
//!     let resp = client.request(req).await?;
//!     let body = resp.into_body();
//! 
//!     // Convert body to string
//!     let content = converter.body_to_string(body).await?;
//!     println!("Content: {}", content);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ## Error Handling
//! 
//! The library provides detailed error types through `StreamConverterError`:
//! 
//! ```rust,no_run
//! use b_stb::{StreamConverter, StreamConverterError};
//! 
//! async fn handle_body(body: hyper::Body) {
//!     let converter = StreamConverter::new();
//!     match converter.body_to_string(body).await {
//!         Ok(content) => println!("Content: {}", content),
//!         Err(StreamConverterError::EncodingError(e)) => eprintln!("Encoding error: {}", e),
//!         Err(StreamConverterError::IoError(e)) => eprintln!("IO error: {}", e),
//!         Err(StreamConverterError::HyperError(e)) => eprintln!("Hyper error: {}", e),
//!     }
//! }
//! ```

pub mod converter;
pub mod error;
pub mod process;
pub mod util;

pub use converter::StreamConverter;
pub use error::StreamConverterError;
