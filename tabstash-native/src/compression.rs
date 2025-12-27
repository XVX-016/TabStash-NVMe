use zstd::{encode_all, decode_all};
use std::io::Cursor;

pub fn compress(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    Ok(encode_all(Cursor::new(data), 3)?)
}

pub fn decompress(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    Ok(decode_all(Cursor::new(data))?)
}

