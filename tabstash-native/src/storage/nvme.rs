use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::path::{Path, PathBuf};
use anyhow::Result;

pub async fn write_snapshot(snapshots_dir: &Path, snapshot_id: &str, data: &[u8]) -> Result<()> {
    // MANDATORY atomic write pattern:
    // 1. Write to temp file
    let temp_path = snapshots_dir.join(format!("{}.tmp", snapshot_id));
    let final_path = snapshots_dir.join(format!("{}.bin", snapshot_id));
    
    // 2. Write to temp file
    let mut file = fs::File::create(&temp_path).await?;
    file.write_all(data).await?;
    
    // 3. fsync temp file
    file.sync_all().await?;
    drop(file);
    
    // 4. rename temp → final
    fs::rename(&temp_path, &final_path).await?;
    
    // 5. fsync parent directory
    let parent_dir = fs::File::open(snapshots_dir).await?;
    parent_dir.sync_all().await?;
    
    Ok(())
}

pub async fn read_snapshot(snapshots_dir: &Path, snapshot_id: &str) -> Result<Vec<u8>> {
    let path = snapshots_dir.join(format!("{}.bin", snapshot_id));
    Ok(fs::read(path).await?)
}

pub async fn delete_snapshot(snapshots_dir: &Path, snapshot_id: &str) -> Result<()> {
    let path = snapshots_dir.join(format!("{}.bin", snapshot_id));
    fs::remove_file(path).await?;
    Ok(())
}

