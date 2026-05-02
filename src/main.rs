//! Agent-friendly gemtts built with agent-cli-framework.
//!
//! Demonstrates every pattern from the framework:
//!   - Modular structure (cli, config, error, output, commands/)
//!   - JSON envelope on stdout, coloured table on TTY
//!   - Semantic exit codes (0-4)
//!   - `--quiet` to suppress informational output
//!   - `agent-info` for machine-readable capability discovery
//!   - `config show/path` for configuration management
//!   - `skill install` to register with AI agent platforms
//!   - `doctor` for API/ffmpeg diagnostics
//!   - expressive TTS script, voice, tag, and audio generation commands
//!   - `update` for self-update via GitHub Releases

mod audio;
mod catalog;
mod cli;
mod commands;
mod config;
mod error;
mod gemini;
mod guard;
mod output;
mod prompt;

use clap::Parser;

use cli::{AuthAction, Cli, Commands, ConfigAction, SkillAction};
use output::{Ctx, Format};

/// Pre-scan argv for --json before clap parses. This ensures --json is
/// honored even on help, version, and parse-error paths where clap hasn't
/// populated the Cli struct yet.
fn has_json_flag() -> bool {
    std::env::args_os().any(|a| a == "--json")
}

fn main() {
    let json_flag = has_json_flag();

    // Use try_parse so clap errors go through the JSON envelope instead of
    // printing human-only text that breaks agent pipelines.
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            // Help and --version are NOT errors. Exit 0.
            if matches!(
                e.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            ) {
                let format = Format::detect(json_flag);
                match format {
                    Format::Json => {
                        output::print_help_json(e);
                        std::process::exit(0);
                    }
                    Format::Human => e.exit(), // clap prints coloured help, exits 0
                }
            }

            // Actual parse errors -- always exit 3, never let clap own the exit.
            let format = Format::detect(json_flag);
            output::print_clap_error(format, &e);
            std::process::exit(3);
        }
    };

    let ctx = Ctx::new(cli.json, cli.quiet);

    // Config is loaded lazily -- only commands that need it call config::load().
    // This ensures agent-info, config path, skill, and offline catalog commands work,
    // even when config.toml is malformed.
    let result = match cli.command {
        Commands::Speak(args) => {
            config::load().and_then(|cfg| commands::tts::speak(ctx, args, &cfg))
        }
        Commands::Script(args) => {
            config::load().and_then(|cfg| commands::tts::script(ctx, args, &cfg))
        }
        Commands::Lint(args) => commands::tts::lint(ctx, args),
        Commands::Voices { action } => commands::catalog::voices(ctx, action),
        Commands::Tags { action } => commands::catalog::tags(ctx, action),
        Commands::Languages { action } => commands::catalog::languages(ctx, action),
        Commands::Doctor(args) => config::load()
            .and_then(|cfg| commands::tts::doctor(ctx, args.live, args.require_ffmpeg, &cfg)),
        Commands::Auth { action } => match action {
            AuthAction::Set { api_key } => commands::auth::set(ctx, api_key),
            AuthAction::ImportEnv => commands::auth::import_env(ctx),
            AuthAction::Status => config::load().and_then(|cfg| commands::auth::status(ctx, &cfg)),
        },
        Commands::AgentInfo => {
            commands::agent_info::run();
            Ok(())
        }
        Commands::Skill { action } => match action {
            SkillAction::Install => commands::skill::install(ctx),
            SkillAction::Status => commands::skill::status(ctx),
        },
        Commands::Config { action } => match action {
            ConfigAction::Show => config::load().and_then(|cfg| commands::config::show(ctx, &cfg)),
            ConfigAction::Path => commands::config::path(ctx),
            ConfigAction::Init => commands::config::init(ctx),
            ConfigAction::Set { key, value } => commands::config::set(ctx, key, value),
            ConfigAction::Get { key } => {
                config::load().and_then(|cfg| commands::config::get(ctx, &cfg, key))
            }
        },
        Commands::Update { check } => {
            config::load().and_then(|cfg| commands::update::run(ctx, check, &cfg))
        }
        Commands::Contract { code } => commands::contract::run(ctx, code),
    };

    if let Err(e) = result {
        output::print_error(ctx.format, &e);
        std::process::exit(e.exit_code());
    }
}
