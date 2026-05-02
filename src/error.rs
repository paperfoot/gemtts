/// Error types with semantic exit codes.
///
/// Every error maps to an exit code (1-4), a machine-readable code, and a
/// recovery suggestion that agents can follow literally.

#[derive(thiserror::Error, Debug)]
#[allow(dead_code)] // Some variants demonstrate the full exit code contract (0-4)
pub enum AppError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("{0}")]
    Transient(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Audio error: {0}")]
    Audio(String),

    #[error("Update failed: {0}")]
    Update(String),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidInput(_) => 3,
            Self::Config(_) => 2,
            Self::RateLimited(_) => 4,
            Self::Transient(_) | Self::Io(_) | Self::Http(_) | Self::Audio(_) | Self::Update(_) => {
                1
            }
        }
    }

    pub fn error_code(&self) -> &str {
        match self {
            Self::InvalidInput(_) => "invalid_input",
            Self::Config(_) => "config_error",
            Self::Transient(_) => "transient_error",
            Self::RateLimited(_) => "rate_limited",
            Self::Io(_) => "io_error",
            Self::Http(_) => "http_error",
            Self::Audio(_) => "audio_error",
            Self::Update(_) => "update_error",
        }
    }

    pub fn suggestion(&self) -> &str {
        match self {
            Self::InvalidInput(_) => {
                concat!("Check arguments with: ", env!("CARGO_PKG_NAME"), " --help")
            }
            Self::Config(_) => concat!(
                "Run: ",
                env!("CARGO_PKG_NAME"),
                " doctor --live, or set the key with ",
                env!("CARGO_PKG_NAME"),
                " auth import-env"
            ),
            Self::Transient(_) | Self::Io(_) | Self::Http(_) => "Retry the command",
            Self::Audio(_) => "Run doctor --require-ffmpeg, or use --format wav",
            Self::RateLimited(_) => "Wait a moment and retry",
            Self::Update(_) => concat!(
                "Retry later, or install manually via cargo install ",
                env!("CARGO_PKG_NAME")
            ),
        }
    }
}
