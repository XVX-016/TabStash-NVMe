use tabstash_native::storage::nvme;
use tempfile::TempDir;
use std::path::Path;
use tokio::fs;

#[tokio::test]
async fn test_write_and_read_snapshot() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let snapshot_id = "test-snapshot-123";
    let data = b"test snapshot data";
    
    nvme::write_snapshot(temp_dir.path(), snapshot_id, data)
        .await
        .expect("Write failed");
    
    let read_data = nvme::read_snapshot(temp_dir.path(), snapshot_id)
        .await
        .expect("Read failed");
    
    assert_eq!(data, read_data.as_slice());
}

#[tokio::test]
async fn test_atomic_write_no_partial_file() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let snapshot_id = "atomic-test";
    let data = b"atomic write test data";
    
    // Write should create final file, not leave temp file
    nvme::write_snapshot(temp_dir.path(), snapshot_id, data)
        .await
        .expect("Write failed");
    
    // Verify final file exists
    let final_path = temp_dir.path().join(format!("{}.bin", snapshot_id));
    assert!(final_path.exists(), "Final file should exist");
    
    // Verify temp file does NOT exist
    let temp_path = temp_dir.path().join(format!("{}.tmp", snapshot_id));
    assert!(!temp_path.exists(), "Temp file should not exist after write");
}

#[tokio::test]
async fn test_read_nonexistent_snapshot() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let result = nvme::read_snapshot(temp_dir.path(), "nonexistent")
        .await;
    
    assert!(result.is_err(), "Should fail on nonexistent file");
}

#[tokio::test]
async fn test_delete_snapshot() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let snapshot_id = "delete-test";
    let data = b"data to delete";
    
    // Write snapshot
    nvme::write_snapshot(temp_dir.path(), snapshot_id, data)
        .await
        .expect("Write failed");
    
    // Verify it exists
    let final_path = temp_dir.path().join(format!("{}.bin", snapshot_id));
    assert!(final_path.exists());
    
    // Delete it
    nvme::delete_snapshot(temp_dir.path(), snapshot_id)
        .await
        .expect("Delete failed");
    
    // Verify it's gone
    assert!(!final_path.exists(), "File should be deleted");
}

#[tokio::test]
async fn test_delete_nonexistent_snapshot() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let result = nvme::delete_snapshot(temp_dir.path(), "nonexistent")
        .await;
    
    assert!(result.is_err(), "Should fail on nonexistent file");
}

#[tokio::test]
async fn test_concurrent_writes() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    
    // Write multiple snapshots concurrently
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let path = temp_dir.path().to_path_buf();
            let snapshot_id = format!("concurrent-{}", i);
            let data = format!("data for snapshot {}", i).into_bytes();
            tokio::spawn(async move {
                nvme::write_snapshot(&path, &snapshot_id, &data).await
            })
        })
        .collect();
    
    // Wait for all writes
    for handle in handles {
        handle.await.expect("Task failed").expect("Write failed");
    }
    
    // Verify all files exist
    for i in 0..10 {
        let snapshot_id = format!("concurrent-{}", i);
        let final_path = temp_dir.path().join(format!("{}.bin", snapshot_id));
        assert!(final_path.exists(), "File {} should exist", i);
        
        // Verify content
        let read_data = nvme::read_snapshot(temp_dir.path(), &snapshot_id)
            .await
            .expect("Read failed");
        let expected = format!("data for snapshot {}", i);
        assert_eq!(expected.as_bytes(), read_data.as_slice());
    }
}

#[tokio::test]
async fn test_large_snapshot_write() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let snapshot_id = "large-snapshot";
    // 10 MB of data
    let data: Vec<u8> = (0..10 * 1024 * 1024).map(|i| (i % 256) as u8).collect();
    
    nvme::write_snapshot(temp_dir.path(), snapshot_id, &data)
        .await
        .expect("Write failed");
    
    let read_data = nvme::read_snapshot(temp_dir.path(), snapshot_id)
        .await
        .expect("Read failed");
    
    assert_eq!(data.len(), read_data.len());
    assert_eq!(data, read_data);
}

