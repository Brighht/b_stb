use b_stb::util::{bytes_to_string, concat_bytes, default_buffer_size};
use bytes::Bytes;

#[test]
fn test_bytes_to_string() {
    let bytes = b"Hello, World!";
    let result = bytes_to_string(bytes).unwrap();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_concat_bytes() {
    let chunks = vec![
        Bytes::from("Hello"),
        Bytes::from(", "),
        Bytes::from("World!"),
    ];
    
    let result = concat_bytes(chunks);
    assert_eq!(result, b"Hello, World!");
}

#[test]
fn test_default_buffer_size() {
    assert_eq!(default_buffer_size(), 8192);
}

#[test]
fn test_invalid_utf8() {
    let invalid_utf8 = vec![0xFF, 0xFF, 0xFF];
    let result = bytes_to_string(&invalid_utf8);
    assert!(result.is_none());
} 