use tabstash_native::storage::{Index, TabMetadata};
use std::sync::Arc;
use sled::Db;
use tempfile::TempDir;

fn create_test_db() -> (TempDir, Arc<Db>) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join(".index");
    let db = Arc::new(Db::open(&db_path).expect("Failed to open test database"));
    (temp_dir, db)
}

#[test]
fn test_index_insert_and_get() {
    let (_temp_dir, db) = create_test_db();
    let index = Index::new(db);
    
    let metadata = TabMetadata {
        tab_id: 42,
        url: "https://example.com".to_string(),
        timestamp: 1735123123,
        size: 1024,
        snapshot_id: "snapshot-123".to_string(),
    };
    
    index.insert(&metadata).expect("Insert failed");
    
    let retrieved = index.get(42).expect("Get failed");
    assert!(retrieved.is_some());
    
    let retrieved_meta = retrieved.unwrap();
    assert_eq!(retrieved_meta.tab_id, 42);
    assert_eq!(retrieved_meta.url, "https://example.com");
    assert_eq!(retrieved_meta.snapshot_id, "snapshot-123");
}

#[test]
fn test_index_get_nonexistent() {
    let (_temp_dir, db) = create_test_db();
    let index = Index::new(db);
    
    let result = index.get(999).expect("Get should not fail");
    assert!(result.is_none(), "Should return None for nonexistent tab");
}

#[test]
fn test_index_remove() {
    let (_temp_dir, db) = create_test_db();
    let index = Index::new(db);
    
    let metadata = TabMetadata {
        tab_id: 42,
        url: "https://example.com".to_string(),
        timestamp: 1735123123,
        size: 1024,
        snapshot_id: "snapshot-123".to_string(),
    };
    
    index.insert(&metadata).expect("Insert failed");
    assert!(index.get(42).expect("Get failed").is_some());
    
    index.remove(42).expect("Remove failed");
    assert!(index.get(42).expect("Get failed").is_none());
}

#[test]
fn test_index_list_all() {
    let (_temp_dir, db) = create_test_db();
    let index = Index::new(db);
    
    // Insert multiple tabs
    for i in 1..=5 {
        let metadata = TabMetadata {
            tab_id: i,
            url: format!("https://example{}.com", i),
            timestamp: 1735123123 + i as u64,
            size: 1024 * i as u64,
            snapshot_id: format!("snapshot-{}", i),
        };
        index.insert(&metadata).expect("Insert failed");
    }
    
    let all = index.list_all().expect("List failed");
    assert_eq!(all.len(), 5);
    
    // Verify all are present
    for i in 1..=5 {
        assert!(all.iter().any(|m| m.tab_id == i));
    }
}

#[test]
fn test_index_list_empty() {
    let (_temp_dir, db) = create_test_db();
    let index = Index::new(db);
    
    let all = index.list_all().expect("List failed");
    assert_eq!(all.len(), 0);
}

#[test]
fn test_index_persistence_across_restart() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join(".index");
    
    // First session: insert data
    {
        let db = Arc::new(Db::open(&db_path).expect("Failed to open database"));
        let index = Index::new(db);
        
        let metadata = TabMetadata {
            tab_id: 100,
            url: "https://persistent.com".to_string(),
            timestamp: 1735123123,
            size: 2048,
            snapshot_id: "persistent-snapshot".to_string(),
        };
        
        index.insert(&metadata).expect("Insert failed");
    }
    
    // Simulate restart: close and reopen database
    {
        let db = Arc::new(Db::open(&db_path).expect("Failed to reopen database"));
        let index = Index::new(db);
        
        let retrieved = index.get(100).expect("Get failed");
        assert!(retrieved.is_some());
        
        let metadata = retrieved.unwrap();
        assert_eq!(metadata.tab_id, 100);
        assert_eq!(metadata.url, "https://persistent.com");
        assert_eq!(metadata.snapshot_id, "persistent-snapshot");
    }
}

#[test]
fn test_index_multiple_tabs_persistence() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join(".index");
    
    // Insert multiple tabs
    {
        let db = Arc::new(Db::open(&db_path).expect("Failed to open database"));
        let index = Index::new(db);
        
        for i in 1..=10 {
            let metadata = TabMetadata {
                tab_id: i,
                url: format!("https://site{}.com", i),
                timestamp: 1735123123 + i as u64,
                size: 1024 * i as u64,
                snapshot_id: format!("snap-{}", i),
            };
            index.insert(&metadata).expect("Insert failed");
        }
    }
    
    // Restart and verify all are still there
    {
        let db = Arc::new(Db::open(&db_path).expect("Failed to reopen database"));
        let index = Index::new(db);
        
        let all = index.list_all().expect("List failed");
        assert_eq!(all.len(), 10);
        
        for i in 1..=10 {
            let retrieved = index.get(i).expect("Get failed");
            assert!(retrieved.is_some());
            assert_eq!(retrieved.unwrap().tab_id, i);
        }
    }
}

#[test]
fn test_index_metadata_correctness() {
    let (_temp_dir, db) = create_test_db();
    let index = Index::new(db);
    
    let original = TabMetadata {
        tab_id: 200,
        url: "https://test.com/page".to_string(),
        timestamp: 1234567890,
        size: 54321,
        snapshot_id: "unique-snapshot-id-12345".to_string(),
    };
    
    index.insert(&original).expect("Insert failed");
    
    let retrieved = index.get(200).expect("Get failed").unwrap();
    
    // Verify all fields match
    assert_eq!(retrieved.tab_id, original.tab_id);
    assert_eq!(retrieved.url, original.url);
    assert_eq!(retrieved.timestamp, original.timestamp);
    assert_eq!(retrieved.size, original.size);
    assert_eq!(retrieved.snapshot_id, original.snapshot_id);
}

#[test]
fn test_index_update_overwrites() {
    let (_temp_dir, db) = create_test_db();
    let index = Index::new(db);
    
    // Insert initial metadata
    let metadata1 = TabMetadata {
        tab_id: 50,
        url: "https://old.com".to_string(),
        timestamp: 1000,
        size: 100,
        snapshot_id: "old-snapshot".to_string(),
    };
    index.insert(&metadata1).expect("Insert failed");
    
    // Update with new metadata (same tab_id)
    let metadata2 = TabMetadata {
        tab_id: 50,
        url: "https://new.com".to_string(),
        timestamp: 2000,
        size: 200,
        snapshot_id: "new-snapshot".to_string(),
    };
    index.insert(&metadata2).expect("Insert failed");
    
    // Verify it was overwritten
    let retrieved = index.get(50).expect("Get failed").unwrap();
    assert_eq!(retrieved.url, "https://new.com");
    assert_eq!(retrieved.snapshot_id, "new-snapshot");
}

