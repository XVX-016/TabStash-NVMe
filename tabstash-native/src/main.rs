// Modules are defined in lib.rs for testing
use tabstash_native::protocol::{Request, Response};
use tabstash_native::ipc::{read_message, write_message};
use tabstash_native::engine::EngineContext;
use std::path::PathBuf;
use std::sync::Arc;
use sled::Db;
use tokio::fs;
use anyhow::Result;

/// Get the platform-appropriate snapshots directory
fn get_snapshots_directory() -> Result<PathBuf> {
    let base_dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
    
    Ok(base_dir.join("TabStash").join("snapshots"))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize shared state once
    // Use platform-appropriate data directory
    let snapshots_dir = get_snapshots_directory()?;
    
    // Create snapshots directory if missing
    if let Err(e) = fs::create_dir_all(&snapshots_dir).await {
        eprintln!("FATAL: Failed to create snapshots directory: {}", e);
        std::process::exit(1);
    }
    
    // Open sled database once at startup
    // Storage corruption/sled failure → exit immediately
    let db_path = snapshots_dir.join(".index");
    let db = match Db::open(&db_path) {
        Ok(db) => Arc::new(db),
        Err(e) => {
            eprintln!("FATAL: Failed to open sled database: {}", e);
            eprintln!("Storage corruption detected - exiting immediately");
            std::process::exit(1);
        }
    };
    
    // Create engine context
    let ctx = EngineContext::new(snapshots_dir, db);
    
    // Main message loop
    loop {
        match read_message().await {
            Ok(raw) => {
                // Parse request first to get ID for error responses
                match serde_json::from_slice::<Request>(&raw) {
                    Ok(req) => {
                        // Route to handler
                        let response = tabstash_native::engine::handle(req, &ctx).await;
                        if let Err(e) = send_response(&response).await {
                            eprintln!("Failed to send response: {}", e);
                            // Protocol error - continue loop
                        }
                    }
                    Err(e) => {
                        // Can't parse request - respond with unknown ID
                        let error_response = Response {
                            id: "unknown".to_string(),
                            status: "ERROR".to_string(),
                            data: None,
                            error: Some(format!("Invalid request format: {}", e)),
                        };
                        if let Err(send_err) = send_response(&error_response).await {
                            eprintln!("Failed to send error response: {}", send_err);
                        }
                    }
                }
            }
            Err(e) => {
                // I/O error reading message - could be EOF or broken pipe
                // For temporary I/O failures, log and continue
                // For permanent failures (like broken pipe), the loop will naturally exit
                eprintln!("I/O error reading message: {}", e);
                // Continue loop - browser may have closed connection
            }
        }
    }
}

async fn send_response(response: &Response) -> Result<()> {
    let data = serde_json::to_vec(response)?;
    write_message(&data).await?;
    Ok(())
}

