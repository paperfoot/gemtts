use serde::Serialize;

use crate::config::{self, AppConfig};
use crate::error::AppError;
use crate::output::{self, Ctx};

#[derive(Serialize)]
struct AuthResult {
    configured: bool,
    source: String,
    api_key: String,
    path: String,
}

pub fn set(ctx: Ctx, api_key: String) -> Result<(), AppError> {
    validate_key(&api_key)?;
    let config = config::set_value("keys.api_key", &api_key)?;
    let result = auth_result(&config);
    output::print_success_or(ctx, &result, |r| {
        println!("Saved Gemini API key to {}", r.path);
    });
    Ok(())
}

pub fn import_env(ctx: Ctx) -> Result<(), AppError> {
    let Some((source, key)) = config::api_key_from_env() else {
        return Err(AppError::Config(
            "no key found in GEMINI_API_KEY, GOOGLE_API_KEY, or GOOGLE_AI_API_KEY".into(),
        ));
    };
    validate_key(&key)?;
    let config = config::set_value("keys.api_key", &key)?;
    let mut result = auth_result(&config);
    result.source = source.into();
    output::print_success_or(ctx, &result, |r| {
        println!("Imported {} into {}", r.source, r.path);
    });
    Ok(())
}

pub fn status(ctx: Ctx, config: &AppConfig) -> Result<(), AppError> {
    let result = auth_result(config);
    output::print_success_or(ctx, &result, |r| {
        if r.configured {
            println!("Gemini API key configured: {} ({})", r.api_key, r.source);
        } else {
            println!("Gemini API key not configured");
        }
    });
    Ok(())
}

fn auth_result(config: &AppConfig) -> AuthResult {
    let key = config::api_key(config);
    AuthResult {
        configured: key.is_some(),
        source: key
            .as_ref()
            .map(|(source, _)| source.clone())
            .unwrap_or_else(|| "none".into()),
        api_key: key
            .as_ref()
            .map(|(_, value)| config::mask_secret(value))
            .unwrap_or_else(|| "not_set".into()),
        path: config::config_path().display().to_string(),
    }
}

fn validate_key(api_key: &str) -> Result<(), AppError> {
    let trimmed = api_key.trim();
    if trimmed.len() < 20 {
        return Err(AppError::InvalidInput(
            "Gemini API key looks too short".into(),
        ));
    }
    if trimmed.contains(char::is_whitespace) {
        return Err(AppError::InvalidInput(
            "Gemini API key must not contain whitespace".into(),
        ));
    }
    Ok(())
}
