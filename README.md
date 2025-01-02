# b_stb

A Rust library that efficiently converts Hyper response bodies and async streams into strings or bytes. Built for modern web applications, b_stb provides a simple, reliable way to handle streaming data.

## Features

- **Hyper Integration**: Seamlessly works with Hyper's `Body` type
- **Async Support**: Built for async/await with Tokio
- **Flexible Conversion**: Convert streams to both String and bytes
- **Chunked Transfer**: Handles chunked transfer encoding efficiently
- **Configurable**: Customizable buffer sizes for optimal performance
- **Error Handling**: Comprehensive error types for better error management
- **Memory Efficient**: Streams data in chunks to manage memory usage

## Installation

Add b_stb to your `Cargo.toml`:

```toml
[dependencies]
b_stb = "0.1.0"
```

## Quick Start

```rust
use b_stb::StreamConverter;
use hyper::{Body, Client, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a converter instance
    let converter = StreamConverter::new();
    
    
    let client = Client::new();
    let req = Request::builder()
        .uri("https://example.com")
        .body(Body::empty())?;

    let resp = client.request(req).await?;
    let body = resp.into_body();

    
    let content = converter.body_to_string(body).await?;
    println!("Response content: {}", content);

    Ok(())
}
```

## API Reference

### StreamConverter

```rust
// Create a new StreamConverter with default buffer size (8KB)
let converter = StreamConverter::new();

// Create a StreamConverter with custom buffer size
let converter = StreamConverter::with_buffer_size(16384);
```

### Methods

#### `body_to_string`
```rust
async fn body_to_string(&self, body: Body) -> Result<String, StreamConverterError>
```
Converts a Hyper response body into a String.

#### `body_to_bytes`
```rust
async fn body_to_bytes(&self, body: Body) -> Result<Vec<u8>, StreamConverterError>
```
Converts a Hyper response body into a vector of bytes.

#### `to_string`
```rust
async fn to_string<R>(&self, reader: &mut R) -> Result<String, StreamConverterError>
where
    R: AsyncRead + Unpin
```
Converts any async reader into a String.

#### `to_bytes`
```rust
async fn to_bytes<R>(&self, reader: &mut R) -> Result<Vec<u8>, StreamConverterError>
where
    R: AsyncRead + Unpin
```
Converts any async reader into a vector of bytes.

### Utility Functions

```rust
// Convert bytes to string
let string_content = b_stb::util::bytes_to_string(&bytes)?;

// Concatenate multiple byte chunks
let combined = b_stb::util::concat_bytes(chunks);

// Get default buffer size
let buffer_size = b_stb::util::default_buffer_size();
```

## Error Handling

StreamConverter provides detailed error types:
- `IoError`: For input/output errors
- `EncodingError`: For UTF-8 encoding errors
- `HyperError`: For Hyper-specific errors

```rust
match converter.body_to_string(body).await {
    Ok(content) => println!("Content: {}", content),
    Err(StreamConverterError::EncodingError(e)) => eprintln!("Encoding error: {}", e),
    Err(StreamConverterError::IoError(e)) => eprintln!("IO error: {}", e),
    Err(StreamConverterError::HyperError(e)) => eprintln!("Hyper error: {}", e),
}
```

## Examples

### Processing Chunked Data
```rust
let chunks = vec![
    Bytes::from("Hello"),
    Bytes::from(", "),
    Bytes::from("World!"),
];

let body = Body::wrap_stream(futures_util::stream::iter(
    chunks.into_iter().map(Ok::<_, hyper::Error>)
));

let result = converter.body_to_string(body).await?;
assert_eq!(result, "Hello, World!");
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
