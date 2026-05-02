use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli::AudioFormat;
use crate::config;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize)]
pub struct AudioWriteResult {
    pub path: String,
    pub format: AudioFormat,
    pub bytes_written: u64,
    pub sample_rate: u32,
    pub channels: u16,
    pub encoder: String,
}

pub fn resolve_format(requested: AudioFormat, out: &Path) -> AudioFormat {
    if requested != AudioFormat::Auto {
        return requested;
    }
    match out
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("wav")
        .to_ascii_lowercase()
        .as_str()
    {
        "pcm" | "raw" => AudioFormat::Pcm,
        "mp3" => AudioFormat::Mp3,
        "m4a" | "aac" => AudioFormat::M4a,
        "flac" => AudioFormat::Flac,
        _ => AudioFormat::Wav,
    }
}

pub fn write_audio(
    pcm: &[u8],
    out: &Path,
    requested: AudioFormat,
    sample_rate: u32,
    channels: u16,
) -> Result<AudioWriteResult, AppError> {
    if pcm.is_empty() {
        return Err(AppError::Audio("Gemini returned empty audio data".into()));
    }
    let format = resolve_format(requested, out);
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    match format {
        AudioFormat::Wav | AudioFormat::Auto => {
            write_wav(out, pcm, sample_rate, channels)?;
            result(out, format, sample_rate, channels, "native-wav")
        }
        AudioFormat::Pcm => {
            std::fs::write(out, pcm)?;
            result(out, format, sample_rate, channels, "native-pcm")
        }
        AudioFormat::Mp3 | AudioFormat::M4a | AudioFormat::Flac => {
            write_with_ffmpeg(pcm, out, format, sample_rate, channels)?;
            result(out, format, sample_rate, channels, "ffmpeg")
        }
    }
}

fn result(
    out: &Path,
    format: AudioFormat,
    sample_rate: u32,
    channels: u16,
    encoder: &str,
) -> Result<AudioWriteResult, AppError> {
    let bytes_written = std::fs::metadata(out)?.len();
    Ok(AudioWriteResult {
        path: out.display().to_string(),
        format,
        bytes_written,
        sample_rate,
        channels,
        encoder: encoder.into(),
    })
}

fn write_wav(path: &Path, pcm: &[u8], sample_rate: u32, channels: u16) -> Result<(), AppError> {
    let mut bytes = Vec::with_capacity(44 + pcm.len());
    let bits_per_sample = 16u16;
    let byte_rate = sample_rate * channels as u32 * bits_per_sample as u32 / 8;
    let block_align = channels * bits_per_sample / 8;
    let data_len = pcm.len() as u32;
    let riff_len = 36u32
        .checked_add(data_len)
        .ok_or_else(|| AppError::Audio("audio too large for wav container".into()))?;

    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&riff_len.to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&channels.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&byte_rate.to_le_bytes());
    bytes.extend_from_slice(&block_align.to_le_bytes());
    bytes.extend_from_slice(&bits_per_sample.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_len.to_le_bytes());
    bytes.extend_from_slice(pcm);
    std::fs::write(path, bytes)?;
    Ok(())
}

fn write_with_ffmpeg(
    pcm: &[u8],
    out: &Path,
    format: AudioFormat,
    sample_rate: u32,
    channels: u16,
) -> Result<(), AppError> {
    if !ffmpeg_available() {
        return Err(AppError::Audio(
            "ffmpeg is required for mp3, m4a, and flac output".into(),
        ));
    }

    std::fs::create_dir_all(config::cache_dir())?;
    let tmp = temp_pcm_path();
    std::fs::write(&tmp, pcm)?;

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-hide_banner")
        .arg("-loglevel")
        .arg("error")
        .arg("-y")
        .arg("-f")
        .arg("s16le")
        .arg("-ar")
        .arg(sample_rate.to_string())
        .arg("-ac")
        .arg(channels.to_string())
        .arg("-i")
        .arg(&tmp);

    match format {
        AudioFormat::Mp3 => {
            cmd.arg("-codec:a").arg("libmp3lame").arg("-q:a").arg("2");
        }
        AudioFormat::M4a => {
            cmd.arg("-codec:a").arg("aac").arg("-b:a").arg("192k");
        }
        AudioFormat::Flac => {
            cmd.arg("-codec:a").arg("flac");
        }
        AudioFormat::Auto | AudioFormat::Wav | AudioFormat::Pcm => {}
    }

    let output = cmd.arg(out).output()?;
    let _ = std::fs::remove_file(&tmp);
    if !output.status.success() {
        return Err(AppError::Audio(format!(
            "ffmpeg failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(())
}

fn temp_pcm_path() -> PathBuf {
    config::cache_dir().join(format!(
        "gemini-tts-{}-{}.pcm",
        std::process::id(),
        now_millis()
    ))
}

fn now_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

pub fn ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .is_ok_and(|out| out.status.success())
}

pub fn play(path: &Path) -> Result<(), AppError> {
    let player = if cfg!(target_os = "macos") {
        "afplay"
    } else {
        "ffplay"
    };
    let status = if player == "ffplay" {
        Command::new(player)
            .arg("-nodisp")
            .arg("-autoexit")
            .arg(path)
            .status()
    } else {
        Command::new(player).arg(path).status()
    };

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(AppError::Audio(format!(
            "audio player exited with status {status}"
        ))),
        Err(e) => Err(AppError::Audio(format!("audio player not available: {e}"))),
    }
}
