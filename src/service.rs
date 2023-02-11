use crate::api::{github_proto, github_signature};
use crate::config;
use crate::errors;
use crate::github;

use axum::{extract::TypedHeader, Json};
use log::{debug, info};
use serde_json::json;

pub async fn check() -> &'static str {
    "ok"
}

pub async fn github_event(
    TypedHeader(event_type): TypedHeader<github_proto::XGitHubEvent>,
    TypedHeader(signature): TypedHeader<github_proto::XHubSignature>,
    body: String,
) -> Result<Json<String>, errors::RequestErrorResult> {
    info!("Received GitHub webhook, type={}", event_type.0);

    // Check X-Hub-Signature
    github_signature::check_signature(
        &config::CONFIG.github.webhook_secret.clone(),
        &signature.0,
        &body,
    )?;

    debug!("body={}", body);

    // Handle the event
    Ok(Json(github::handle_event_body(
        &event_type.0.as_ref(),
        &body,
    ).await?))
}

pub async fn gitlab_event(Json(event): Json<serde_json::Value>) -> Json<serde_json::Value> {
    info!("{:?}", event);
    Json(json!({"hello":"hi"}))
}
