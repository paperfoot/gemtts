use serde::Serialize;

use crate::catalog;
use crate::cli::{LanguagesAction, TagsAction, VoicesAction};
use crate::error::AppError;
use crate::output::{self, Ctx};

pub fn voices(ctx: Ctx, action: VoicesAction) -> Result<(), AppError> {
    match action {
        VoicesAction::List { query } => {
            let mut voices = catalog::voices();
            if let Some(query) = query {
                voices.retain(|v| {
                    catalog::filter_text(
                        &format!("{} {} {} {}", v.name, v.character, v.best_for, v.notes),
                        &query,
                    )
                });
            }
            output::print_success_or(ctx, &voices, |items| {
                let mut table = comfy_table::Table::new();
                table.set_header(vec!["Voice", "Character", "Best For", "Notes"]);
                for v in items {
                    table.add_row(vec![v.name, v.character, v.best_for, v.notes]);
                }
                println!("{table}");
            });
            Ok(())
        }
        VoicesAction::Recommend { brief, count } => {
            let mut voices = catalog::recommend_voices(&brief, count);
            if voices.is_empty() {
                voices = catalog::voices().into_iter().take(count.max(1)).collect();
            }
            let result = RecommendationResult { brief, voices };
            output::print_success_or(ctx, &result, |r| {
                let mut table = comfy_table::Table::new();
                table.set_header(vec!["Voice", "Character", "Why"]);
                for v in &r.voices {
                    table.add_row(vec![v.name, v.character, v.best_for]);
                }
                println!("{table}");
            });
            Ok(())
        }
    }
}

#[derive(Serialize)]
struct RecommendationResult {
    brief: String,
    voices: Vec<catalog::Voice>,
}

pub fn tags(ctx: Ctx, action: TagsAction) -> Result<(), AppError> {
    match action {
        TagsAction::List { category } => {
            let mut tags = catalog::tags();
            if let Some(category) = category {
                tags.retain(|t| t.category == category);
            }
            output::print_success_or(ctx, &tags, |items| {
                let mut table = comfy_table::Table::new();
                table.set_header(vec!["Tag", "Category", "Use For", "Example"]);
                for tag in items {
                    table.add_row(vec![
                        tag.tag.to_string(),
                        format!("{:?}", tag.category),
                        tag.use_for.to_string(),
                        tag.example.to_string(),
                    ]);
                }
                println!("{table}");
            });
            Ok(())
        }
        TagsAction::Search { query } => {
            let tags: Vec<_> = catalog::tags()
                .into_iter()
                .filter(|t| {
                    catalog::filter_text(
                        &format!("{} {:?} {} {}", t.tag, t.category, t.use_for, t.example),
                        &query,
                    )
                })
                .collect();
            let result = TagSearchResult { query, tags };
            output::print_success_or(ctx, &result, |r| {
                for tag in &r.tags {
                    println!("{} - {}", tag.tag, tag.use_for);
                }
            });
            Ok(())
        }
        TagsAction::Recipes => {
            let recipes = catalog::recipes();
            output::print_success_or(ctx, &recipes, |items| {
                let mut table = comfy_table::Table::new();
                table.set_header(vec!["Recipe", "Use Case", "Voices"]);
                for recipe in items {
                    table.add_row(vec![
                        recipe.name.to_string(),
                        recipe.use_case.to_string(),
                        recipe.voice_hints.join(", "),
                    ]);
                }
                println!("{table}");
            });
            Ok(())
        }
    }
}

#[derive(Serialize)]
struct TagSearchResult {
    query: String,
    tags: Vec<catalog::Tag>,
}

pub fn languages(ctx: Ctx, action: LanguagesAction) -> Result<(), AppError> {
    match action {
        LanguagesAction::List { query } => {
            let mut languages = catalog::languages();
            if let Some(query) = query {
                languages.retain(|l| {
                    catalog::filter_text(&format!("{} {} {}", l.code, l.name, l.hint), &query)
                });
            }
            output::print_success_or(ctx, &languages, |items| {
                let mut table = comfy_table::Table::new();
                table.set_header(vec!["Code", "Language", "Prompt Hint"]);
                for lang in items {
                    table.add_row(vec![lang.code, lang.name, lang.hint]);
                }
                println!("{table}");
            });
            Ok(())
        }
    }
}
