use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: String,
    pub version: u32,
    pub action: Action,
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: String,
    pub status: String,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Action {
    StoreTab,
    RestoreTab,
    DeleteTab,
    ListTabs,
    HealthCheck,
}

// Strong payload types for internal use
#[derive(Debug, Deserialize)]
pub struct StoreTabPayload {
    #[serde(rename = "tabId")]
    pub tab_id: u32,
    pub url: String,
    pub html: String,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct RestoreTabPayload {
    #[serde(rename = "tabId")]
    pub tab_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTabPayload {
    #[serde(rename = "tabId")]
    pub tab_id: u32,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ListTabsPayload {
    // Empty for now, can add filters later
}

impl Default for ListTabsPayload {
    fn default() -> Self {
        ListTabsPayload {}
    }
}

