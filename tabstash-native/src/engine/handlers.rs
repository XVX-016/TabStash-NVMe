use crate::protocol::{Response, StoreTabPayload, RestoreTabPayload, DeleteTabPayload, ListTabsPayload};
use crate::engine::context::EngineContext;
use crate::storage::{nvme, index};
use crate::compression;
use serde_json::json;
use uuid::Uuid;
use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

pub async fn handle_health_check(request_id: String) -> Response {
    Response {
        id: request_id,
        status: "OK".to_string(),
        data: Some(json!({"alive": true})),
        error: None,
    }
}

/// Get the platform-appropriate Chrome Native Messaging manifest path
fn get_manifest_path() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA")
            .map_err(|_| anyhow::anyhow!("LOCALAPPDATA environment variable not set"))?;
        Ok(PathBuf::from(local_app_data)
            .join("Google")
            .join("Chrome")
            .join("User Data")
            .join("NativeMessagingHosts")
            .join("tabstash_native.json"))
    }
    
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME")
            .map_err(|_| anyhow::anyhow!("HOME environment variable not set"))?;
        Ok(PathBuf::from(home)
            .join(".config")
            .join("google-chrome")
            .join("NativeMessagingHosts")
            .join("tabstash_native.json"))
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Err(anyhow::anyhow!("Unsupported platform"))
    }
}

pub async fn handle_get_manifest_info(request_id: String) -> Response {
    match get_manifest_info_internal().await {
        Ok(info) => Response {
            id: request_id,
            status: "OK".to_string(),
            data: Some(info),
            error: None,
        },
        Err(e) => Response {
            id: request_id,
            status: "ERROR".to_string(),
            data: None,
            error: Some(format!("Failed to read manifest: {}", e)),
        },
    }
}

async fn get_manifest_info_internal() -> Result<serde_json::Value> {
    let manifest_path = get_manifest_path()?;
    
    // Read manifest file
    let content = fs::read_to_string(&manifest_path).await
        .map_err(|e| anyhow::anyhow!("Failed to read manifest at {}: {}", manifest_path.display(), e))?;
    
    let manifest: Value = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse manifest JSON: {}", e))?;
    
    // Extract allowed_origins
    let allowed_origins = manifest.get("allowed_origins")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("Manifest missing or invalid allowed_origins"))?
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| {
            // Extract extension ID from chrome-extension://ID/ format
            s.strip_prefix("chrome-extension://")
                .and_then(|s| s.strip_suffix("/"))
                .unwrap_or(s)
                .to_string()
        })
        .collect::<Vec<String>>();
    
    // Get version from Cargo.toml at compile time
    let native_version = env!("CARGO_PKG_VERSION");
    let protocol_version = 1u32; // Current protocol version
    
    Ok(json!({
        "allowedOrigins": allowed_origins,
        "nativeVersion": native_version,
        "protocolVersion": protocol_version
    }))
}

pub async fn handle_store_tab(
    request_id: String,
    ctx: &EngineContext,
    payload: StoreTabPayload,
) -> Response {
    match store_tab_internal(ctx, payload).await {
        Ok(snapshot_id) => Response {
            id: request_id,
            status: "OK".to_string(),
            data: Some(json!({ "snapshotId": snapshot_id })),
            error: None,
        },
        Err(e) => Response {
            id: request_id,
            status: "ERROR".to_string(),
            data: None,
            error: Some(format!("Failed to store tab: {}", e)),
        },
    }
}

async fn store_tab_internal(ctx: &EngineContext, payload: StoreTabPayload) -> Result<String> {
    // Serialize payload
    let raw = serde_json::to_vec(&payload)?;
    
    // Compress
    let compressed = compression::compress(&raw)?;
    
    // Generate snapshot ID
    let snapshot_id = Uuid::new_v4().to_string();
    
    // Write to disk (atomic)
    nvme::write_snapshot(&ctx.snapshots_dir, &snapshot_id, &compressed).await?;
    
    // Update index
    let index = index::Index::new(ctx.db.clone());
    let metadata = index::TabMetadata {
        tab_id: payload.tab_id,
        url: payload.url,
        timestamp: payload.timestamp,
        size: compressed.len() as u64,
        snapshot_id: snapshot_id.clone(),
    };
    index.insert(&metadata)?;
    
    Ok(snapshot_id)
}

pub async fn handle_restore_tab(
    request_id: String,
    ctx: &EngineContext,
    payload: RestoreTabPayload,
) -> Response {
    match restore_tab_internal(ctx, payload).await {
        Ok(data) => Response {
            id: request_id,
            status: "OK".to_string(),
            data: Some(data),
            error: None,
        },
        Err(e) => Response {
            id: request_id,
            status: "ERROR".to_string(),
            data: None,
            error: Some(format!("Failed to restore tab: {}", e)),
        },
    }
}

async fn restore_tab_internal(
    ctx: &EngineContext,
    payload: RestoreTabPayload,
) -> Result<serde_json::Value> {
    // Query index
    let index = index::Index::new(ctx.db.clone());
    let metadata = index.get(payload.tab_id)?
        .ok_or_else(|| anyhow::anyhow!("Tab {} not found", payload.tab_id))?;
    
    // Read file
    let compressed = nvme::read_snapshot(&ctx.snapshots_dir, &metadata.snapshot_id).await?;
    
    // Decompress
    let raw = compression::decompress(&compressed)?;
    
    // Deserialize
    let payload: StoreTabPayload = serde_json::from_slice(&raw)?;
    
    Ok(json!({
        "tabId": payload.tab_id,
        "url": payload.url,
        "html": payload.html,
        "timestamp": payload.timestamp,
    }))
}

pub async fn handle_delete_tab(
    request_id: String,
    ctx: &EngineContext,
    payload: DeleteTabPayload,
) -> Response {
    match delete_tab_internal(ctx, payload).await {
        Ok(_) => Response {
            id: request_id,
            status: "OK".to_string(),
            data: Some(json!({"deleted": true})),
            error: None,
        },
        Err(e) => Response {
            id: request_id,
            status: "ERROR".to_string(),
            data: None,
            error: Some(format!("Failed to delete tab: {}", e)),
        },
    }
}

async fn delete_tab_internal(ctx: &EngineContext, payload: DeleteTabPayload) -> Result<()> {
    // Get metadata to find snapshot_id
    let index = index::Index::new(ctx.db.clone());
    let metadata = index.get(payload.tab_id)?
        .ok_or_else(|| anyhow::anyhow!("Tab {} not found", payload.tab_id))?;
    
    // Delete file
    nvme::delete_snapshot(&ctx.snapshots_dir, &metadata.snapshot_id).await?;
    
    // Remove from index
    index.remove(payload.tab_id)?;
    
    Ok(())
}

pub async fn handle_list_tabs(
    request_id: String,
    ctx: &EngineContext,
    _payload: ListTabsPayload,
) -> Response {
    match list_tabs_internal(ctx).await {
        Ok(tabs) => Response {
            id: request_id,
            status: "OK".to_string(),
            data: Some(json!({ "tabs": tabs })),
            error: None,
        },
        Err(e) => Response {
            id: request_id,
            status: "ERROR".to_string(),
            data: None,
            error: Some(format!("Failed to list tabs: {}", e)),
        },
    }
}

async fn list_tabs_internal(ctx: &EngineContext) -> Result<Vec<serde_json::Value>> {
    let index = index::Index::new(ctx.db.clone());
    let metadata_list = index.list_all()?;
    
    let tabs: Vec<serde_json::Value> = metadata_list
        .into_iter()
        .map(|m| {
            json!({
                "tabId": m.tab_id,
                "url": m.url,
                "timestamp": m.timestamp,
                "size": m.size,
                "snapshotId": m.snapshot_id,
            })
        })
        .collect();
    
    Ok(tabs)
}

