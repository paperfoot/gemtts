use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::AudioFormat;
use crate::config;
use crate::error::AppError;
use crate::prompt::SpeakerVoice;

pub const STANDARD_INPUT_USD_PER_MILLION: f64 = 1.0;
pub const STANDARD_AUDIO_OUTPUT_USD_PER_MILLION: f64 = 20.0;
pub const AUDIO_TOKENS_PER_SECOND: f64 = 25.0;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiUsageMetadata {
    pub prompt_token_count: Option<u64>,
    pub candidates_token_count: Option<u64>,
    pub total_token_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageEstimate {
    pub input_tokens: u64,
    pub input_tokens_source: String,
    pub audio_output_tokens: u64,
    pub audio_output_tokens_source: String,
    pub total_billable_tokens: u64,
    pub audio_seconds: f64,
    pub input_cost_usd: f64,
    pub audio_output_cost_usd: f64,
    pub total_cost_usd: f64,
    pub pricing: PricingSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingSnapshot {
    pub model_family: String,
    pub tier: String,
    pub input_usd_per_1m_text_tokens: f64,
    pub audio_output_usd_per_1m_audio_tokens: f64,
    pub audio_tokens_per_second: f64,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub created_unix_seconds: u64,
    pub command: String,
    pub model: String,
    pub voice: String,
    pub speakers: Vec<SpeakerVoice>,
    pub output_path: String,
    pub output_format: AudioFormat,
    pub prompt_chars: usize,
    pub transcript_chars: usize,
    pub api_usage: ApiUsageMetadata,
    pub estimate: UsageEstimate,
}

#[derive(Debug, Clone, Serialize)]
pub struct UsageSummary {
    pub records: usize,
    pub input_tokens: u64,
    pub audio_output_tokens: u64,
    pub total_billable_tokens: u64,
    pub audio_seconds: f64,
    pub input_cost_usd: f64,
    pub audio_output_cost_usd: f64,
    pub total_cost_usd: f64,
    pub path: String,
    pub pricing: PricingSnapshot,
}

pub fn usage_path() -> PathBuf {
    config::state_dir().join("usage.jsonl")
}

pub fn api_usage_from_response(json: &Value) -> ApiUsageMetadata {
    let usage = json
        .get("usageMetadata")
        .or_else(|| json.get("usage_metadata"));
    let Some(usage) = usage else {
        return ApiUsageMetadata::default();
    };

    ApiUsageMetadata {
        prompt_token_count: read_u64(usage, "promptTokenCount", "prompt_token_count"),
        candidates_token_count: read_u64(usage, "candidatesTokenCount", "candidates_token_count"),
        total_token_count: read_u64(usage, "totalTokenCount", "total_token_count"),
    }
}

pub fn estimate(
    pcm_bytes: usize,
    sample_rate: u32,
    channels: u16,
    prompt_chars: usize,
    api_usage: &ApiUsageMetadata,
) -> UsageEstimate {
    let audio_seconds = audio_seconds_from_pcm(pcm_bytes, sample_rate, channels);
    let duration_audio_tokens = (audio_seconds * AUDIO_TOKENS_PER_SECOND).ceil() as u64;
    let (audio_output_tokens, audio_output_tokens_source) =
        if let Some(tokens) = api_usage.candidates_token_count {
            (tokens, "api_usage_metadata".to_string())
        } else {
            (
                duration_audio_tokens,
                "estimated_from_pcm_duration".to_string(),
            )
        };
    let (input_tokens, input_tokens_source) = if let Some(tokens) = api_usage.prompt_token_count {
        (tokens, "api_usage_metadata".to_string())
    } else {
        (
            estimate_text_tokens(prompt_chars),
            "estimated_from_chars".to_string(),
        )
    };
    let input_cost_usd = cost_usd(input_tokens, STANDARD_INPUT_USD_PER_MILLION);
    let audio_output_cost_usd =
        cost_usd(audio_output_tokens, STANDARD_AUDIO_OUTPUT_USD_PER_MILLION);

    UsageEstimate {
        input_tokens,
        input_tokens_source,
        audio_output_tokens,
        audio_output_tokens_source,
        total_billable_tokens: input_tokens + audio_output_tokens,
        audio_seconds,
        input_cost_usd,
        audio_output_cost_usd,
        total_cost_usd: input_cost_usd + audio_output_cost_usd,
        pricing: pricing_snapshot(),
    }
}

pub fn append_record(record: &UsageRecord) -> Result<(), AppError> {
    let path = usage_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    serde_json::to_writer(&mut file, record).map_err(|e| AppError::Config(e.to_string()))?;
    file.write_all(b"\n")?;
    Ok(())
}

pub fn load_records() -> Result<Vec<UsageRecord>, AppError> {
    let path = usage_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path)?;
    let mut records = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let mut record = serde_json::from_str::<UsageRecord>(line).map_err(|e| {
            AppError::Config(format!(
                "usage ledger line {} is invalid JSON: {e}",
                line_no + 1
            ))
        })?;
        normalize_record(&mut record);
        records.push(record);
    }
    Ok(records)
}

pub fn summarize(records: &[UsageRecord]) -> UsageSummary {
    let input_tokens = records.iter().map(|r| r.estimate.input_tokens).sum();
    let audio_output_tokens = records.iter().map(|r| r.estimate.audio_output_tokens).sum();
    let total_billable_tokens = records
        .iter()
        .map(|r| r.estimate.total_billable_tokens)
        .sum();
    let audio_seconds = records.iter().map(|r| r.estimate.audio_seconds).sum();
    let input_cost_usd = records.iter().map(|r| r.estimate.input_cost_usd).sum();
    let audio_output_cost_usd = records
        .iter()
        .map(|r| r.estimate.audio_output_cost_usd)
        .sum();
    let total_cost_usd = records.iter().map(|r| r.estimate.total_cost_usd).sum();

    UsageSummary {
        records: records.len(),
        input_tokens,
        audio_output_tokens,
        total_billable_tokens,
        audio_seconds,
        input_cost_usd,
        audio_output_cost_usd,
        total_cost_usd,
        path: usage_path().display().to_string(),
        pricing: pricing_snapshot(),
    }
}

pub fn now_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn pricing_snapshot() -> PricingSnapshot {
    PricingSnapshot {
        model_family: "gemini-3.1-flash-tts-preview".into(),
        tier: "standard_paid_estimate".into(),
        input_usd_per_1m_text_tokens: STANDARD_INPUT_USD_PER_MILLION,
        audio_output_usd_per_1m_audio_tokens: STANDARD_AUDIO_OUTPUT_USD_PER_MILLION,
        audio_tokens_per_second: AUDIO_TOKENS_PER_SECOND,
        note: "Estimate uses Google Gemini API standard paid TTS pricing; free-tier usage may be charged at $0.".into(),
    }
}

fn read_u64(value: &Value, camel: &str, snake: &str) -> Option<u64> {
    value
        .get(camel)
        .or_else(|| value.get(snake))
        .and_then(Value::as_u64)
}

fn audio_seconds_from_pcm(pcm_bytes: usize, sample_rate: u32, channels: u16) -> f64 {
    let bytes_per_second = sample_rate as f64 * channels as f64 * 2.0;
    if bytes_per_second <= 0.0 {
        return 0.0;
    }
    pcm_bytes as f64 / bytes_per_second
}

fn estimate_text_tokens(chars: usize) -> u64 {
    ((chars as f64) / 4.0).ceil().max(1.0) as u64
}

fn cost_usd(tokens: u64, usd_per_million: f64) -> f64 {
    tokens as f64 * usd_per_million / 1_000_000.0
}

fn normalize_record(record: &mut UsageRecord) {
    if let Some(tokens) = record.api_usage.prompt_token_count {
        record.estimate.input_tokens = tokens;
        record.estimate.input_tokens_source = "api_usage_metadata".into();
    }
    if let Some(tokens) = record.api_usage.candidates_token_count {
        record.estimate.audio_output_tokens = tokens;
        record.estimate.audio_output_tokens_source = "api_usage_metadata".into();
    }
    record.estimate.input_cost_usd =
        cost_usd(record.estimate.input_tokens, STANDARD_INPUT_USD_PER_MILLION);
    record.estimate.audio_output_cost_usd = cost_usd(
        record.estimate.audio_output_tokens,
        STANDARD_AUDIO_OUTPUT_USD_PER_MILLION,
    );
    record.estimate.total_billable_tokens =
        record.estimate.input_tokens + record.estimate.audio_output_tokens;
    record.estimate.total_cost_usd =
        record.estimate.input_cost_usd + record.estimate.audio_output_cost_usd;
}

#[cfg(test)]
mod tests {
    use super::{api_usage_from_response, estimate, normalize_record};

    #[test]
    fn extracts_camel_case_usage_metadata() {
        let json = serde_json::json!({
            "usageMetadata": {
                "promptTokenCount": 12,
                "candidatesTokenCount": 25,
                "totalTokenCount": 37
            }
        });
        let usage = api_usage_from_response(&json);
        assert_eq!(usage.prompt_token_count, Some(12));
        assert_eq!(usage.candidates_token_count, Some(25));
        assert_eq!(usage.total_token_count, Some(37));
    }

    #[test]
    fn estimates_audio_tokens_from_pcm_duration() {
        let usage = Default::default();
        let estimate = estimate(48_000, 24_000, 1, 40, &usage);
        assert_eq!(estimate.audio_seconds, 1.0);
        assert_eq!(estimate.audio_output_tokens, 25);
        assert_eq!(estimate.input_tokens, 10);
    }

    #[test]
    fn prefers_api_output_tokens_when_available() {
        let usage = super::ApiUsageMetadata {
            prompt_token_count: Some(3),
            candidates_token_count: Some(99),
            total_token_count: Some(102),
        };
        let estimate = estimate(48_000, 24_000, 1, 40, &usage);
        assert_eq!(estimate.audio_output_tokens, 99);
        assert_eq!(estimate.audio_output_tokens_source, "api_usage_metadata");
    }

    #[test]
    fn normalizes_older_records_to_api_counts() {
        let usage = super::ApiUsageMetadata {
            prompt_token_count: Some(3),
            candidates_token_count: Some(99),
            total_token_count: Some(102),
        };
        let mut record = super::UsageRecord {
            created_unix_seconds: 1,
            command: "speak".into(),
            model: "gemini-3.1-flash-tts-preview".into(),
            voice: "Kore".into(),
            speakers: Vec::new(),
            output_path: "out.wav".into(),
            output_format: crate::cli::AudioFormat::Wav,
            prompt_chars: 40,
            transcript_chars: 40,
            api_usage: usage,
            estimate: estimate(48_000, 24_000, 1, 40, &Default::default()),
        };

        normalize_record(&mut record);

        assert_eq!(record.estimate.input_tokens, 3);
        assert_eq!(record.estimate.audio_output_tokens, 99);
        assert_eq!(
            record.estimate.total_cost_usd,
            record.estimate.input_cost_usd + record.estimate.audio_output_cost_usd
        );
    }
}
