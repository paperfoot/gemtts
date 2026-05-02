/// Machine-readable capability manifest.
///
/// agent-info is always JSON -- the whole point is machine readability.
pub fn run() {
    let name = env!("CARGO_PKG_NAME");
    let config_path = crate::config::config_path();

    let info = serde_json::json!({
        "name": name,
        "version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "commands": {
            "speak <text>": {
                "description": "Generate speech audio from inline text or a text file. Writes audio to --out and returns metadata.",
                "args": [{"name": "text", "kind": "positional", "type": "string", "required": true, "description": "Text to speak, or path when --text-file is set"}],
                "options": [
                    {"name": "--text-file", "type": "bool", "default": false, "description": "Read text from the positional path"},
                    {"name": "--out", "type": "path", "default": "speech.wav", "description": "Output audio file"},
                    {"name": "--format", "type": "enum", "values": ["auto", "wav", "pcm", "mp3", "m4a", "flac"], "default": "auto", "description": "wav/pcm native; mp3/m4a/flac require ffmpeg"},
                    {"name": "--voice", "type": "string", "default": "config.defaults.voice", "description": "Single-speaker Gemini prebuilt voice"},
                    {"name": "--speaker", "type": "repeat NAME=VOICE", "description": "Multi-speaker cast, exactly 2 speakers when used. Transcript names must match."},
                    {"name": "--model", "type": "string", "default": "gemini-3.1-flash-tts-preview"},
                    {"name": "--profile/--scene/--style/--pace/--accent/--language/--tag", "type": "string", "description": "Prompt direction fields. If present, the CLI builds a structured TTS prompt."},
                    {"name": "--raw", "type": "bool", "default": false, "description": "Pass text exactly as the Gemini prompt"},
                    {"name": "--play", "type": "bool", "default": false, "description": "Play generated file after writing"},
                    {"name": "--force", "type": "bool", "default": false, "description": "Bypass duplicate generation lock"}
                ],
                "examples": [
                    format!("{name} speak \"Say cheerfully: Have a wonderful day!\" --voice Kore -o day.wav"),
                    format!("{name} speak script.txt --text-file --speaker Joe=Kore --speaker Jane=Puck -o dialogue.mp3"),
                    format!("{name} speak \"[whispers] this is quiet\" --voice Achernar -o whisper.wav")
                ]
            },
            "script <text>": {
                "description": "Build a structured prompt with audio profile, scene, director notes, cast, and transcript. Does not call the API.",
                "args": [{"name": "text", "required": true}],
                "options": ["--text-file", "--out", "--profile", "--scene", "--style", "--pace", "--accent", "--language", "--tag", "--speaker"]
            },
            "lint <text>": {
                "description": "Inspect prompt/script for length, tag density, app-specific wrappers, and multi-speaker mismatch risks.",
                "args": [{"name": "text", "required": true}],
                "options": ["--text-file", "--speaker"]
            },
            "voices list": {
                "description": "List built-in Gemini voice names with practical descriptions.",
                "options": [{"name": "--query", "type": "string"}]
            },
            "voices recommend <brief>": {
                "description": "Recommend voices for a natural-language use case.",
                "args": [{"name": "brief", "required": true}],
                "options": [{"name": "--count", "type": "integer", "default": 5}]
            },
            "tags list/search/recipes": {
                "description": "Explore inline tag categories and reliable prompt recipes.",
                "options": [{"name": "--category", "values": ["emotion", "pace", "volume", "pause", "nonverbal", "character", "accent"]}]
            },
            "languages list": {
                "description": "List language and locale hints. TTS control tags should generally stay in English.",
                "options": [{"name": "--query", "type": "string"}]
            },
            "doctor": {
                "description": "Check config, key, ffmpeg, and optional live Gemini model plus audio generation access.",
                "options": [{"name": "--live", "type": "bool"}, {"name": "--require-ffmpeg", "type": "bool"}]
            },
            "auth set/import-env/status": {
                "description": "Save or inspect Gemini API key. Secrets are masked in output."
            },
            "agent-info": {"description": "This manifest", "aliases": ["info"], "args": [], "options": []},
            "skill install": {"description": "Install skill file to agent platforms", "args": [], "options": []},
            "skill status": {"description": "Check skill installation status", "args": [], "options": []},
            "config show/path/init/set/get": {"description": "Manage ~/.config/gemtts/config.toml", "args": [], "options": []},
            "update": {
                "description": "Check crates.io version and print source-aware cargo/Homebrew update guidance.",
                "args": [],
                "options": [{"name": "--check", "type": "bool", "required": false, "default": false}]
            }
        },
        "agent_guidance": {
            "fast_audio": format!("{name} speak \"Say warmly: hello\" --voice Kore -o hello.wav --json"),
            "author_first": format!("{name} script \"Welcome back\" --style \"warm expert narrator\" --tag \"[warmly]\" --json"),
            "quality_gate": format!("{name} lint script.txt --text-file --json"),
            "before_important_jobs": format!("{name} doctor --live --json"),
            "prompt_rules": [
                "Use global director notes for style, pace, accent, and language; use inline square-bracket tags only for local changes.",
                "The 30 Gemini voice names are prebuilt voice timbres, not per-language voices.",
                "Gemini TTS auto-detects transcript language. For Italian, use language code it or an Italian transcript plus --accent/--style direction; do not invent an Italian voice ID.",
                "Keep tag instructions in English even when the transcript is not English.",
                "Do not strip or normalize bracket tags before speak; Gemini uses them for delivery control.",
                "Avoid tag inflation. If every sentence has tags, move the pattern into director notes.",
                "For multi-speaker output, pass exactly two --speaker Name=Voice mappings and make transcript lines start with exactly Name:.",
                "Short takes are more reliable than long single generations. Lint warns when scripts are likely too long."
            ]
        },
        "global_flags": {
            "--json": {"description": "Force JSON output (auto-enabled when piped)", "type": "bool", "default": false},
            "--quiet": {"description": "Suppress informational output", "type": "bool", "default": false}
        },
        "exit_codes": {
            "0": "Success",
            "1": "Transient error (IO, network, audio encoder) -- retry or fix dependency",
            "2": "Config error -- fix setup/key",
            "3": "Bad input -- fix arguments",
            "4": "Rate limited -- wait and retry"
        },
        "envelope": {
            "version": "1",
            "success": "{ version, status, data }",
            "error": "{ version, status, error: { code, message, suggestion } }"
        },
        "config": {
            "path": config_path.display().to_string(),
            "env_prefix": crate::config::ENV_PREFIX,
            "api_key_env": ["GEMINI_API_KEY", "GOOGLE_API_KEY", "GOOGLE_AI_API_KEY"]
        },
        "audio": {
            "native_formats": ["wav", "pcm"],
            "ffmpeg_formats": ["mp3", "m4a", "flac"],
            "sample_rate": 24000,
            "channels": 1,
            "stdout_contract": "stdout is metadata only; audio is written to --out"
        },
        "google_tts_facts": {
            "endpoint": "POST https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent",
            "model_check_endpoint": "GET https://generativelanguage.googleapis.com/v1beta/models/{model}",
            "default_model": "gemini-3.1-flash-tts-preview",
            "voice_count": 30,
            "voice_policy": "Voice names are language-neutral prebuilt timbres from Google docs; language is not selected through voiceName.",
            "language_policy": "TTS auto-detects transcript language. The language catalog uses Google's documented BCP-47 codes; inline tags should generally stay in English.",
            "output_policy": "Gemini TTS returns base64 inline audio as raw signed 16-bit 24 kHz mono PCM. The CLI wraps WAV itself and uses ffmpeg for compressed formats.",
            "limits": [
                "Text input only, audio output only",
                "Model page lists 8192 input tokens and 16384 output tokens for gemini-3.1-flash-tts-preview",
                "Speech guide also describes a 32k-token TTS session context window",
                "No streaming for TTS",
                "Up to 2 speakers in multi-speaker config",
                "Preview model can occasionally return server errors or classifier rejections; the CLI retries transient generation failures"
            ]
        },
        "auto_json_when_piped": true
    });
    println!("{}", serde_json::to_string_pretty(&info).unwrap());
}
