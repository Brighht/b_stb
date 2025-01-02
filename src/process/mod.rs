//! Process module provides direct stream processing functionality.
//! 
//! This module contains functions for processing streams without needing to create
//! a StreamConverter instance. Useful for one-off stream processing operations.

use crate::error::StreamConverterError;
use futures_util::StreamExt;
use hyper::Body;

/// Processes a Hyper response body into a vector of bytes.
/// 
/// This is a convenience function that processes a Hyper body without requiring
/// a StreamConverter instance. It's useful for simple, one-off conversions.
/// 
/// # Arguments
/// 
/// * `body` - The Hyper response body to process
/// 
/// # Returns
/// 
/// A Result containing either the processed bytes or a StreamConverterError
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use b_stb::process::process_stream;
/// use hyper::Body;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let body = Body::from("Hello, World!");
///     let bytes = process_stream(body).await?;
///     println!("Processed {} bytes", bytes.len());
///     Ok(())
/// }
/// ```
pub async fn process_stream(mut body: Body) -> Result<Vec<u8>, StreamConverterError> {
    let mut bytes = Vec::new();
    while let Some(chunk) = body.next().await {
        let chunk = chunk.map_err(StreamConverterError::HyperError)?;
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
} 