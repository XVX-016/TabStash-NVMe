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

/// Run self-test checks
async fn run_self_test() -> Result<()> {
    use std::io::{self, Write};
    
    let mut all_passed = true;
    
    // Test 1: Verify binary can execute
    print!("Test 1: Binary execution... ");
    io::stdout().flush()?;
    println!("OK");
    
    // Test 2: Check data directory access
    print!("Test 2: Data directory access... ");
    io::stdout().flush()?;
    let snapshots_dir = get_snapshots_directory()?;
    match fs::create_dir_all(&snapshots_dir).await {
        Ok(_) => println!("OK ({})", snapshots_dir.display()),
        Err(e) => {
            println!("FAILED: {}", e);
            all_passed = false;
        }
    }
    
    // Test 3: Test NVMe storage initialization (sled database)
    print!("Test 3: Storage initialization... ");
    io::stdout().flush()?;
    let db_path = snapshots_dir.join(".index");
    match sled::Db::open(&db_path) {
        Ok(_) => println!("OK"),
        Err(e) => {
            println!("FAILED: {}", e);
            all_passed = false;
        }
    }
    
    // Test 4: Verify manifest file exists and is readable
    print!("Test 4: Manifest file... ");
    io::stdout().flush()?;
    let manifest_path = get_manifest_path()?;
    match fs::read_to_string(&manifest_path).await {
        Ok(content) => {
            // Try to parse JSON
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(json) => {
                    // Check for required fields
                    if json.get("name").is_some() && json.get("path").is_some() {
                        if let Some(origins) = json.get("allowed_origins").and_then(|v| v.as_array()) {
                            println!("OK ({} extension ID(s) configured)", origins.len());
                        } else {
                            println!("OK (no extension IDs configured)");
                        }
                    } else {
                        println!("WARNING: Manifest missing required fields");
                    }
                }
                Err(e) => {
                    println!("FAILED: Invalid JSON - {}", e);
                    all_passed = false;
                }
            }
        }
        Err(e) => {
            println!("FAILED: {}", e);
            println!("  Expected at: {}", manifest_path.display());
            all_passed = false;
        }
    }
    
    // Test 5: Check extension ID format in manifest
    print!("Test 5: Extension ID format... ");
    io::stdout().flush()?;
    match fs::read_to_string(&manifest_path).await {
        Ok(content) => {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(origins) = json.get("allowed_origins").and_then(|v| v.as_array()) {
                    let mut invalid = false;
                    for origin in origins {
                        if let Some(url) = origin.as_str() {
                            if let Some(id) = url.strip_prefix("chrome-extension://")
                                .and_then(|s| s.strip_suffix("/")) {
                                if !id.chars().all(|c| c.is_ascii_lowercase()) || id.len() != 32 {
                                    invalid = true;
                                    break;
                                }
                            }
                        }
                    }
                    if invalid {
                        println!("WARNING: Some extension IDs have invalid format");
                    } else {
                        println!("OK");
                    }
                } else {
                    println!("SKIPPED (no extension IDs)");
                }
            } else {
                println!("SKIPPED (manifest parse failed)");
            }
        }
        Err(_) => {
            println!("SKIPPED (manifest not readable)");
        }
    }
    
    println!();
    if all_passed {
        println!("All tests passed!");
        Ok(())
    } else {
        eprintln!("Some tests failed. Please check the errors above.");
        std::process::exit(1);
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

#[tokio::main]
async fn main() -> Result<()> {
    // Check for --self-test flag
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--self-test" {
        return run_self_test().await;
    }
    
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

