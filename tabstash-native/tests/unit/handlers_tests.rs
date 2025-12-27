use tabstash_native::engine::handlers;
use tabstash_native::engine::context::EngineContext;
use tabstash_native::protocol::{StoreTabPayload, RestoreTabPayload, DeleteTabPayload, ListTabsPayload};
use std::sync::Arc;
use sled::Db;
use tempfile::TempDir;

fn create_test_context() -> (TempDir, EngineContext) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let snapshots_dir = temp_dir.path().to_path_buf();
    
    std::fs::create_dir_all(&snapshots_dir).expect("Failed to create snapshots dir");
    
    let db_path = snapshots_dir.join(".index");
    let db = Arc::new(Db::open(&db_path).expect("Failed to open test database"));
    
    let ctx = EngineContext::new(snapshots_dir, db);
    (temp_dir, ctx)
}

#[tokio::test]
async fn test_handle_health_check() {
    let response = handlers::handle_health_check("test-id".to_string()).await;
    
    assert_eq!(response.id, "test-id");
    assert_eq!(response.status, "OK");
    assert!(response.data.is_some());
    assert!(response.error.is_none());
}

#[tokio::test]
async fn test_handle_store_tab() {
    let (_temp_dir, ctx) = create_test_context();
    let payload = StoreTabPayload {
        tab_id: 42,
        url: "https://example.com".to_string(),
        html: "<html><body>Test</body></html>".to_string(),
        timestamp: 1735123123,
    };
    
    let response = handlers::handle_store_tab("req-1".to_string(), &ctx, payload).await;
    
    assert_eq!(response.id, "req-1");
    assert_eq!(response.status, "OK");
    assert!(response.data.is_some());
    assert!(response.error.is_none());
    
    // Verify snapshot_id is in response
    if let Some(data) = response.data {
        assert!(data.get("snapshotId").is_some());
    }
}

#[tokio::test]
async fn test_handle_restore_tab() {
    let (_temp_dir, ctx) = create_test_context();
    
    // First store a tab
    let store_payload = StoreTabPayload {
        tab_id: 42,
        url: "https://example.com".to_string(),
        html: "<html><body>Test content</body></html>".to_string(),
        timestamp: 1735123123,
    };
    
    let store_response = handlers::handle_store_tab("store-req".to_string(), &ctx, store_payload).await;
    assert_eq!(store_response.status, "OK");
    
    // Now restore it
    let restore_payload = RestoreTabPayload { tab_id: 42 };
    let restore_response = handlers::handle_restore_tab("restore-req".to_string(), &ctx, restore_payload).await;
    
    assert_eq!(restore_response.id, "restore-req");
    assert_eq!(restore_response.status, "OK");
    assert!(restore_response.data.is_some());
    assert!(restore_response.error.is_none());
    
    // Verify restored data
    if let Some(data) = restore_response.data {
        assert_eq!(data.get("tabId"), Some(&serde_json::json!(42)));
        assert_eq!(data.get("url"), Some(&serde_json::json!("https://example.com")));
        assert!(data.get("html").is_some());
    }
}

#[tokio::test]
async fn test_handle_restore_nonexistent_tab() {
    let (_temp_dir, ctx) = create_test_context();
    let payload = RestoreTabPayload { tab_id: 999 };
    
    let response = handlers::handle_restore_tab("req-1".to_string(), &ctx, payload).await;
    
    assert_eq!(response.status, "ERROR");
    assert!(response.error.is_some());
    assert!(response.error.unwrap().contains("not found"));
}

#[tokio::test]
async fn test_handle_delete_tab() {
    let (_temp_dir, ctx) = create_test_context();
    
    // First store a tab
    let store_payload = StoreTabPayload {
        tab_id: 42,
        url: "https://example.com".to_string(),
        html: "<html><body>Test</body></html>".to_string(),
        timestamp: 1735123123,
    };
    
    handlers::handle_store_tab("store-req".to_string(), &ctx, store_payload).await;
    
    // Delete it
    let delete_payload = DeleteTabPayload { tab_id: 42 };
    let delete_response = handlers::handle_delete_tab("delete-req".to_string(), &ctx, delete_payload).await;
    
    assert_eq!(delete_response.status, "OK");
    assert!(delete_response.data.is_some());
    
    // Verify it's gone by trying to restore
    let restore_payload = RestoreTabPayload { tab_id: 42 };
    let restore_response = handlers::handle_restore_tab("restore-req".to_string(), &ctx, restore_payload).await;
    assert_eq!(restore_response.status, "ERROR");
}

#[tokio::test]
async fn test_handle_delete_nonexistent_tab() {
    let (_temp_dir, ctx) = create_test_context();
    let payload = DeleteTabPayload { tab_id: 999 };
    
    let response = handlers::handle_delete_tab("req-1".to_string(), &ctx, payload).await;
    
    assert_eq!(response.status, "ERROR");
    assert!(response.error.is_some());
}

#[tokio::test]
async fn test_handle_list_tabs() {
    let (_temp_dir, ctx) = create_test_context();
    
    // Store multiple tabs
    for i in 1..=5 {
        let payload = StoreTabPayload {
            tab_id: i,
            url: format!("https://example{}.com", i),
            html: format!("<html><body>Tab {}</body></html>", i),
            timestamp: 1735123123 + i as u64,
        };
        handlers::handle_store_tab(format!("store-{}", i), &ctx, payload).await;
    }
    
    // List all tabs
    let list_payload = ListTabsPayload {};
    let response = handlers::handle_list_tabs("list-req".to_string(), &ctx, list_payload).await;
    
    assert_eq!(response.status, "OK");
    assert!(response.data.is_some());
    
    if let Some(data) = response.data {
        let tabs = data.get("tabs").expect("Should have tabs array");
        assert_eq!(tabs.as_array().unwrap().len(), 5);
    }
}

#[tokio::test]
async fn test_handle_list_tabs_empty() {
    let (_temp_dir, ctx) = create_test_context();
    let payload = ListTabsPayload {};
    
    let response = handlers::handle_list_tabs("list-req".to_string(), &ctx, payload).await;
    
    assert_eq!(response.status, "OK");
    if let Some(data) = response.data {
        let tabs = data.get("tabs").expect("Should have tabs array");
        assert_eq!(tabs.as_array().unwrap().len(), 0);
    }
}

#[tokio::test]
async fn test_store_restore_roundtrip() {
    let (_temp_dir, ctx) = create_test_context();
    
    let original_payload = StoreTabPayload {
        tab_id: 100,
        url: "https://test.com".to_string(),
        html: "<html><body>Original content</body></html>".to_string(),
        timestamp: 1735123123,
    };
    
    // Store
    handlers::handle_store_tab("store".to_string(), &ctx, original_payload).await;
    
    // Restore
    let restore_payload = RestoreTabPayload { tab_id: 100 };
    let restore_response = handlers::handle_restore_tab("restore".to_string(), &ctx, restore_payload).await;
    
    assert_eq!(restore_response.status, "OK");
    if let Some(data) = restore_response.data {
        assert_eq!(data.get("tabId"), Some(&serde_json::json!(100)));
        assert_eq!(data.get("url"), Some(&serde_json::json!("https://test.com")));
        assert_eq!(data.get("html"), Some(&serde_json::json!("<html><body>Original content</body></html>")));
    }
}

