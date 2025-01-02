use std::error::Error;
use std::fmt;
use std::string::FromUtf8Error;
use std::io;
use hyper::Error as HyperError;

/// Represents all possible errors that can occur when converting streams.
/// 
/// This enum provides detailed error information for:
/// - I/O operations
/// - UTF-8 encoding issues
/// - Hyper-specific errors
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use b_stb::{StreamConverter, StreamConverterError};
/// use hyper::Body;
/// 
/// async fn example() {
///     let converter = StreamConverter::new();
///     let body = Body::empty();
///     
///     match converter.body_to_string(body).await {
///         Ok(content) => println!("Success: {}", content),
///         Err(StreamConverterError::EncodingError(e)) => eprintln!("Invalid UTF-8: {}", e),
///         Err(StreamConverterError::IoError(e)) => eprintln!("IO Error: {}", e),
///         Err(StreamConverterError::HyperError(e)) => eprintln!("Hyper Error: {}", e),
///     }
/// }
/// ```
#[derive(Debug)]
pub enum StreamConverterError {
    /// Represents errors that occur during I/O operations
    IoError(io::Error),
    /// Represents errors that occur when converting bytes to UTF-8 strings
    EncodingError(FromUtf8Error),
    /// Represents errors that occur in the Hyper HTTP client
    HyperError(HyperError),
}

impl fmt::Display for StreamConverterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamConverterError::IoError(e) => write!(f, "IO error: {}", e),
            StreamConverterError::EncodingError(e) => write!(f, "Encoding error: {}", e),
            StreamConverterError::HyperError(e) => write!(f, "Hyper error: {}", e),
        }
    }
}

impl Error for StreamConverterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            StreamConverterError::IoError(e) => Some(e),
            StreamConverterError::EncodingError(e) => Some(e),
            StreamConverterError::HyperError(e) => Some(e),
        }
    }
}

