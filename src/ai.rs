//! AI query module for RustCast.
//!
//! Sends user queries to a configurable AI provider (e.g. OpenRouter)
//! and returns the response text.

use crate::config::AiConfig;
use log::error;

/// Sends a query to the configured AI provider and returns the response.
///
/// This function is blocking and should be called from within a
/// `tokio::task::spawn_blocking` context.
pub fn query_ai(config: &AiConfig, query: &str) -> String {
    if config.api_key.is_empty() {
        return "Error: AI api_key is not set in config.toml".to_string();
    }

    let body = serde_json::json!({
        "model": config.model,
        "messages": [
            {
                "role": "user",
                "content": query
            }
        ]
    });

    let response = minreq::post(&config.provider_url)
        .with_header("Authorization", format!("Bearer {}", config.api_key))
        .with_header("Content-Type", "application/json")
        .with_header("HTTP-Referer", "https://github.com/notime2/rustcast")
        .with_header("X-Title", "RustCast")
        .with_body(body.to_string())
        .with_timeout(30)
        .send();

    match response {
        Ok(resp) => {
            let json: serde_json::Value =
                match serde_json::from_str(resp.as_str().unwrap_or("")) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("AI response parse error: {e}");
                        return format!("Error parsing response: {e}");
                    }
                };

            json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("No response from AI")
                .to_string()
        }
        Err(e) => {
            error!("AI request error: {e}");
            format!("Error: {e}")
        }
    }
}
