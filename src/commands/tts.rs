use serde::Serialize;
use serde_json::{Value, json};

use crate::audio::{self, AudioWriteResult};
use crate::catalog;
use crate::cli::{AudioFormat, LintArgs, ScriptArgs, SpeakArgs};
use crate::config::{self, AppConfig};
use crate::error::AppError;
use crate::gemini::{self, GenerateRequest};
use crate::guard::GenerationGuard;
use crate::output::{self, Ctx};
use crate::prompt::{self, LintFinding};
use crate::usage;

#[derive(Serialize)]
struct SpeakResult {
    audio: AudioWriteResult,
    model: String,
    voice: String,
    mime_type: String,
    prompt_chars: usize,
    transcript_chars: usize,
    usage: usage::UsageEstimate,
    usage_ledger_path: String,
    structured_prompt: bool,
    warnings: Vec<String>,
    speakers: Vec<prompt::SpeakerVoice>,
}

pub fn speak(ctx: Ctx, args: SpeakArgs, config: &AppConfig) -> Result<(), AppError> {
    let transcript = prompt::load_text(&args.text, args.text_file)?;
    let prompt_build = prompt::build_for_speak(&transcript, &args, config)?;
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| config.defaults.model.clone());
    let voice_input = args
        .voice
        .clone()
        .unwrap_or_else(|| config.defaults.voice.clone());
    let Some(voice) = catalog::canonical_voice_name(&voice_input).map(str::to_string) else {
        return Err(AppError::InvalidInput(format!(
            "unsupported Gemini TTS voice {voice_input:?}. Valid voices: {}",
            catalog::voice_names().join(", ")
        )));
    };
    let (_source, api_key) = config::require_api_key(config)?;
    let _guard = GenerationGuard::acquire(args.force)?;

    let request = GenerateRequest {
        model: model.clone(),
        prompt: prompt_build.prompt.clone(),
        voice: voice.clone(),
        speakers: prompt_build.speakers.clone(),
        timeout_seconds: config.defaults.timeout_seconds,
    };
    let generated = gemini::generate(&api_key, &request)?;
    let requested_format = if args.format == crate::cli::AudioFormat::Auto {
        config.defaults.audio_format
    } else {
        args.format
    };
    let audio = audio::write_audio(
        &generated.pcm,
        &args.out,
        requested_format,
        config.defaults.sample_rate,
        config.defaults.channels,
    )?;
    let usage_estimate = usage::estimate(
        generated.pcm.len(),
        config.defaults.sample_rate,
        config.defaults.channels,
        generated.prompt_chars,
        &generated.usage,
    );
    let usage_record = usage::UsageRecord {
        created_unix_seconds: usage::now_unix_seconds(),
        command: "speak".into(),
        model: model.clone(),
        voice: voice.clone(),
        speakers: prompt_build.speakers.clone(),
        output_path: audio.path.clone(),
        output_format: audio.format,
        prompt_chars: generated.prompt_chars,
        transcript_chars: prompt_build.transcript_chars,
        api_usage: generated.usage.clone(),
        estimate: usage_estimate.clone(),
    };
    usage::append_record(&usage_record)?;

    if args.play {
        audio::play(&args.out)?;
    }

    let result = SpeakResult {
        audio,
        model,
        voice,
        mime_type: generated.mime_type,
        prompt_chars: generated.prompt_chars,
        transcript_chars: prompt_build.transcript_chars,
        usage: usage_estimate,
        usage_ledger_path: usage::usage_path().display().to_string(),
        structured_prompt: prompt_build.structured,
        warnings: prompt_build.warnings,
        speakers: prompt_build.speakers,
    };
    output::print_success_or(ctx, &result, |r| {
        use owo_colors::OwoColorize;
        println!(
            "{} {} ({}, {} bytes)",
            "wrote".green().bold(),
            r.audio.path,
            r.audio.format,
            r.audio.bytes_written
        );
        println!(
            "estimated cost: ${:.6} ({} input tokens, {} audio tokens, {:.2}s)",
            r.usage.total_cost_usd,
            r.usage.input_tokens,
            r.usage.audio_output_tokens,
            r.usage.audio_seconds
        );
        if !r.warnings.is_empty() {
            for warning in &r.warnings {
                eprintln!("warning: {warning}");
            }
        }
    });
    Ok(())
}

#[derive(Serialize)]
struct ScriptResult {
    prompt: String,
    prompt_chars: usize,
    transcript_chars: usize,
    out: Option<String>,
    warnings: Vec<String>,
    speakers: Vec<prompt::SpeakerVoice>,
}

pub fn script(ctx: Ctx, args: ScriptArgs, config: &AppConfig) -> Result<(), AppError> {
    let transcript = prompt::load_text(&args.text, args.text_file)?;
    let built = prompt::build_for_script(&transcript, &args, config)?;
    if let Some(path) = &args.out {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(path, &built.prompt)?;
    }
    let result = ScriptResult {
        prompt: built.prompt,
        prompt_chars: built.prompt_chars,
        transcript_chars: built.transcript_chars,
        out: args.out.as_ref().map(|p| p.display().to_string()),
        warnings: built.warnings,
        speakers: built.speakers,
    };
    output::print_success_or(ctx, &result, |r| {
        if let Some(path) = &r.out {
            println!("Wrote prompt to {path}");
        } else {
            println!("{}", r.prompt);
        }
    });
    Ok(())
}

#[derive(Serialize)]
struct LintResult {
    findings: Vec<LintFinding>,
    summary: LintSummary,
}

#[derive(Serialize)]
struct LintSummary {
    ok: bool,
    warn: usize,
    info: usize,
}

pub fn lint(ctx: Ctx, args: LintArgs) -> Result<(), AppError> {
    let text = prompt::load_text(&args.text, args.text_file)?;
    let findings = prompt::lint_prompt(&text, &args.speaker);
    let warn = findings.iter().filter(|f| f.severity == "warn").count();
    let info = findings.iter().filter(|f| f.severity == "info").count();
    let result = LintResult {
        summary: LintSummary {
            ok: warn == 0,
            warn,
            info,
        },
        findings,
    };
    output::print_success_or(ctx, &result, |r| {
        use owo_colors::OwoColorize;
        if r.summary.ok {
            println!("{}", "No blocking TTS prompt issues found".green());
        }
        for finding in &r.findings {
            let label = if finding.severity == "warn" {
                "warn".yellow().to_string()
            } else {
                "info".dimmed().to_string()
            };
            println!("[{label}] {}: {}", finding.code, finding.message);
            println!("  {}", finding.suggestion);
        }
    });
    Ok(())
}

#[derive(Serialize)]
pub struct DoctorCheck {
    pub name: String,
    pub status: String,
    pub message: String,
    pub suggestion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

#[derive(Serialize)]
pub struct DoctorResult {
    pub checks: Vec<DoctorCheck>,
    pub summary: DoctorSummary,
}

#[derive(Serialize)]
pub struct DoctorSummary {
    pub pass: usize,
    pub warn: usize,
    pub fail: usize,
}

pub fn doctor(
    ctx: Ctx,
    live: bool,
    require_ffmpeg: bool,
    config: &AppConfig,
) -> Result<(), AppError> {
    let mut checks = Vec::new();
    let path = config::config_path();
    let existing_path = config::existing_config_path();
    checks.push(DoctorCheck {
        name: "config_file".into(),
        status: if existing_path.is_some() {
            "pass"
        } else {
            "warn"
        }
        .into(),
        message: if let Some(path) = existing_path {
            path.display().to_string()
        } else {
            format!(
                "{} does not exist; defaults/env will be used",
                path.display()
            )
        },
        suggestion: if config::existing_config_path().is_some() {
            None
        } else {
            Some(format!("Run {} config init", env!("CARGO_PKG_NAME")))
        },
        details: None,
    });

    let key = config::api_key(config);
    checks.push(DoctorCheck {
        name: "api_key".into(),
        status: if key.is_some() { "pass" } else { "fail" }.into(),
        message: key
            .as_ref()
            .map(|(source, value)| format!("{} ({})", config::mask_secret(value), source))
            .unwrap_or_else(|| "not configured".into()),
        suggestion: if key.is_some() {
            None
        } else {
            Some(format!(
                "Set GEMINI_API_KEY or run {} auth set --api-key <key>",
                env!("CARGO_PKG_NAME")
            ))
        },
        details: None,
    });

    let ffmpeg = audio::ffmpeg_available();
    checks.push(DoctorCheck {
        name: "ffmpeg".into(),
        status: if ffmpeg {
            "pass"
        } else if require_ffmpeg {
            "fail"
        } else {
            "warn"
        }
        .into(),
        message: if ffmpeg {
            "ffmpeg is available for mp3/m4a/flac".into()
        } else {
            "ffmpeg not found; wav and pcm still work".into()
        },
        suggestion: if ffmpeg {
            None
        } else {
            Some("Install ffmpeg for compressed audio: brew install ffmpeg".into())
        },
        details: None,
    });

    if live {
        if let Some((_source, api_key)) = key {
            match gemini::check_model(
                &api_key,
                &config.defaults.model,
                config.defaults.timeout_seconds,
            ) {
                Ok(name) => checks.push(DoctorCheck {
                    name: "live_model".into(),
                    status: "pass".into(),
                    message: name,
                    suggestion: None,
                    details: None,
                }),
                Err(e) => checks.push(doctor_error_check("live_model", e)),
            }

            let request = GenerateRequest {
                model: config.defaults.model.clone(),
                prompt: "Say clearly: Gemini TTS live check.".into(),
                voice: config.defaults.voice.clone(),
                speakers: Vec::new(),
                timeout_seconds: config.defaults.timeout_seconds,
            };
            match gemini::generate(&api_key, &request) {
                Ok(audio) if !audio.pcm.is_empty() => {
                    let usage_estimate = usage::estimate(
                        audio.pcm.len(),
                        config.defaults.sample_rate,
                        config.defaults.channels,
                        audio.prompt_chars,
                        &audio.usage,
                    );
                    let usage_record = usage::UsageRecord {
                        created_unix_seconds: usage::now_unix_seconds(),
                        command: "doctor --live".into(),
                        model: request.model.clone(),
                        voice: request.voice.clone(),
                        speakers: Vec::new(),
                        output_path: "(discarded live check audio)".into(),
                        output_format: AudioFormat::Pcm,
                        prompt_chars: audio.prompt_chars,
                        transcript_chars: request.prompt.chars().count(),
                        api_usage: audio.usage.clone(),
                        estimate: usage_estimate,
                    };
                    let usage_message = match usage::append_record(&usage_record) {
                        Ok(()) => "usage logged",
                        Err(e) => {
                            checks.push(DoctorCheck {
                                name: "usage_ledger".into(),
                                status: "fail".into(),
                                message: e.to_string(),
                                suggestion: Some(
                                    "Check write permissions for the gemtts state directory."
                                        .into(),
                                ),
                                details: None,
                            });
                            "usage logging failed"
                        }
                    };
                    checks.push(DoctorCheck {
                        name: "live_audio".into(),
                        status: "pass".into(),
                        message: format!(
                            "{} bytes, {}; {}",
                            audio.pcm.len(),
                            audio.mime_type,
                            usage_message
                        ),
                        suggestion: None,
                        details: None,
                    });
                }
                Ok(_) => checks.push(DoctorCheck {
                    name: "live_audio".into(),
                    status: "fail".into(),
                    message: "Gemini returned empty audio data".into(),
                    suggestion: Some("Retry once; if it repeats, check model/key status.".into()),
                    details: None,
                }),
                Err(e) => checks.push(doctor_error_check("live_audio", e)),
            }
        }
    }

    let summary = DoctorSummary {
        pass: checks.iter().filter(|c| c.status == "pass").count(),
        warn: checks.iter().filter(|c| c.status == "warn").count(),
        fail: checks.iter().filter(|c| c.status == "fail").count(),
    };
    let has_fail = summary.fail > 0;
    let result = DoctorResult { checks, summary };
    output::print_success_or(ctx, &result, |r| {
        use owo_colors::OwoColorize;
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Check", "Status", "Message"]);
        for check in &r.checks {
            let status = match check.status.as_str() {
                "pass" => "pass".green().to_string(),
                "warn" => "warn".yellow().to_string(),
                _ => "fail".red().to_string(),
            };
            table.add_row(vec![check.name.clone(), status, check.message.clone()]);
        }
        println!("{table}");
    });

    if has_fail {
        std::process::exit(2);
    }
    Ok(())
}

fn doctor_error_check(name: &str, error: AppError) -> DoctorCheck {
    let status = if error.exit_code() == 4 {
        "warn"
    } else {
        "fail"
    };
    let (suggestion, details) = match &error {
        AppError::RateLimited(message) => {
            let info = gemini::rate_limit_info(message);
            let retry = info.retry_after_seconds.unwrap_or_default();
            let suggestion = if info.quota_kind.as_deref() == Some("requests_per_day") {
                "Gemini returned a per-day request quota error for the API key currently used; wait for the Google retry-after window and verify the matching AI Studio project. This is not proof by itself that credits are depleted."
            } else if retry > 60.0 {
                "Gemini returned a long retry-after window; wait for that window or check AI Studio rate limits and billing status."
            } else {
                "Gemini returned a short rate limit; retry after the reported retry-after window."
            };
            (
                suggestion.to_string(),
                Some(json!({
                    "rate_limit": info,
                    "check_urls": {
                        "active_rate_limits": "https://aistudio.google.com/ratelimits",
                        "billing": "https://aistudio.google.com/billing"
                    }
                })),
            )
        }
        _ => (error.suggestion().into(), None),
    };
    DoctorCheck {
        name: name.into(),
        status: status.into(),
        message: error.to_string(),
        suggestion: Some(suggestion),
        details,
    }
}
