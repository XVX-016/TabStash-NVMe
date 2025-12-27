use crate::protocol::{Request, Response, Action, StoreTabPayload, RestoreTabPayload, DeleteTabPayload, ListTabsPayload};
use crate::engine::context::EngineContext;
use crate::engine::handlers;
use anyhow::Result;

pub async fn handle(req: Request, ctx: &EngineContext) -> Response {
    match req.action {
        Action::HealthCheck => {
            handlers::handle_health_check(req.id).await
        }
        Action::StoreTab => {
            match deserialize_payload::<StoreTabPayload>(&req.payload) {
                Ok(payload) => handlers::handle_store_tab(req.id, ctx, payload).await,
                Err(e) => Response {
                    id: req.id,
                    status: "ERROR".to_string(),
                    data: None,
                    error: Some(format!("Invalid payload: {}", e)),
                },
            }
        }
        Action::RestoreTab => {
            match deserialize_payload::<RestoreTabPayload>(&req.payload) {
                Ok(payload) => handlers::handle_restore_tab(req.id, ctx, payload).await,
                Err(e) => Response {
                    id: req.id,
                    status: "ERROR".to_string(),
                    data: None,
                    error: Some(format!("Invalid payload: {}", e)),
                },
            }
        }
        Action::DeleteTab => {
            match deserialize_payload::<DeleteTabPayload>(&req.payload) {
                Ok(payload) => handlers::handle_delete_tab(req.id, ctx, payload).await,
                Err(e) => Response {
                    id: req.id,
                    status: "ERROR".to_string(),
                    data: None,
                    error: Some(format!("Invalid payload: {}", e)),
                },
            }
        }
        Action::ListTabs => {
            match deserialize_payload::<ListTabsPayload>(&req.payload) {
                Ok(payload) => handlers::handle_list_tabs(req.id, ctx, payload).await,
                Err(e) => Response {
                    id: req.id,
                    status: "ERROR".to_string(),
                    data: None,
                    error: Some(format!("Invalid payload: {}", e)),
                },
            }
        }
    }
}

fn deserialize_payload<T: serde::de::DeserializeOwned>(
    value: &serde_json::Value,
) -> Result<T> {
    Ok(serde_json::from_value(value.clone())?)
}

