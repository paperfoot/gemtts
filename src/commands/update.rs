use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::time::Duration;

use crate::error::AppError;
use crate::output::{self, Ctx};

#[derive(Serialize)]
struct UpdateResult {
    current_version: String,
    latest_version: Option<String>,
    status: String,
    install_source: String,
    update_command: String,
    message: String,
}

#[derive(Deserialize)]
struct CratesResponse {
    #[serde(rename = "crate")]
    krate: CrateInfo,
}

#[derive(Deserialize)]
struct CrateInfo {
    max_version: String,
}

pub fn run(ctx: Ctx, check: bool, _config: &crate::config::AppConfig) -> Result<(), AppError> {
    let current = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let (install_source, update_command) = detect_install_source(name);
    let latest = latest_crates_version(name)?;
    let version_order = compare_versions(current, &latest);
    let status = match (version_order, check) {
        (Ordering::Equal, _) => "up_to_date",
        (Ordering::Less, true) => "update_available",
        (Ordering::Less, false) => "manual_update_required",
        (Ordering::Greater, _) => "ahead_of_registry",
    };
    let message = if check {
        match version_order {
            Ordering::Equal => format!("{name} is up to date."),
            Ordering::Less => format!("Update available. Run: {update_command}"),
            Ordering::Greater => format!(
                "{name} is newer than the latest crates.io version; publish before packaging."
            ),
        }
    } else {
        format!("Use the package manager that installed this binary: {update_command}")
    };

    let result = UpdateResult {
        current_version: current.into(),
        latest_version: Some(latest),
        status: status.into(),
        install_source: install_source.into(),
        update_command,
        message,
    };

    output::print_success_or(ctx, &result, |r| {
        println!("{}", r.message);
        if !check && r.status == "manual_update_required" {
            println!(
                "This CLI is distributed through crates.io and Homebrew, not GitHub release assets."
            );
        }
    });
    Ok(())
}

fn compare_versions(current: &str, latest: &str) -> Ordering {
    let current_parts = parse_version(current);
    let latest_parts = parse_version(latest);
    current_parts.cmp(&latest_parts)
}

fn parse_version(version: &str) -> Vec<u64> {
    version
        .trim_start_matches('v')
        .split('.')
        .map(|part| {
            part.chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or(0)
        })
        .collect()
}

fn latest_crates_version(name: &str) -> Result<String, AppError> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let url = format!("https://crates.io/api/v1/crates/{name}");
    let response = client
        .get(url)
        .header(
            "User-Agent",
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        )
        .send()?;
    let status = response.status();
    let text = response.text()?;
    if !status.is_success() {
        return Err(AppError::Transient(format!(
            "crates.io version check failed ({status}): {}",
            text.trim()
        )));
    }
    let parsed: CratesResponse = serde_json::from_str(&text)
        .map_err(|e| AppError::Transient(format!("crates.io returned invalid JSON: {e}")))?;
    Ok(parsed.krate.max_version)
}

fn detect_install_source(name: &str) -> (&'static str, String) {
    let exe = std::env::current_exe()
        .ok()
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    if exe.contains("/Cellar/") || exe.contains("homebrew") {
        ("homebrew", format!("brew upgrade paperfoot/tap/{name}"))
    } else if exe.contains(".cargo/bin") {
        ("cargo", format!("cargo install {name} --force"))
    } else {
        ("local-build", format!("cargo install {name} --force"))
    }
}

#[cfg(test)]
mod tests {
    use super::compare_versions;
    use std::cmp::Ordering;

    #[test]
    fn compares_semver_like_versions() {
        assert_eq!(compare_versions("0.1.1", "0.1.0"), Ordering::Greater);
        assert_eq!(compare_versions("0.1.0", "0.1.1"), Ordering::Less);
        assert_eq!(compare_versions("v0.1.1", "0.1.1"), Ordering::Equal);
    }
}
