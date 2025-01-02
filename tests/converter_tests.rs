use hyper::Body;
use b_stb::{StreamConverter, process::process_stream, util::bytes_to_string};
use bytes::Bytes;

#[tokio::test]
async fn test_body_to_string() {
    let converter = StreamConverter::new();
    let body = Body::from("stream is string!");
    
    let result = converter.body_to_string(body).await.unwrap();
    assert_eq!(result, "stream is string!");
}

#[tokio::test]
async fn test_body_to_bytes() {
    let converter = StreamConverter::new();
    let expected = b"steam is byte!".to_vec();
    let body = Body::from(expected.clone());
    
    let result = converter.body_to_bytes(body).await.unwrap();
    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_process_stream() {
    let body = Body::from("Test Stream");
    let bytes = process_stream(body).await.unwrap();
    let result = bytes_to_string(&bytes).unwrap();
    
    assert_eq!(result, "Test Stream");
}

#[tokio::test]
async fn test_large_body() {
    let converter = StreamConverter::new();
    let large_string = "a".repeat(16384); // 16KB string
    let body = Body::from(large_string.clone());
    
    let result = converter.body_to_string(body).await.unwrap();
    assert_eq!(result, large_string);
}

#[tokio::test]
async fn test_chunked_body() {
    let converter = StreamConverter::new();
    let chunks = vec![
        Bytes::from("Hello"),
        Bytes::from(", "),
        Bytes::from("World!"),
    ];
    
    let body = Body::wrap_stream(futures_util::stream::iter(
        chunks.into_iter().map(Ok::<_, hyper::Error>)
    ));
    
    let result = converter.body_to_string(body).await.unwrap();
    assert_eq!(result, "Hello, World!");
} 