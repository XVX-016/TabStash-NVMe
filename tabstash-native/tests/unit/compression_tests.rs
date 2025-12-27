use tabstash_native::compression;

#[test]
fn test_compress_decompress_roundtrip_small() {
    let original = b"Hello, world!";
    let compressed = compression::compress(original).expect("Compression failed");
    let decompressed = compression::decompress(&compressed).expect("Decompression failed");
    assert_eq!(original, decompressed.as_slice());
}

#[test]
fn test_compress_decompress_roundtrip_medium() {
    let original = vec![0u8; 1024 * 1024]; // 1 MB
    let compressed = compression::compress(&original).expect("Compression failed");
    let decompressed = compression::decompress(&compressed).expect("Decompression failed");
    assert_eq!(original, decompressed);
}

#[test]
fn test_compress_decompress_roundtrip_large() {
    // Simulate large tab (50 MB)
    let original: Vec<u8> = (0..50 * 1024 * 1024).map(|i| (i % 256) as u8).collect();
    let compressed = compression::compress(&original).expect("Compression failed");
    let decompressed = compression::decompress(&compressed).expect("Decompression failed");
    assert_eq!(original, decompressed);
}

#[test]
fn test_compress_decompress_empty() {
    let original = b"";
    let compressed = compression::compress(original).expect("Compression failed");
    let decompressed = compression::decompress(&compressed).expect("Decompression failed");
    assert_eq!(original, decompressed.as_slice());
}

#[test]
fn test_compression_reduces_size() {
    // Create data with repetition (should compress well)
    let original: Vec<u8> = vec![0u8; 10000].repeat(100); // 1 MB of zeros
    let compressed = compression::compress(&original).expect("Compression failed");
    // Compressed should be smaller (zeros compress very well)
    assert!(compressed.len() < original.len());
}

#[test]
fn test_decompress_corrupt_data() {
    let corrupt_data = b"not valid compressed data";
    let result = compression::decompress(corrupt_data);
    assert!(result.is_err(), "Should fail on corrupt data");
}

#[test]
fn test_compress_html_content() {
    // Simulate HTML content
    let html = r#"<html><head><title>Test</title></head><body><h1>Hello World</h1><p>This is a test page with some content that should compress reasonably well.</p></body></html>"#.repeat(1000);
    let original = html.as_bytes();
    let compressed = compression::compress(original).expect("Compression failed");
    let decompressed = compression::decompress(&compressed).expect("Decompression failed");
    assert_eq!(original, decompressed.as_slice());
}

