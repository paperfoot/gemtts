use serde::Serialize;

use crate::config::{self, AppConfig};
use crate::error::AppError;
use crate::output::{self, Ctx};

pub fn show(ctx: Ctx, config: &AppConfig) -> Result<(), AppError> {
    let public = config::public_config(config);
    output::print_success_or(ctx, &public, |c| {
        println!("{}", serde_json::to_string_pretty(c).unwrap());
    });
    Ok(())
}

#[derive(Serialize)]
struct ConfigPath {
    path: String,
    exists: bool,
}

pub fn path(ctx: Ctx) -> Result<(), AppError> {
    let p = config::config_path();
    let result = ConfigPath {
        path: p.display().to_string(),
        exists: p.exists(),
    };
    output::print_success_or(ctx, &result, |r| {
        println!("{}", r.path);
        if !r.exists {
            use owo_colors::OwoColorize;
            println!("  {}", "(file does not exist, using defaults)".dimmed());
        }
    });
    Ok(())
}

#[derive(Serialize)]
struct InitResult {
    path: String,
    created: bool,
}

pub fn init(ctx: Ctx) -> Result<(), AppError> {
    let (path, created) = config::init_if_missing()?;
    let result = InitResult {
        path: path.display().to_string(),
        created,
    };
    output::print_success_or(ctx, &result, |r| {
        if r.created {
            println!("Created {}", r.path);
        } else {
            println!("Already exists: {}", r.path);
        }
    });
    Ok(())
}

#[derive(Serialize)]
struct SetResult {
    key: String,
    value: serde_json::Value,
    path: String,
}

pub fn set(ctx: Ctx, key: String, value: String) -> Result<(), AppError> {
    let config = config::set_value(&key, &value)?;
    let shown = config::get_value(&config, &key)?;
    let result = SetResult {
        key,
        value: shown,
        path: config::config_path().display().to_string(),
    };
    output::print_success_or(ctx, &result, |r| {
        println!("Set {} in {}", r.key, r.path);
    });
    Ok(())
}

#[derive(Serialize)]
struct GetResult {
    key: String,
    value: serde_json::Value,
}

pub fn get(ctx: Ctx, config: &AppConfig, key: String) -> Result<(), AppError> {
    let value = config::get_value(config, &key)?;
    let result = GetResult { key, value };
    output::print_success_or(ctx, &result, |r| {
        println!("{}", serde_json::to_string_pretty(&r.value).unwrap());
    });
    Ok(())
}
