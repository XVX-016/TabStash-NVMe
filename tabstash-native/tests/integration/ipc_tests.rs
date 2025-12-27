use tabstash_native::protocol::{Request, Response, Action};

// Helper to create IPC message
fn create_ipc_message(data: &[u8]) -> Vec<u8> {
    let mut message = Vec::new();
    let len = data.len() as u32;
    message.extend_from_slice(&len.to_le_bytes());
    message.extend_from_slice(data);
    message
}

// Helper to parse IPC message
fn parse_ipc_message(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 4 {
        return Err("Message too short".to_string());
    }
    let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    if data.len() < 4 + len {
        return Err("Message incomplete".to_string());
    }
    Ok(data[4..4 + len].to_vec())
}

#[test]
fn test_ipc_message_formatting() {
    let json_data = b"{\"test\": \"data\"}";
    let message = create_ipc_message(json_data);
    
    // Verify length prefix
    let len = u32::from_le_bytes([message[0], message[1], message[2], message[3]]);
    assert_eq!(len, json_data.len() as u32);
    
    // Verify body
    let body = &message[4..];
    assert_eq!(body, json_data);
}

#[test]
fn test_parse_ipc_message() {
    let json_data = b"{\"test\": \"data\"}";
    let message = create_ipc_message(json_data);
    
    let parsed = parse_ipc_message(&message).expect("Parse failed");
    assert_eq!(parsed, json_data);
}

#[test]
fn test_parse_invalid_length() {
    let invalid = b"\x00\x00\x00"; // Too short
    let result = parse_ipc_message(invalid);
    assert!(result.is_err());
}

#[test]
fn test_parse_incomplete_message() {
    let incomplete = b"\x10\x00\x00\x00{\"test\": \"da"; // Length says 16 bytes but only 12 provided
    let result = parse_ipc_message(incomplete);
    assert!(result.is_err());
}

#[test]
fn test_request_response_id_matching() {
    let request = Request {
        id: "test-123".to_string(),
        version: 1,
        action: Action::HealthCheck,
        payload: serde_json::json!({}),
    };
    
    let request_json = serde_json::to_string(&request).expect("Serialize failed");
    let request_obj: serde_json::Value = serde_json::from_str(&request_json).expect("Deserialize failed");
    
    // Verify ID is preserved
    assert_eq!(request_obj.get("id"), Some(&serde_json::json!("test-123")));
}

#[test]
fn test_all_action_types_serialize() {
    let actions = vec![
        Action::HealthCheck,
        Action::StoreTab,
        Action::RestoreTab,
        Action::DeleteTab,
        Action::ListTabs,
    ];
    
    for action in actions {
        let request = Request {
            id: "test".to_string(),
            version: 1,
            action,
            payload: serde_json::json!({}),
        };
        
        let json = serde_json::to_string(&request).expect("Serialize failed");
        let parsed: Request = serde_json::from_str(&json).expect("Deserialize failed");
        assert_eq!(parsed.id, "test");
    }
}

#[test]
fn test_error_response_format() {
    let response = Response {
        id: "req-1".to_string(),
        status: "ERROR".to_string(),
        data: None,
        error: Some("Test error message".to_string()),
    };
    
    let json = serde_json::to_string(&response).expect("Serialize failed");
    let parsed: Response = serde_json::from_str(&json).expect("Deserialize failed");
    
    assert_eq!(parsed.status, "ERROR");
    assert_eq!(parsed.error, Some("Test error message".to_string()));
}

#[test]
fn test_ok_response_format() {
    let response = Response {
        id: "req-1".to_string(),
        status: "OK".to_string(),
        data: Some(serde_json::json!({"snapshotId": "test-123"})),
        error: None,
    };
    
    let json = serde_json::to_string(&response).expect("Serialize failed");
    let parsed: Response = serde_json::from_str(&json).expect("Deserialize failed");
    
    assert_eq!(parsed.status, "OK");
    assert!(parsed.data.is_some());
    assert!(parsed.error.is_none());
}

#[test]
fn test_malformed_json_request() {
    let invalid_json = b"{invalid json}";
    let result: Result<Request, _> = serde_json::from_slice(invalid_json);
    assert!(result.is_err(), "Should fail on invalid JSON");
}

#[test]
fn test_store_tab_request_format() {
    let request = Request {
        id: "req-1".to_string(),
        version: 1,
        action: Action::StoreTab,
        payload: serde_json::json!({
            "tabId": 42,
            "url": "https://example.com",
            "html": "<html></html>",
            "timestamp": 1735123123
        }),
    };
    
    let json = serde_json::to_string(&request).expect("Serialize failed");
    let parsed: Request = serde_json::from_str(&json).expect("Deserialize failed");
    
    assert_eq!(parsed.action, Action::StoreTab);
    assert!(parsed.payload.get("tabId").is_some());
}

#[test]
fn test_restore_tab_request_format() {
    let request = Request {
        id: "req-1".to_string(),
        version: 1,
        action: Action::RestoreTab,
        payload: serde_json::json!({
            "tabId": 42
        }),
    };
    
    let json = serde_json::to_string(&request).expect("Serialize failed");
    let parsed: Request = serde_json::from_str(&json).expect("Deserialize failed");
    
    assert_eq!(parsed.action, Action::RestoreTab);
    assert_eq!(parsed.payload.get("tabId"), Some(&serde_json::json!(42)));
}

