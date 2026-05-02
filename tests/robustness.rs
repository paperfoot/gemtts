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

#[test]
fn invalid_voice_rejected_before_api_key_lookup() {
    let tmp = tempfile::tempdir().unwrap();
    bin()
        .env("HOME", tmp.path())
        .env_remove("GEMINI_API_KEY")
        .env_remove("GOOGLE_API_KEY")
        .env_remove("GOOGLE_AI_API_KEY")
        .args(["speak", "World", "--voice", "ItalianVoice"])
        .assert()
        .code(3);
}

#[test]
fn single_speaker_mapping_is_rejected() {
    bin()
        .args(["script", "Host: Hello", "--speaker", "Host=Kore"])
        .assert()
        .code(3);
}

#[test]
fn speaker_voice_names_are_canonicalized() {
    bin()
        .args([
            "script",
            "Host: Hello\nGuest: Ciao",
            "--speaker",
            "Host=kore",
            "--speaker",
            "Guest=puck",
            "--json",
        ])
        .assert()
        .code(0)
        .stdout(predicates::str::contains("\"voice\": \"Kore\""))
        .stdout(predicates::str::contains("\"voice\": \"Puck\""));
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

#[test]
fn env_api_key_overrides_config_key() {
    let tmp = tempfile::tempdir().unwrap();
    let config_dir = config_dir_in(tmp.path());
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(
        config_dir.join("config.toml"),
        "[keys]\napi_key = \"config-key-123456\"\n",
    )
    .unwrap();

    bin()
        .env("HOME", tmp.path())
        .env("GEMINI_API_KEY", "env-key-123456")
        .args(["config", "show", "--json"])
        .assert()
        .code(0)
        .stdout(predicates::str::contains(
            "\"api_key_source\": \"GEMINI_API_KEY\"",
        ));
}

#[test]
fn legacy_gemini_tts_cli_config_is_loaded_after_rename() {
    let tmp = tempfile::tempdir().unwrap();
    let legacy_dir = tmp.path().join(".config").join("gemini-tts-cli");
    std::fs::create_dir_all(&legacy_dir).unwrap();
    std::fs::write(
        legacy_dir.join("config.toml"),
        "[keys]\napi_key = \"legacy-key-123456\"\n",
    )
    .unwrap();

    bin()
        .env("HOME", tmp.path())
        .env_remove("GEMINI_API_KEY")
        .env_remove("GOOGLE_API_KEY")
        .env_remove("GOOGLE_AI_API_KEY")
        .args(["config", "show", "--json"])
        .assert()
        .code(0)
        .stdout(predicates::str::contains("\"api_key_source\": \"config\""))
        .stdout(predicates::str::contains("lega...3456"));
}
