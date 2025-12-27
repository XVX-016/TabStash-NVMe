use crate::protocol::{Response, StoreTabPayload, RestoreTabPayload, DeleteTabPayload, ListTabsPayload};
use crate::engine::context::EngineContext;
use crate::storage::{nvme, index};
use crate::compression;
use serde_json::json;
use uuid::Uuid;
use anyhow::Result;

pub async fn handle_health_check(request_id: String) -> Response {
    Response {
        id: request_id,
        status: "OK".to_string(),
        data: Some(json!({"alive": true})),
        error: None,
    }
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

