use tokio::io::{AsyncRead, AsyncReadExt};
use futures_util::StreamExt;
use hyper::Body;

use crate::error::StreamConverterError;

/// A utility for converting various types of streams into strings or bytes.
/// 
/// `StreamConverter` provides methods to efficiently convert Hyper response bodies
/// and async readers into strings or byte vectors. It handles chunked data and
/// supports configurable buffer sizes for optimal performance.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use b_stb::StreamConverter;
/// use hyper::Body;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let converter = StreamConverter::new();
///     let body = Body::from("Hello, World!");
///     
///     let content = converter.body_to_string(body).await?;
///     assert_eq!(content, "Hello, World!");
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct StreamConverter {
    buffer_size: usize,
}

impl StreamConverter {
    /// Creates a new StreamConverter with the default buffer size (8KB).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use b_stb::StreamConverter;
    /// 
    /// let converter = StreamConverter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            buffer_size: 8192 // Default 8KB buffer
        }
    }

    /// Creates a new StreamConverter with a custom buffer size.
    /// 
    /// # Arguments
    /// 
    /// * `buffer_size` - The size of the internal buffer in bytes
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use b_stb::StreamConverter;
    /// 
    /// let converter = StreamConverter::with_buffer_size(16384); // 16KB buffer
    /// ```
    pub fn with_buffer_size(buffer_size: usize) -> Self {
        Self { buffer_size }
    }

    /// Converts a Hyper body into a String.
    /// 
    /// This method efficiently handles chunked transfer encoding and
    /// automatically manages memory usage through streaming.
    /// 
    /// # Arguments
    /// 
    /// * `body` - The Hyper response body to convert
    /// 
    /// # Returns
    /// 
    /// A Result containing either the converted String or a StreamConverterError
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use b_stb::StreamConverter;
    /// use hyper::Body;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let converter = StreamConverter::new();
    ///     let body = Body::from("Hello");
    ///     
    ///     let content = converter.body_to_string(body).await?;
    ///     assert_eq!(content, "Hello");
    ///     Ok(())
    /// }
    /// ```
    pub async fn body_to_string(&self, body: Body) -> Result<String, StreamConverterError> {
        let bytes = self.body_to_bytes(body).await?;
        String::from_utf8(bytes)
            .map_err(StreamConverterError::EncodingError)
    }

    /// Converts a Hyper body into a vector of bytes.
    /// 
    /// # Arguments
    /// 
    /// * `body` - The Hyper response body to convert
    /// 
    /// # Returns
    /// 
    /// A Result containing either the byte vector or a StreamConverterError
    pub async fn body_to_bytes(&self, mut body: Body) -> Result<Vec<u8>, StreamConverterError> {
        let mut bytes = Vec::new();
        while let Some(chunk) = body.next().await {
            let chunk = chunk.map_err(StreamConverterError::HyperError)?;
            bytes.extend_from_slice(&chunk);
        }
        Ok(bytes)
    }

    /// Converts an async reader into a String.
    /// 
    /// This method reads from any async reader that implements `AsyncRead` and `Unpin`,
    /// converting the bytes into a UTF-8 string.
    /// 
    /// # Arguments
    /// 
    /// * `reader` - Any async reader implementing AsyncRead + Unpin
    /// 
    /// # Returns
    /// 
    /// A Result containing either the converted String or a StreamConverterError
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use b_stb::StreamConverter;
    /// use tokio::fs::File;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let converter = StreamConverter::new();
    ///     let mut file = File::open("example.txt").await?;
    ///     
    ///     let content = converter.to_string(&mut file).await?;
    ///     println!("File content: {}", content);
    ///     Ok(())
    /// }
    /// ```
    pub async fn to_string<R>(&self, reader: &mut R) -> Result<String, StreamConverterError>
    where
        R: AsyncRead + Unpin,
    {
        let mut buffer = vec![0; self.buffer_size];
        let mut result = String::new();

        loop {
            let bytes_read = reader.read(&mut buffer).await
                .map_err(StreamConverterError::IoError)?;
            
            if bytes_read == 0 {
                break;
            }

            let chunk = String::from_utf8(buffer[..bytes_read].to_vec())
                .map_err(StreamConverterError::EncodingError)?;
            result.push_str(&chunk);
        }

        Ok(result)
    }

    /// Converts an async reader into a vector of bytes.
    /// 
    /// This method efficiently reads from any async reader that implements `AsyncRead` and `Unpin`,
    /// collecting the bytes into a vector.
    /// 
    /// # Arguments
    /// 
    /// * `reader` - Any async reader implementing AsyncRead + Unpin
    /// 
    /// # Returns
    /// 
    /// A Result containing either the byte vector or a StreamConverterError
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use b_stb::StreamConverter;
    /// use tokio::fs::File;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let converter = StreamConverter::new();
    ///     let mut file = File::open("image.jpg").await?;
    ///     
    ///     let bytes = converter.to_bytes(&mut file).await?;
    ///     println!("File size: {} bytes", bytes.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn to_bytes<R>(&self, reader: &mut R) -> Result<Vec<u8>, StreamConverterError>
    where
        R: AsyncRead + Unpin,
    {
        let mut buffer = vec![0; self.buffer_size];
        let mut result = Vec::new();

        loop {
            let bytes_read = reader.read(&mut buffer).await
                .map_err(StreamConverterError::IoError)?;
            
            if bytes_read == 0 {
                break;
            }

            result.extend_from_slice(&buffer[..bytes_read]);
        }

        Ok(result)
    }
}

impl Default for StreamConverter {
    fn default() -> Self {
        Self::new()
    }
}
