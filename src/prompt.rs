use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::catalog;
use crate::cli::{ScriptArgs, SpeakArgs};
use crate::config::AppConfig;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerVoice {
    pub speaker: String,
    pub voice: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PromptBuild {
    pub prompt: String,
    pub structured: bool,
    pub transcript_chars: usize,
    pub prompt_chars: usize,
    pub warnings: Vec<String>,
    pub speakers: Vec<SpeakerVoice>,
}

#[derive(Debug, Clone, Default)]
pub struct Direction {
    pub profile: Option<String>,
    pub scene: Option<String>,
    pub style: Option<String>,
    pub pace: Option<String>,
    pub accent: Option<String>,
    pub language: Option<String>,
    pub tags: Vec<String>,
    pub speakers: Vec<SpeakerVoice>,
}

impl Direction {
    pub fn from_speak(args: &SpeakArgs, config: &AppConfig) -> Result<Self, AppError> {
        Ok(Self {
            profile: args.profile.clone(),
            scene: args.scene.clone(),
            style: args.style.clone(),
            pace: args.pace.clone(),
            accent: args.accent.clone(),
            language: args.language.clone(),
            tags: args.tag.clone(),
            speakers: parse_speakers(&args.speaker, config)?,
        })
    }

    pub fn from_script(args: &ScriptArgs, config: &AppConfig) -> Result<Self, AppError> {
        Ok(Self {
            profile: args.profile.clone(),
            scene: args.scene.clone(),
            style: args.style.clone(),
            pace: args.pace.clone(),
            accent: args.accent.clone(),
            language: args.language.clone(),
            tags: args.tag.clone(),
            speakers: parse_speakers(&args.speaker, config)?,
        })
    }

    fn has_explicit_direction(&self) -> bool {
        self.profile.is_some()
            || self.scene.is_some()
            || self.style.is_some()
            || self.pace.is_some()
            || self.accent.is_some()
            || self.language.is_some()
            || !self.tags.is_empty()
            || !self.speakers.is_empty()
    }
}

pub fn load_text(text: &str, from_file: bool) -> Result<String, AppError> {
    if from_file {
        let path = Path::new(text);
        return std::fs::read_to_string(path).map_err(AppError::from);
    }
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(AppError::InvalidInput("text cannot be empty".into()));
    }
    Ok(trimmed.to_string())
}

pub fn build_for_speak(
    transcript: &str,
    args: &SpeakArgs,
    config: &AppConfig,
) -> Result<PromptBuild, AppError> {
    let direction = Direction::from_speak(args, config)?;
    if args.raw || !direction.has_explicit_direction() || looks_structured(transcript) {
        return Ok(raw_prompt(transcript, direction.speakers));
    }
    Ok(structured_prompt(transcript, &direction, config))
}

pub fn build_for_script(
    transcript: &str,
    args: &ScriptArgs,
    config: &AppConfig,
) -> Result<PromptBuild, AppError> {
    let direction = Direction::from_script(args, config)?;
    Ok(structured_prompt(transcript, &direction, config))
}

fn raw_prompt(transcript: &str, speakers: Vec<SpeakerVoice>) -> PromptBuild {
    let warnings = lint_prompt(transcript, &speaker_names(&speakers))
        .into_iter()
        .filter(|w| w.severity != "info")
        .map(|w| w.message)
        .collect::<Vec<_>>();
    PromptBuild {
        prompt: transcript.to_string(),
        structured: false,
        transcript_chars: transcript.chars().count(),
        prompt_chars: transcript.chars().count(),
        warnings,
        speakers,
    }
}

fn structured_prompt(transcript: &str, direction: &Direction, config: &AppConfig) -> PromptBuild {
    let profile = direction
        .profile
        .as_deref()
        .unwrap_or(&config.prompt.profile);
    let scene = direction.scene.as_deref().unwrap_or(&config.prompt.scene);
    let style = direction.style.as_deref().unwrap_or(&config.prompt.style);
    let pace = direction.pace.as_deref().unwrap_or(&config.prompt.pace);
    let accent = direction.accent.as_deref().unwrap_or(&config.prompt.accent);
    let language = direction
        .language
        .as_deref()
        .unwrap_or(&config.prompt.language);

    let mut prompt = String::new();
    prompt.push_str("Synthesize speech for the performance defined below. The audio profile, scene, director notes, cast, and context are direction only. Do not speak them. Speak only the lines under #### TRANSCRIPT.\n\n");
    prompt.push_str("# AUDIO PROFILE: ");
    prompt.push_str(profile);
    prompt.push_str("\n\n## THE SCENE\n");
    prompt.push_str(scene);
    prompt.push_str("\n\n### DIRECTOR'S NOTES\n");
    prompt.push_str("Style: ");
    prompt.push_str(style);
    prompt.push_str("\nPacing: ");
    prompt.push_str(pace);
    prompt.push_str("\nAccent: ");
    prompt.push_str(accent);
    prompt.push_str("\nLanguage: ");
    prompt.push_str(language);
    prompt.push_str("\nReliability: Keep the selected voice and written tone aligned. Use tags sparingly; tags should modify delivery, not replace a coherent transcript.");

    if !direction.tags.is_empty() {
        prompt.push_str("\nUseful inline tags for this script: ");
        prompt.push_str(&direction.tags.join(" "));
    }

    if !direction.speakers.is_empty() {
        prompt.push_str("\n\n### CAST\n");
        for speaker in &direction.speakers {
            prompt.push_str("- ");
            prompt.push_str(&speaker.speaker);
            prompt.push_str(": use voice ");
            prompt.push_str(&speaker.voice);
            prompt.push_str(". Transcript lines must start with exactly this speaker name.\n");
        }
    }

    prompt.push_str("\n\n#### TRANSCRIPT\n");
    prompt.push_str(transcript.trim());

    let warnings = lint_prompt(&prompt, &speaker_names(&direction.speakers))
        .into_iter()
        .filter(|w| w.severity != "info")
        .map(|w| w.message)
        .collect::<Vec<_>>();

    PromptBuild {
        prompt_chars: prompt.chars().count(),
        transcript_chars: transcript.chars().count(),
        prompt,
        structured: true,
        warnings,
        speakers: direction.speakers.clone(),
    }
}

pub fn parse_speakers(raw: &[String], _config: &AppConfig) -> Result<Vec<SpeakerVoice>, AppError> {
    if raw.len() > 2 {
        return Err(AppError::InvalidInput(
            "Gemini TTS multi-speaker config supports at most 2 speakers".into(),
        ));
    }

    let mut speakers = Vec::new();
    for item in raw {
        let Some((name, voice)) = item.split_once('=') else {
            return Err(AppError::InvalidInput(format!(
                "speaker must be NAME=VOICE, got {item}"
            )));
        };
        let name = name.trim();
        let voice = voice.trim();
        if name.is_empty() || voice.is_empty() {
            return Err(AppError::InvalidInput(format!(
                "speaker must be NAME=VOICE, got {item}"
            )));
        }
        speakers.push(SpeakerVoice {
            speaker: name.to_string(),
            voice: voice.to_string(),
        });
    }

    if speakers.is_empty() {
        return Ok(Vec::new());
    }

    if speakers.len() == 1 {
        return Err(AppError::InvalidInput(
            "multi-speaker TTS requires exactly 2 --speaker NAME=VOICE mappings; use --voice for one speaker".into(),
        ));
    }

    for speaker in &mut speakers {
        let Some(voice) = catalog::canonical_voice_name(&speaker.voice) else {
            return Err(AppError::InvalidInput(format!(
                "unsupported Gemini TTS voice {:?}. Valid voices: {}",
                speaker.voice,
                catalog::voice_names().join(", ")
            )));
        };
        speaker.voice = voice.to_string();
    }

    Ok(speakers)
}

fn speaker_names(speakers: &[SpeakerVoice]) -> Vec<String> {
    speakers.iter().map(|s| s.speaker.clone()).collect()
}

fn looks_structured(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("#### transcript")
        || lower.contains("# audio profile")
        || lower.contains("### director")
}

#[derive(Debug, Clone, Serialize)]
pub struct LintFinding {
    pub severity: &'static str,
    pub code: &'static str,
    pub message: String,
    pub suggestion: String,
}

pub fn lint_prompt(text: &str, speakers: &[String]) -> Vec<LintFinding> {
    let mut findings = Vec::new();
    let chars = text.chars().count();
    let words = text.split_whitespace().count();
    let tag_count = count_tags(text);

    if chars > 6_000 {
        findings.push(LintFinding {
            severity: "warn",
            code: "long_prompt",
            message: format!(
                "Prompt is {chars} characters; long Gemini TTS generations can drift or degrade."
            ),
            suggestion:
                "Split long scripts into shorter takes and review each output before stitching."
                    .into(),
        });
    } else if words > 450 {
        findings.push(LintFinding {
            severity: "warn",
            code: "long_take",
            message: format!("Script is about {words} words; short takes are more reliable for preview TTS."),
            suggestion: "Prefer 30-60 second takes for important audio. Use stable voice/profile notes across chunks.".into(),
        });
    } else {
        findings.push(LintFinding {
            severity: "info",
            code: "length_ok",
            message: format!("Length looks reasonable: {words} words, {chars} characters."),
            suggestion: "Generate and listen to the full file once before using it downstream."
                .into(),
        });
    }

    if tag_count >= 3 && words > 0 {
        let density = tag_count as f64 / words as f64;
        if density > 0.08 {
            findings.push(LintFinding {
                severity: "warn",
                code: "tag_inflation",
                message: format!("{tag_count} inline tags across {words} words is dense."),
                suggestion:
                    "Use director notes for global tone and reserve tags for local changes.".into(),
            });
        }
    }

    if text.contains("][") {
        findings.push(LintFinding {
            severity: "warn",
            code: "adjacent_tags",
            message: "Adjacent tags may be spoken literally or ignored.".into(),
            suggestion: "Separate tags with words or punctuation, for example: [softly] Hello, [short pause] welcome back.".into(),
        });
    }

    if text.contains("].") || text.contains("]\n") {
        findings.push(LintFinding {
            severity: "info",
            code: "tag_boundary",
            message: "Tags followed by sentence breaks can sound chopped in some prompts.".into(),
            suggestion: "For smoother phrasing, try commas between tagged clauses instead of period-separated fragments.".into(),
        });
    }

    if !speakers.is_empty() {
        for speaker in speakers {
            let prefix = format!("{speaker}:");
            if !text.contains(&prefix) {
                findings.push(LintFinding {
                    severity: "warn",
                    code: "missing_speaker_line",
                    message: format!("No transcript line starts with expected speaker prefix {prefix:?}."),
                    suggestion: "For multi-speaker TTS, transcript names must match --speaker names exactly.".into(),
                });
            }
        }
    }

    if text.contains("[[tts") {
        findings.push(LintFinding {
            severity: "warn",
            code: "foreign_tts_directive",
            message: "Found [[tts...]] wrapper directives.".into(),
            suggestion: "Use Gemini inline audio tags like [whispers] directly inside the transcript; do not wrap with app-specific TTS tags.".into(),
        });
    }

    findings
}

fn count_tags(text: &str) -> usize {
    let mut count = 0;
    let mut in_tag = false;
    for ch in text.chars() {
        match (in_tag, ch) {
            (false, '[') => in_tag = true,
            (true, ']') => {
                count += 1;
                in_tag = false;
            }
            (true, '\n') => in_tag = false,
            _ => {}
        }
    }
    count
}
