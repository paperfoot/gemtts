use base64::Engine;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

use crate::error::AppError;
use crate::prompt::SpeakerVoice;
use crate::usage::{self, ApiUsageMetadata};

const MAX_AUTOMATIC_RETRY_SECONDS: u64 = 30;

#[derive(Debug, Clone)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub voice: String,
    pub speakers: Vec<SpeakerVoice>,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerateResponse {
    pub pcm: Vec<u8>,
    pub mime_type: String,
    pub model: String,
    pub prompt_chars: usize,
    pub usage: ApiUsageMetadata,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct RateLimitInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_metric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub interpretation: String,
}

pub fn generate(api_key: &str, request: &GenerateRequest) -> Result<GenerateResponse, AppError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(request.timeout_seconds.max(10)))
        .build()?;

    let mut attempt = 0;
    loop {
        match generate_once(&client, api_key, request) {
            Ok(response) => return Ok(response),
            Err(error) if should_retry(&error) && attempt < 2 => {
                let delay = retry_delay(&error, attempt + 1);
                if delay > Duration::from_secs(MAX_AUTOMATIC_RETRY_SECONDS) {
                    return Err(error);
                }
                attempt += 1;
                std::thread::sleep(delay);
            }
            Err(error) => return Err(error),
        }
    }
}

fn generate_once(
    client: &Client,
    api_key: &str,
    request: &GenerateRequest,
) -> Result<GenerateResponse, AppError> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        request.model
    );
    let payload = request_payload(request);
    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?;

    let status = response.status();
    let text = response.text()?;
    if !status.is_success() {
        let message = extract_error_message(&text).unwrap_or_else(|| text.trim().to_string());
        if status.as_u16() == 429 {
            return Err(AppError::RateLimited(message));
        }
        if matches!(status.as_u16(), 400 | 401 | 403) {
            return Err(AppError::Config(format!(
                "Gemini API rejected the request ({status}): {message}"
            )));
        }
        return Err(AppError::Transient(format!(
            "Gemini API returned {status}: {message}"
        )));
    }

    let json: Value = serde_json::from_str(&text)
        .map_err(|e| AppError::Transient(format!("Gemini returned invalid JSON: {e}")))?;
    let (data, mime_type) = extract_audio(&json)?;
    let pcm = base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|e| AppError::Transient(format!("Gemini returned invalid base64 audio: {e}")))?;

    Ok(GenerateResponse {
        pcm,
        mime_type: mime_type.unwrap_or_else(|| "audio/l16; rate=24000; channels=1".into()),
        model: request.model.clone(),
        prompt_chars: request.prompt.chars().count(),
        usage: usage::api_usage_from_response(&json),
    })
}

fn should_retry(error: &AppError) -> bool {
    matches!(
        error,
        AppError::Transient(_) | AppError::RateLimited(_) | AppError::Http(_)
    )
}

fn retry_delay(error: &AppError, attempt: u32) -> Duration {
    if let AppError::RateLimited(message) = error {
        if let Some(delay) = parse_retry_delay(message) {
            return delay + Duration::from_millis(500);
        }
        return Duration::from_secs(2 * attempt as u64);
    }
    Duration::from_millis(400 * attempt as u64)
}

pub fn parse_retry_delay(message: &str) -> Option<Duration> {
    let rest = message.split("Please retry in ").nth(1)?;
    let mut total_seconds = 0.0;
    let mut number = String::new();
    let mut saw_unit = false;

    for ch in rest.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            number.push(ch);
            continue;
        }

        let multiplier = match ch {
            'h' => 3600.0,
            'm' => 60.0,
            's' => 1.0,
            _ if saw_unit => break,
            _ => return None,
        };
        let value = number.parse::<f64>().ok()?;
        if !value.is_finite() || value < 0.0 {
            return None;
        }
        total_seconds += value * multiplier;
        number.clear();
        saw_unit = true;
        if ch == 's' {
            break;
        }
    }

    if !saw_unit || !total_seconds.is_finite() || total_seconds < 0.0 {
        return None;
    }
    Some(Duration::from_millis((total_seconds * 1000.0).ceil() as u64))
}

pub fn rate_limit_info(message: &str) -> RateLimitInfo {
    let mut info = RateLimitInfo {
        retry_after_seconds: parse_retry_delay(message).map(|d| d.as_secs_f64()),
        interpretation: "Rate limit or quota response from Gemini API. This is not by itself proof that prepaid credits are depleted.".into(),
        ..Default::default()
    };

    for line in message.lines() {
        let line = line.trim().trim_start_matches("* ");
        let Some(rest) = line.strip_prefix("Quota exceeded for metric: ") else {
            continue;
        };
        let mut parts = rest.split(',').map(str::trim);
        info.quota_metric = parts.next().map(str::to_string);
        for part in parts {
            if let Some(limit) = part.strip_prefix("limit: ") {
                info.limit = limit.parse::<u64>().ok();
            } else if let Some(model) = part.strip_prefix("model: ") {
                info.model = Some(model.to_string());
            }
        }
    }

    info.quota_kind = info
        .quota_metric
        .as_deref()
        .and_then(quota_kind)
        .map(str::to_string);
    info
}

fn quota_kind(metric: &str) -> Option<&'static str> {
    if metric.contains("tokens") && metric.ends_with("_per_day") {
        Some("tokens_per_day")
    } else if metric.contains("requests") && metric.ends_with("_per_day") {
        Some("requests_per_day")
    } else if metric.contains("requests") && metric.ends_with("_per_minute") {
        Some("requests_per_minute")
    } else if metric.contains("tokens") && metric.ends_with("_per_minute") {
        Some("tokens_per_minute")
    } else {
        None
    }
}

pub fn request_payload(request: &GenerateRequest) -> Value {
    let speech_config = if request.speakers.is_empty() {
        serde_json::json!({
            "voiceConfig": {
                "prebuiltVoiceConfig": {
                    "voiceName": request.voice
                }
            }
        })
    } else {
        let speaker_voice_configs: Vec<Value> = request
            .speakers
            .iter()
            .map(|speaker| {
                serde_json::json!({
                    "speaker": speaker.speaker,
                    "voiceConfig": {
                        "prebuiltVoiceConfig": {
                            "voiceName": speaker.voice
                        }
                    }
                })
            })
            .collect();
        serde_json::json!({
            "multiSpeakerVoiceConfig": {
                "speakerVoiceConfigs": speaker_voice_configs
            }
        })
    };

    serde_json::json!({
        "contents": [{
            "parts": [{
                "text": request.prompt
            }]
        }],
        "generationConfig": {
            "responseModalities": ["AUDIO"],
            "speechConfig": speech_config
        }
    })
}

fn extract_audio(json: &Value) -> Result<(&str, Option<String>), AppError> {
    let candidates = json
        .get("candidates")
        .and_then(Value::as_array)
        .ok_or_else(|| AppError::Transient("Gemini response had no candidates array".into()))?;

    for candidate in candidates {
        let Some(parts) = candidate
            .pointer("/content/parts")
            .and_then(Value::as_array)
        else {
            continue;
        };
        for part in parts {
            let inline = part.get("inlineData").or_else(|| part.get("inline_data"));
            let Some(inline) = inline else {
                continue;
            };
            let data = inline
                .get("data")
                .and_then(Value::as_str)
                .ok_or_else(|| AppError::Transient("Gemini inline audio missing data".into()))?;
            let mime = inline
                .get("mimeType")
                .or_else(|| inline.get("mime_type"))
                .and_then(Value::as_str)
                .map(str::to_string);
            return Ok((data, mime));
        }
    }

    Err(AppError::Transient(
        "Gemini response did not contain inline audio data".into(),
    ))
}

fn extract_error_message(text: &str) -> Option<String> {
    let json: Value = serde_json::from_str(text).ok()?;
    json.pointer("/error/message")
        .and_then(Value::as_str)
        .map(str::to_string)
}

#[derive(Debug, Clone, Deserialize)]
struct ModelResponse {
    name: Option<String>,
}

pub fn check_model(api_key: &str, model: &str, timeout_seconds: u64) -> Result<String, AppError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_seconds.max(10)))
        .build()?;
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{model}");
    let response = client.get(url).header("x-goog-api-key", api_key).send()?;
    let status = response.status();
    let text = response.text()?;
    if !status.is_success() {
        let message = extract_error_message(&text).unwrap_or_else(|| text.trim().to_string());
        if status.as_u16() == 429 {
            return Err(AppError::RateLimited(message));
        }
        return Err(AppError::Config(format!(
            "model check failed ({status}): {message}"
        )));
    }
    let parsed: ModelResponse = serde_json::from_str(&text)
        .map_err(|e| AppError::Transient(format!("model check returned invalid JSON: {e}")))?;
    Ok(parsed.name.unwrap_or_else(|| model.into()))
}

#[cfg(test)]
mod tests {
    use super::{parse_retry_delay, rate_limit_info};

    #[test]
    fn parses_google_retry_hint() {
        let delay = parse_retry_delay("Quota exceeded. Please retry in 2.830310597s.").unwrap();
        assert_eq!(delay.as_millis(), 2_831);
    }

    #[test]
    fn parses_google_hour_retry_hint() {
        let delay = parse_retry_delay("Please retry in 3h3m25.362287008s.").unwrap();
        assert_eq!(delay.as_secs(), 11_005);
    }

    #[test]
    fn extracts_rate_limit_info() {
        let message = "You exceeded your current quota.\n* Quota exceeded for metric: generativelanguage.googleapis.com/generate_requests_per_model_per_day, limit: 100, model: gemini-3.1-flash-tts\nPlease retry in 3h3m25.362287008s.";
        let info = rate_limit_info(message);
        assert_eq!(
            info.quota_metric.as_deref(),
            Some("generativelanguage.googleapis.com/generate_requests_per_model_per_day")
        );
        assert_eq!(info.quota_kind.as_deref(), Some("requests_per_day"));
        assert_eq!(info.limit, Some(100));
        assert_eq!(info.model.as_deref(), Some("gemini-3.1-flash-tts"));
        assert!(info.retry_after_seconds.unwrap() > 11_000.0);
    }

    #[test]
    fn missing_retry_hint_returns_none() {
        assert!(parse_retry_delay("Quota exceeded").is_none());
    }
}
