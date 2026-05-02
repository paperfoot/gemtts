//! Robustness tests: verify recovery from bad state.
//!
//! These tests ensure discovery and diagnostic commands work even when
//! configuration is malformed, and that enforced constraints match agent-info.

use assert_cmd::Command;

fn bin() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

fn config_dir_in(tmp: &std::path::Path) -> std::path::PathBuf {
    tmp.join(".config").join(env!("CARGO_PKG_NAME"))
}

// ── Malformed config resilience ────────────────────────────────────────────

/// agent-info must work even with a broken config file.
#[test]
fn agent_info_works_with_malformed_config() {
    let tmp = tempfile::tempdir().unwrap();
    let config_dir = config_dir_in(tmp.path());
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), "{{invalid toml").unwrap();

    bin()
        .env("HOME", tmp.path())
        .arg("agent-info")
        .assert()
        .code(0);
}

/// config path must work even with a broken config file.
#[test]
fn config_path_works_with_malformed_config() {
    let tmp = tempfile::tempdir().unwrap();
    let config_dir = config_dir_in(tmp.path());
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), "{{invalid toml").unwrap();

    bin()
        .env("HOME", tmp.path())
        .args(["config", "path"])
        .assert()
        .code(0);
}

/// config show should fail gracefully with exit 2 on malformed config.
#[test]
fn config_show_fails_with_malformed_config() {
    let tmp = tempfile::tempdir().unwrap();
    let config_dir = config_dir_in(tmp.path());
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), "{{invalid toml").unwrap();

    bin()
        .env("HOME", tmp.path())
        .args(["config", "show"])
        .assert()
        .code(2);
}

// ── Constraint enforcement ─────────────────────────────────────────────────

/// Invalid --format value should be rejected by clap (exit 3).
#[test]
fn invalid_format_rejected() {
    bin()
        .args(["speak", "World", "--format", "nonsense"])
        .assert()
        .code(3);
}

/// script command works without --quiet even when HOME is unusual.
#[test]
fn script_works_with_temp_home() {
    let tmp = tempfile::tempdir().unwrap();
    bin()
        .env("HOME", tmp.path())
        .args(["script", "World"])
        .assert()
        .code(0);
}
