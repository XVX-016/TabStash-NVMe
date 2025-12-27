use std::sync::Arc;
use sled::Db;
use tempfile::TempDir;
use tabstash_native::engine::context::EngineContext;
use tabstash_native::protocol::StoreTabPayload;

pub fn create_test_context() -> (TempDir, EngineContext) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let snapshots_dir = temp_dir.path().to_path_buf();
    
    // Create snapshots subdirectory
    std::fs::create_dir_all(&snapshots_dir).expect("Failed to create snapshots dir");
    
    // Open sled database
    let db_path = snapshots_dir.join(".index");
    let db = Arc::new(Db::open(&db_path).expect("Failed to open test database"));
    
    let ctx = EngineContext::new(snapshots_dir, db);
    (temp_dir, ctx)
}

pub fn create_test_snapshot_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

pub fn create_test_db(path: &std::path::Path) -> Arc<Db> {
    Arc::new(Db::open(path).expect("Failed to open test database"))
}

pub fn sample_store_payload() -> StoreTabPayload {
    StoreTabPayload {
        tab_id: 42,
        url: "https://example.com".to_string(),
        html: "<html><body>Test content</body></html>".to_string(),
        timestamp: 1735123123,
    }
}

pub fn sample_store_payload_with_id(tab_id: u32) -> StoreTabPayload {
    StoreTabPayload {
        tab_id,
        url: format!("https://example{}.com", tab_id),
        html: format!("<html><body>Test content for tab {}</body></html>", tab_id),
        timestamp: 1735123123 + tab_id as u64,
    }
}

pub fn send_ipc_message(data: &[u8]) -> Vec<u8> {
    let mut message = Vec::new();
    let len = data.len() as u32;
    message.extend_from_slice(&len.to_le_bytes());
    message.extend_from_slice(data);
    message
}

pub fn receive_ipc_message(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 4 {
        return Err("Message too short".to_string());
    }
    let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    if data.len() < 4 + len {
        return Err("Message incomplete".to_string());
    }
    Ok(data[4..4 + len].to_vec())
}

