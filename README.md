# gemtts

Agent-friendly Gemini text-to-speech CLI for expressive scripts, voices, tags, languages, and audio files.

It is built around practical use by AI agents: the binary explains itself with `agent-info`, emits JSON envelopes when piped, keeps audio out of stdout, diagnoses its own setup, and gives agents voice/tag/script guidance instead of exposing only a raw API call.

## Install

```bash
cargo install gemtts
```

For compressed output formats, install `ffmpeg`:

```bash
brew install ffmpeg
```

## Quick Start

```bash
gemtts auth import-env
gemtts doctor --live

gemtts speak "Say warmly: Gemini TTS is ready." --voice Kore -o ready.wav
gemtts speak "[whispers] This part is quiet." --voice Achernar -o whisper.mp3 --format mp3
```

WAV and raw PCM are written directly. MP3, M4A, and FLAC use `ffmpeg`.

Gemini TTS does not expose separate per-language voice IDs. Google documents 30
prebuilt voice names as voice timbres, and the model auto-detects the transcript
language. For Italian, use an Italian transcript plus `--language Italian` or
`--language it`, and add accent direction such as `--accent "heavy Italian
accent"` when the accent matters.

## Agent Workflows

Discover the command contract:

```bash
gemtts agent-info
```

Choose a voice:

```bash
gemtts voices list
gemtts voices recommend "warm expert narrator for medical guidance"
```

Find tags and prompt recipes:

```bash
gemtts tags list
gemtts tags search whisper
gemtts tags recipes
```

Build a structured prompt before generation:

```bash
gemtts script "Welcome back. The audio pipeline is ready." \
  --style "warm expert narrator with a slight smile" \
  --accent "British English from London" \
  --tag "[warmly]" \
  --out prompt.txt
```

Generate from a script:

```bash
gemtts speak prompt.txt --text-file --voice Sulafat -o narration.wav
```

Multi-speaker dialogue:

```bash
gemtts speak dialogue.txt --text-file \
  --speaker Host=Kore \
  --speaker Guest=Puck \
  -o dialogue.mp3
```

For multi-speaker output, transcript lines should use the exact speaker names:

```text
Host: Welcome back.
Guest: [excitedly] This is the good part.
```

## Prompt Quality

Gemini 3.1 Flash TTS responds well to a clear structure:

```text
Synthesize speech for the performance defined below. The audio profile, scene,
director notes, cast, and context are direction only. Do not speak them. Speak
only the lines under #### TRANSCRIPT.

# AUDIO PROFILE: Clear narrator

## THE SCENE
A clean studio recording for direct listener comprehension.

### DIRECTOR'S NOTES
Style: warm, precise, expressive without overacting.
Pacing: medium pace with deliberate pauses.
Accent: British English from London.
Language: English.

#### TRANSCRIPT
[warmly] Welcome back. [short pause] The audio pipeline is ready.
```

Use director notes for global tone. Use square-bracket tags for local changes:

```text
[warmly] [whispers] [shouting] [short pause] [very slow] [sighs] [laughs]
```

Run lint before important jobs:

```bash
gemtts lint prompt.txt --text-file
```

The linter checks for long takes, tag inflation, app-specific `[[tts]]` wrappers, and multi-speaker name mismatches. This is based on current Gemini TTS docs and public issue patterns: preserve Gemini tags, keep tags in English, avoid over-specifying every sentence, and split long takes when quality matters.

`doctor --live` checks both Google endpoints used by the CLI: it reads the model
metadata endpoint and makes a tiny `generateContent` request, then verifies that
Gemini returned non-empty PCM audio.

## Configuration

Config lives at:

```text
~/.config/gemtts/config.toml
```

Commands:

```bash
gemtts config init
gemtts config show
gemtts config set defaults.voice Sulafat
gemtts config set defaults.audio_format mp3
gemtts config get keys.api_key
gemtts update --check
gemtts usage summary
```

API key sources:

```bash
export GEMINI_API_KEY=...
gemtts auth import-env
gemtts auth status
```

Secrets are masked in command output. The config file is written with `0600` permissions on Unix.

## JSON Contract

In a terminal, commands render human-readable output. When piped or with `--json`, commands emit a JSON envelope:

```json
{
  "version": "1",
  "status": "success",
  "data": {
    "audio": {
      "path": "ready.wav",
      "format": "wav",
      "sample_rate": 24000,
      "channels": 1
    }
  }
}
```

Audio is always written to `--out`. Stdout stays metadata-only so agents can pipe it safely.

## Usage And Cost Estimates

Every successful TTS generation call, including `speak` and `doctor --live`,
appends a JSONL record to:

```text
~/.local/share/gemtts/usage.jsonl
```

Commands:

```bash
gemtts usage summary
gemtts usage list --limit 10
gemtts usage path
```

The ledger stores Gemini `usageMetadata` when the API returns it, plus an audio
duration estimate from decoded PCM. Cost estimates use Google Gemini 3.1 Flash
TTS standard paid pricing: `$1.00 / 1M` text input tokens and `$20.00 / 1M`
audio output tokens, with audio output counted as 25 tokens per second. Free
tier usage may be charged at `$0`; treat the CLI number as a conservative paid
tier estimate, not a billing statement.

## Exit Codes

| Code | Meaning |
| --- | --- |
| `0` | Success |
| `1` | Transient, IO, network, or audio encoder error |
| `2` | Config or credential error |
| `3` | Bad input |
| `4` | Rate limited |

## Development

```bash
cargo test
cargo run -- agent-info
cargo run -- doctor --live
```

## License

MIT
