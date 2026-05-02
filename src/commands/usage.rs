use serde::Serialize;

use crate::cli::UsageAction;
use crate::error::AppError;
use crate::output::{self, Ctx};
use crate::usage;

#[derive(Serialize)]
struct UsageListResult {
    records: Vec<usage::UsageRecord>,
    summary: usage::UsageSummary,
}

#[derive(Serialize)]
struct UsagePathResult {
    path: String,
}

pub fn run(ctx: Ctx, action: UsageAction) -> Result<(), AppError> {
    match action {
        UsageAction::Summary => summary(ctx),
        UsageAction::List { limit } => list(ctx, limit),
        UsageAction::Path => path(ctx),
    }
}

fn summary(ctx: Ctx) -> Result<(), AppError> {
    let records = usage::load_records()?;
    let summary = usage::summarize(&records);
    output::print_success_or(ctx, &summary, |s| {
        println!("usage records: {}", s.records);
        println!(
            "audio: {:.2}s, {} tokens",
            s.audio_seconds, s.audio_output_tokens
        );
        println!("input tokens: {}", s.input_tokens);
        println!("estimated total: ${:.6}", s.total_cost_usd);
        println!("ledger: {}", s.path);
    });
    Ok(())
}

fn list(ctx: Ctx, limit: usize) -> Result<(), AppError> {
    let records = usage::load_records()?;
    let start = records.len().saturating_sub(limit);
    let selected = records[start..].to_vec();
    let result = UsageListResult {
        summary: usage::summarize(&records),
        records: selected,
    };
    output::print_success_or(ctx, &result, |r| {
        for record in &r.records {
            println!(
                "{} ${:.6} {:.2}s {}",
                record.output_path,
                record.estimate.total_cost_usd,
                record.estimate.audio_seconds,
                record.model
            );
        }
        println!("estimated total: ${:.6}", r.summary.total_cost_usd);
    });
    Ok(())
}

fn path(ctx: Ctx) -> Result<(), AppError> {
    let result = UsagePathResult {
        path: usage::usage_path().display().to_string(),
    };
    output::print_success_or(ctx, &result, |r| println!("{}", r.path));
    Ok(())
}
