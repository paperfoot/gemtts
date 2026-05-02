use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

const HELP_FOOTER: &str = r#"Tips:
  * Fast path: gemini-tts-cli speak "Say warmly: hello" -o hello.wav
  * Tags pass through: gemini-tts-cli speak "[whispers] this part is quiet" --voice Achernar
  * Add direction only when useful: --style, --pace, --accent, --scene, --profile
  * Use script first when an agent is authoring performance text, then pipe that prompt into speak
  * Run voices recommend "excited podcast host" or tags list to choose better defaults
  * MP3, M4A, and FLAC require ffmpeg; WAV and PCM are written directly
  * Run doctor --live before important jobs to verify the API key and model actually produce audio

Examples:
  gemini-tts-cli speak "Say cheerfully: Have a wonderful day!" --voice Kore -o day.wav
  gemini-tts-cli script "Welcome back." --style "calm expert narrator" --accent "London English" --tag "[warmly]"
  gemini-tts-cli speak script.txt --text-file --speaker Joe=Kore --speaker Jane=Puck -o dialogue.mp3
  gemini-tts-cli voices recommend "sleepy intimate audiobook"
  gemini-tts-cli auth import-env
"#;

#[derive(Parser)]
#[command(
    version,
    about = "Generate expressive Gemini TTS audio for humans and AI agents",
    long_about = "Agent-friendly Gemini text-to-speech CLI. It generates WAV/PCM natively, compressed audio through ffmpeg, and exposes voices, tags, languages, prompt templates, diagnostics, and JSON envelopes for reliable agent use.",
    after_long_help = HELP_FOOTER
)]
pub struct Cli {
    /// Force JSON output even in a terminal
    #[arg(long, global = true)]
    pub json: bool,

    /// Suppress informational output
    #[arg(long, global = true)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate speech audio from text or a text file
    Speak(SpeakArgs),
    /// Build a structured Gemini TTS prompt without calling the API
    Script(ScriptArgs),
    /// Inspect a script for tag, length, and prompt reliability issues
    Lint(LintArgs),
    /// Explore and recommend Gemini prebuilt voices
    Voices {
        #[command(subcommand)]
        action: VoicesAction,
    },
    /// Explore inline audio tags and recipes
    Tags {
        #[command(subcommand)]
        action: TagsAction,
    },
    /// List language hints and locale codes useful for prompts
    Languages {
        #[command(subcommand)]
        action: LanguagesAction,
    },
    /// Check config, credentials, ffmpeg, and optionally live audio generation
    Doctor(DoctorArgs),
    /// Manage Gemini API key
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    /// Machine-readable capability manifest
    #[command(visible_alias = "info")]
    AgentInfo,
    /// Manage skill file installation
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Show package-manager update guidance
    Update {
        /// Check only, don't install
        #[arg(long)]
        check: bool,
    },
    /// Hidden: deterministic exit-code trigger for contract tests
    #[command(hide = true)]
    Contract {
        /// Exit code to trigger (0-4)
        code: i32,
    },
}

#[derive(Args, Clone)]
pub struct SpeakArgs {
    /// Text to speak. If --text-file is set, this is a path to a UTF-8 text file.
    pub text: String,

    /// Treat the positional text as a file path.
    #[arg(long)]
    pub text_file: bool,

    /// Output audio path. Extension is used when --format auto.
    #[arg(short, long, default_value = "speech.wav")]
    pub out: PathBuf,

    /// Output format. wav/pcm are native; mp3/m4a/flac require ffmpeg.
    #[arg(long, value_enum, default_value = "auto")]
    pub format: AudioFormat,

    /// Gemini TTS model.
    #[arg(long)]
    pub model: Option<String>,

    /// Prebuilt voice for single-speaker output.
    #[arg(long)]
    pub voice: Option<String>,

    /// Multi-speaker mapping NAME=VOICE. Provide exactly 2 mappings when used.
    #[arg(long, value_name = "NAME=VOICE", num_args = 1..=2)]
    pub speaker: Vec<String>,

    /// Audio profile name, for example "Jaz R." or "Calm clinic narrator".
    #[arg(long)]
    pub profile: Option<String>,

    /// Scene or recording context.
    #[arg(long)]
    pub scene: Option<String>,

    /// Style direction, for example "warm, precise, expert".
    #[arg(long)]
    pub style: Option<String>,

    /// Pace direction, for example "slow and deliberate" or "fast promo read".
    #[arg(long)]
    pub pace: Option<String>,

    /// Accent direction, for example "British English from London".
    #[arg(long)]
    pub accent: Option<String>,

    /// Prompt-only language hint. Gemini auto-detects language; use codes like it or names like Italian.
    #[arg(long)]
    pub language: Option<String>,

    /// Inline tag hints to prepend as director notes, for example "[warmly]".
    #[arg(long, value_name = "TAG")]
    pub tag: Vec<String>,

    /// Use the text exactly as the Gemini prompt, even when direction flags are present.
    #[arg(long)]
    pub raw: bool,

    /// Play the generated audio after writing it (afplay on macOS, ffplay otherwise).
    #[arg(long)]
    pub play: bool,

    /// Allow a second expensive generation when a recent lock file exists.
    #[arg(long)]
    pub force: bool,
}

#[derive(Args, Clone)]
pub struct ScriptArgs {
    /// Transcript text. If --text-file is set, this is a path to a UTF-8 text file.
    pub text: String,

    /// Treat the positional text as a file path.
    #[arg(long)]
    pub text_file: bool,

    /// Optional path to save the structured prompt.
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Audio profile name.
    #[arg(long)]
    pub profile: Option<String>,

    /// Scene or recording context.
    #[arg(long)]
    pub scene: Option<String>,

    /// Style direction.
    #[arg(long)]
    pub style: Option<String>,

    /// Pace direction.
    #[arg(long)]
    pub pace: Option<String>,

    /// Accent direction.
    #[arg(long)]
    pub accent: Option<String>,

    /// Prompt-only language hint. Gemini auto-detects language; this is not an API locale field.
    #[arg(long)]
    pub language: Option<String>,

    /// Inline tag hints to use in the transcript.
    #[arg(long, value_name = "TAG")]
    pub tag: Vec<String>,

    /// Multi-speaker mapping NAME=VOICE, used as cast notes. Provide exactly 2 mappings when used.
    #[arg(long, value_name = "NAME=VOICE", num_args = 1..=2)]
    pub speaker: Vec<String>,
}

#[derive(Args, Clone)]
pub struct LintArgs {
    /// Script or prompt text. If --text-file is set, this is a path to a UTF-8 text file.
    pub text: String,

    /// Treat the positional text as a file path.
    #[arg(long)]
    pub text_file: bool,

    /// Names expected in multi-speaker transcript lines, for example Host or Guest.
    #[arg(long, value_name = "NAME")]
    pub speaker: Vec<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Auto,
    Wav,
    Pcm,
    Mp3,
    M4a,
    Flac,
}

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Wav => write!(f, "wav"),
            Self::Pcm => write!(f, "pcm"),
            Self::Mp3 => write!(f, "mp3"),
            Self::M4a => write!(f, "m4a"),
            Self::Flac => write!(f, "flac"),
        }
    }
}

#[derive(Subcommand)]
pub enum VoicesAction {
    /// List all built-in voice names with practical descriptions
    List {
        /// Filter voice names/descriptions.
        #[arg(long)]
        query: Option<String>,
    },
    /// Recommend voices from a natural-language brief
    Recommend {
        /// Brief such as "warm audiobook", "excited podcast", or "serious medical explainer".
        brief: String,
        /// Number of voices to return.
        #[arg(short, long, default_value = "5")]
        count: usize,
    },
}

#[derive(Subcommand)]
pub enum TagsAction {
    /// List useful inline tags grouped by category
    List {
        /// Filter by category.
        #[arg(long, value_enum)]
        category: Option<TagCategory>,
    },
    /// Search tags by word or use case
    Search {
        /// Search query.
        query: String,
    },
    /// Show ready-to-use tag/script recipes for common use cases
    Recipes,
}

#[derive(Clone, Copy, Debug, ValueEnum, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum TagCategory {
    Emotion,
    Pace,
    Volume,
    Pause,
    Nonverbal,
    Character,
    Accent,
}

#[derive(Subcommand)]
pub enum LanguagesAction {
    /// List supported language and locale hints
    List {
        /// Filter language names or codes.
        #[arg(long)]
        query: Option<String>,
    },
}

#[derive(Args)]
pub struct DoctorArgs {
    /// Make a small live Gemini TTS request to verify audio generation.
    #[arg(long)]
    pub live: bool,

    /// Also require ffmpeg for compressed output formats.
    #[arg(long)]
    pub require_ffmpeg: bool,
}

#[derive(Subcommand)]
pub enum AuthAction {
    /// Save an API key into ~/.config/gemini-tts-cli/config.toml
    Set {
        /// Gemini API key. Prefer GEMINI_API_KEY=... gemini-tts-cli auth import-env for shell history safety.
        #[arg(long)]
        api_key: String,
    },
    /// Import API key from GEMINI_API_KEY, GOOGLE_API_KEY, or GOOGLE_AI_API_KEY
    ImportEnv,
    /// Show whether an API key is configured, with the value masked
    Status,
}

#[derive(Subcommand)]
pub enum SkillAction {
    /// Write skill file to all detected agent platforms
    Install,
    /// Check which platforms have the skill installed
    Status,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Display effective merged configuration, masking secrets
    Show,
    /// Print configuration file path
    Path,
    /// Write a default config file if one does not exist
    Init,
    /// Set a supported config key
    Set {
        /// Key path, for example keys.api_key, defaults.voice, defaults.model.
        key: String,
        /// Value to write.
        value: String,
    },
    /// Get a supported config key, masking secrets
    Get {
        /// Key path.
        key: String,
    },
}
