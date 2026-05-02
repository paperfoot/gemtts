use base64::Engine;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

use crate::error::AppError;
use crate::prompt::SpeakerVoice;

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
}

pub fn generate(api_key: &str, request: &GenerateRequest) -> Result<GenerateResponse, AppError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(request.timeout_seconds.max(10)))
        .build()?;

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
    })
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
        },
        "model": request.model
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
