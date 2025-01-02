//! Utility functions for stream and byte processing.
//! 
//! This module provides helper functions for common operations like
//! converting bytes to strings and concatenating byte chunks.

use bytes::Bytes;

/// Converts a byte slice to a UTF-8 string.
/// 
/// This function attempts to convert a slice of bytes into a valid UTF-8 string.
/// If the bytes are not valid UTF-8, it returns None.
/// 
/// # Arguments
/// 
/// * `bytes` - The byte slice to convert
/// 
/// # Returns
/// 
/// Some(String) if the bytes are valid UTF-8, None otherwise
/// 
/// # Examples
/// 
/// ```rust
/// use b_stb::util::bytes_to_string;
/// 
/// let bytes = b"Hello, World!";
/// if let Some(string) = bytes_to_string(bytes) {
///     println!("Converted string: {}", string);
/// }
/// ```
pub fn bytes_to_string(bytes: &[u8]) -> Option<String> {
    String::from_utf8(bytes.to_vec()).ok()
}

/// Concatenates multiple byte chunks into a single vector.
/// 
/// This function efficiently combines multiple `Bytes` chunks into a single
/// contiguous vector of bytes.
/// 
/// # Arguments
/// 
/// * `chunks` - A vector of Bytes chunks to concatenate
/// 
/// # Returns
/// 
/// A `Vec<u8>` containing all bytes from the input chunks
/// 
/// # Examples
/// 
/// ```rust
/// use b_stb::util::concat_bytes;
/// use bytes::Bytes;
/// 
/// let chunks = vec![
///     Bytes::from("Hello"),
///     Bytes::from(", "),
///     Bytes::from("World!"),
/// ];
/// 
/// let combined = concat_bytes(chunks);
/// assert_eq!(&combined, b"Hello, World!");
/// ```
pub fn concat_bytes(chunks: Vec<Bytes>) -> Vec<u8> {
    let mut result = Vec::new();
    for chunk in chunks {
        result.extend_from_slice(&chunk);
    }
    result
}

/// Returns the default buffer size used by StreamConverter.
/// 
/// This function returns the recommended buffer size for stream operations.
/// The default size is optimized for common use cases.
/// 
/// # Returns
/// 
/// The default buffer size in bytes (8KB)
/// 
/// # Examples
/// 
/// ```rust
/// use b_stb::util::default_buffer_size;
/// 
/// let buffer_size = default_buffer_size();
/// assert_eq!(buffer_size, 8192); // 8KB
/// ```
pub fn default_buffer_size() -> usize {
    8192 // 8KB default buffer size
} 