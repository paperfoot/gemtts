/// Configuration loading with 3-tier precedence:
///   1. Compiled defaults
///   2. TOML config file (~/.config/gemini-tts-cli/config.toml)
///   3. Environment variables (GEMINI_TTS__* nested config, plus GEMINI_API_KEY)
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::catalog;
use crate::cli::AudioFormat;
use crate::error::AppError;

pub const ENV_PREFIX: &str = "GEMINI_TTS_";
const API_KEY_ENV_VARS: [&str; 3] = ["GEMINI_API_KEY", "GOOGLE_API_KEY", "GOOGLE_AI_API_KEY"];

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub keys: KeysConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
    #[serde(default)]
    pub prompt: PromptConfig,
    #[serde(default)]
    pub update: UpdateConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeysConfig {
    #[serde(default)]
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_voice")]
    pub voice: String,
    #[serde(default = "default_audio_format")]
    pub audio_format: AudioFormat,
    #[serde(default = "default_sample_rate")]
    pub sample_rate: u32,
    #[serde(default = "default_channels")]
    pub channels: u16,
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptConfig {
    #[serde(default = "default_profile")]
    pub profile: String,
    #[serde(default = "default_scene")]
    pub scene: String,
    #[serde(default = "default_style")]
    pub style: String,
    #[serde(default = "default_pace")]
    pub pace: String,
    #[serde(default = "default_accent")]
    pub accent: String,
    #[serde(default = "default_language")]
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_update_owner")]
    pub owner: String,
    #[serde(default = "default_update_repo")]
    pub repo: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicConfig {
    pub keys: PublicKeysConfig,
    pub defaults: DefaultsConfig,
    pub prompt: PromptConfig,
    pub update: UpdateConfig,
    pub config_path: String,
    pub env_prefix: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicKeysConfig {
    pub api_key: String,
    pub api_key_source: String,
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            model: default_model(),
            voice: default_voice(),
            audio_format: default_audio_format(),
            sample_rate: default_sample_rate(),
            channels: default_channels(),
            timeout_seconds: default_timeout_seconds(),
        }
    }
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            profile: default_profile(),
            scene: default_scene(),
            style: default_style(),
            pace: default_pace(),
            accent: default_accent(),
            language: default_language(),
        }
    }
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            owner: default_update_owner(),
            repo: default_update_repo(),
        }
    }
}

fn default_model() -> String {
    "gemini-3.1-flash-tts-preview".into()
}

fn default_voice() -> String {
    "Kore".into()
}

fn default_audio_format() -> AudioFormat {
    AudioFormat::Wav
}

fn default_sample_rate() -> u32 {
    24_000
}

fn default_channels() -> u16 {
    1
}

fn default_timeout_seconds() -> u64 {
    90
}

fn default_profile() -> String {
    "Clear, useful narrator".into()
}

fn default_scene() -> String {
    "A clean studio recording for direct listener comprehension.".into()
}

fn default_style() -> String {
    "Natural, expressive, and precise. Do not overact.".into()
}

fn default_pace() -> String {
    "Medium pace with deliberate pauses where tags or punctuation indicate them.".into()
}

fn default_accent() -> String {
    "Neutral unless the transcript or command specifies otherwise.".into()
}

fn default_language() -> String {
    "Use the transcript language unless specified otherwise.".into()
}

fn default_true() -> bool {
    true
}

fn default_update_owner() -> String {
    "paperfoot".into()
}

fn default_update_repo() -> String {
    "gemini-tts-cli".into()
}

pub fn config_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("GEMINI_TTS_CONFIG_DIR") {
        return PathBuf::from(dir);
    }
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".config")
        .join(env!("CARGO_PKG_NAME"))
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn state_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("GEMINI_TTS_STATE_DIR") {
        return PathBuf::from(dir);
    }
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".local")
        .join("share")
        .join(env!("CARGO_PKG_NAME"))
}

pub fn cache_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("GEMINI_TTS_CACHE_DIR") {
        return PathBuf::from(dir);
    }
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".cache")
        .join(env!("CARGO_PKG_NAME"))
}

pub fn load() -> Result<AppConfig, AppError> {
    use figment::Figment;
    use figment::providers::{Env, Format as _, Serialized, Toml};

    Figment::from(Serialized::defaults(AppConfig::default()))
        .merge(Toml::file(config_path()))
        .merge(Env::prefixed(ENV_PREFIX).split("__"))
        .extract()
        .map_err(|e| AppError::Config(e.to_string()))
}

pub fn write(config: &AppConfig) -> Result<PathBuf, AppError> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let text = toml::to_string_pretty(config).map_err(|e| AppError::Config(e.to_string()))?;
    std::fs::write(&path, text)?;
    set_secret_permissions(&path)?;
    Ok(path)
}

#[cfg(unix)]
fn set_secret_permissions(path: &std::path::Path) -> Result<(), AppError> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path)?.permissions();
    permissions.set_mode(0o600);
    std::fs::set_permissions(path, permissions)?;
    Ok(())
}

#[cfg(not(unix))]
fn set_secret_permissions(_path: &std::path::Path) -> Result<(), AppError> {
    Ok(())
}

pub fn api_key_from_env() -> Option<(&'static str, String)> {
    for name in API_KEY_ENV_VARS {
        if let Ok(value) = std::env::var(name) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some((name, trimmed.to_string()));
            }
        }
    }
    None
}

pub fn api_key(config: &AppConfig) -> Option<(String, String)> {
    if let Some((source, key)) = api_key_from_env() {
        return Some((source.into(), key));
    }
    if !config.keys.api_key.trim().is_empty() {
        return Some(("config".into(), config.keys.api_key.trim().to_string()));
    }
    None
}

pub fn require_api_key(config: &AppConfig) -> Result<(String, String), AppError> {
    api_key(config).ok_or_else(|| {
        AppError::Config(format!(
            "missing Gemini API key. Set GEMINI_API_KEY or run {} auth set --api-key <key>",
            env!("CARGO_PKG_NAME")
        ))
    })
}

pub fn mask_secret(value: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        return "not_set".into();
    }
    let chars: Vec<char> = value.chars().collect();
    if chars.len() <= 8 {
        return "*".repeat(chars.len());
    }
    let start: String = chars.iter().take(4).collect();
    let end: String = chars
        .iter()
        .rev()
        .take(4)
        .copied()
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect();
    format!("{start}...{end}")
}

pub fn public_config(config: &AppConfig) -> PublicConfig {
    let (api_key_source, key) = api_key(config).unwrap_or_else(|| ("none".into(), String::new()));
    PublicConfig {
        keys: PublicKeysConfig {
            api_key: mask_secret(&key),
            api_key_source,
        },
        defaults: config.defaults.clone(),
        prompt: config.prompt.clone(),
        update: config.update.clone(),
        config_path: config_path().display().to_string(),
        env_prefix: ENV_PREFIX,
    }
}

pub fn init_if_missing() -> Result<(PathBuf, bool), AppError> {
    let path = config_path();
    if path.exists() {
        return Ok((path, false));
    }
    let config = AppConfig::default();
    let path = write(&config)?;
    Ok((path, true))
}

pub fn set_value(key: &str, value: &str) -> Result<AppConfig, AppError> {
    let mut config = load().unwrap_or_default();
    let value = value.trim().to_string();
    match key {
        "keys.api_key" | "api_key" => config.keys.api_key = value,
        "defaults.model" | "model" => config.defaults.model = value,
        "defaults.voice" | "voice" => {
            let Some(voice) = catalog::canonical_voice_name(&value) else {
                return Err(AppError::InvalidInput(format!(
                    "unsupported Gemini TTS voice {value:?}. Valid voices: {}",
                    catalog::voice_names().join(", ")
                )));
            };
            config.defaults.voice = voice.to_string();
        }
        "defaults.audio_format" | "audio_format" | "format" => {
            config.defaults.audio_format = parse_audio_format(&value)?
        }
        "defaults.sample_rate" | "sample_rate" => {
            config.defaults.sample_rate = value
                .parse()
                .map_err(|_| AppError::InvalidInput("sample_rate must be an integer".into()))?
        }
        "defaults.channels" | "channels" => {
            config.defaults.channels = value
                .parse()
                .map_err(|_| AppError::InvalidInput("channels must be an integer".into()))?
        }
        "defaults.timeout_seconds" | "timeout_seconds" => {
            config.defaults.timeout_seconds = value
                .parse()
                .map_err(|_| AppError::InvalidInput("timeout_seconds must be an integer".into()))?
        }
        "prompt.profile" | "profile" => config.prompt.profile = value,
        "prompt.scene" | "scene" => config.prompt.scene = value,
        "prompt.style" | "style" => config.prompt.style = value,
        "prompt.pace" | "pace" => config.prompt.pace = value,
        "prompt.accent" | "accent" => config.prompt.accent = value,
        "prompt.language" | "language" => config.prompt.language = value,
        "update.enabled" => {
            config.update.enabled = value.parse().map_err(|_| {
                AppError::InvalidInput("update.enabled must be true or false".into())
            })?
        }
        "update.owner" => config.update.owner = value,
        "update.repo" => config.update.repo = value,
        other => {
            return Err(AppError::InvalidInput(format!(
                "unsupported config key: {other}"
            )));
        }
    }
    write(&config)?;
    Ok(config)
}

pub fn get_value(config: &AppConfig, key: &str) -> Result<serde_json::Value, AppError> {
    let value = match key {
        "keys.api_key" | "api_key" => serde_json::json!(mask_secret(
            &api_key(config).map(|(_, k)| k).unwrap_or_default()
        )),
        "defaults.model" | "model" => serde_json::json!(config.defaults.model),
        "defaults.voice" | "voice" => serde_json::json!(config.defaults.voice),
        "defaults.audio_format" | "audio_format" | "format" => {
            serde_json::json!(config.defaults.audio_format)
        }
        "defaults.sample_rate" | "sample_rate" => serde_json::json!(config.defaults.sample_rate),
        "defaults.channels" | "channels" => serde_json::json!(config.defaults.channels),
        "defaults.timeout_seconds" | "timeout_seconds" => {
            serde_json::json!(config.defaults.timeout_seconds)
        }
        "prompt.profile" | "profile" => serde_json::json!(config.prompt.profile),
        "prompt.scene" | "scene" => serde_json::json!(config.prompt.scene),
        "prompt.style" | "style" => serde_json::json!(config.prompt.style),
        "prompt.pace" | "pace" => serde_json::json!(config.prompt.pace),
        "prompt.accent" | "accent" => serde_json::json!(config.prompt.accent),
        "prompt.language" | "language" => serde_json::json!(config.prompt.language),
        "update.enabled" => serde_json::json!(config.update.enabled),
        "update.owner" => serde_json::json!(config.update.owner),
        "update.repo" => serde_json::json!(config.update.repo),
        other => {
            return Err(AppError::InvalidInput(format!(
                "unsupported config key: {other}"
            )));
        }
    };
    Ok(value)
}

fn parse_audio_format(value: &str) -> Result<AudioFormat, AppError> {
    match value.to_ascii_lowercase().as_str() {
        "auto" => Ok(AudioFormat::Auto),
        "wav" => Ok(AudioFormat::Wav),
        "pcm" => Ok(AudioFormat::Pcm),
        "mp3" => Ok(AudioFormat::Mp3),
        "m4a" | "aac" => Ok(AudioFormat::M4a),
        "flac" => Ok(AudioFormat::Flac),
        _ => Err(AppError::InvalidInput(format!(
            "unsupported audio format: {value}. Valid: auto, wav, pcm, mp3, m4a, flac"
        ))),
    }
}
