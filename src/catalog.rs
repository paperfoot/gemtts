use serde::Serialize;

use crate::cli::TagCategory;

#[derive(Debug, Clone, Serialize)]
pub struct Voice {
    pub name: &'static str,
    pub character: &'static str,
    pub best_for: &'static str,
    pub notes: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub tag: &'static str,
    pub category: TagCategory,
    pub use_for: &'static str,
    pub example: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Language {
    pub code: &'static str,
    pub name: &'static str,
    pub hint: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Recipe {
    pub name: &'static str,
    pub use_case: &'static str,
    pub voice_hints: &'static [&'static str],
    pub prompt: &'static str,
}

pub fn voices() -> Vec<Voice> {
    vec![
        Voice {
            name: "Achernar",
            character: "soft",
            best_for: "intimate narration, sleep, calm explanations",
            notes: "Good when harsh energy would be distracting.",
        },
        Voice {
            name: "Achird",
            character: "friendly",
            best_for: "warm assistants, explainers, community updates",
            notes: "Approachable and plain-spoken.",
        },
        Voice {
            name: "Algenib",
            character: "gravelly",
            best_for: "dramatic reads, trailers, characterful narration",
            notes: "Useful for grit, noir, and texture.",
        },
        Voice {
            name: "Algieba",
            character: "smooth",
            best_for: "premium brand reads, polished voiceover",
            notes: "Controlled and easy to listen to.",
        },
        Voice {
            name: "Alnilam",
            character: "firm",
            best_for: "instructions, warnings, operational narration",
            notes: "Use when authority matters more than warmth.",
        },
        Voice {
            name: "Aoede",
            character: "breezy",
            best_for: "lifestyle, casual walkthroughs, upbeat narration",
            notes: "Light and open without too much intensity.",
        },
        Voice {
            name: "Autonoe",
            character: "bright",
            best_for: "short tutorials, product updates, energetic notes",
            notes: "Good default for positive clarity.",
        },
        Voice {
            name: "Callirrhoe",
            character: "easy-going",
            best_for: "conversational scripts, relaxed explainers",
            notes: "Useful for natural unscripted-feeling reads.",
        },
        Voice {
            name: "Charon",
            character: "informative",
            best_for: "news, documentary, factual narration",
            notes: "Steady information delivery.",
        },
        Voice {
            name: "Despina",
            character: "smooth",
            best_for: "audiobook, reflective essays, calm persuasion",
            notes: "Works well with slower pacing.",
        },
        Voice {
            name: "Enceladus",
            character: "breathy",
            best_for: "tired, intimate, vulnerable, soft-spoken reads",
            notes: "Pairs well with tired/bored/whispered directions.",
        },
        Voice {
            name: "Erinome",
            character: "clear",
            best_for: "technical explainers, agent output, educational narration",
            notes: "Prioritize this when comprehension matters.",
        },
        Voice {
            name: "Fenrir",
            character: "excitable",
            best_for: "promos, hooks, high-energy character lines",
            notes: "Can carry excitement without many tags.",
        },
        Voice {
            name: "Gacrux",
            character: "mature",
            best_for: "serious narration, trust-building, institutional audio",
            notes: "Use for gravitas and restraint.",
        },
        Voice {
            name: "Iapetus",
            character: "clear",
            best_for: "instructions, training, summaries, documentation",
            notes: "Reliable for dense material.",
        },
        Voice {
            name: "Kore",
            character: "firm",
            best_for: "default agent output, concise narration, confident lines",
            notes: "Strong default when you do not know the desired persona.",
        },
        Voice {
            name: "Laomedeia",
            character: "upbeat",
            best_for: "podcast intros, ads, friendly announcements",
            notes: "Good with excited and cheerful tags.",
        },
        Voice {
            name: "Leda",
            character: "youthful",
            best_for: "casual characters, playful explainers, social scripts",
            notes: "Best when the text itself sounds young and informal.",
        },
        Voice {
            name: "Orus",
            character: "firm",
            best_for: "direct instructions, crisp authoritative reads",
            notes: "Similar use case to Kore and Alnilam.",
        },
        Voice {
            name: "Puck",
            character: "upbeat",
            best_for: "energetic dialogue, excited hosts, quick demos",
            notes: "Strong for happy/excited contrast in multi-speaker scripts.",
        },
        Voice {
            name: "Pulcherrima",
            character: "forward",
            best_for: "confident promos, clear calls to action",
            notes: "Good when the read should lead the listener.",
        },
        Voice {
            name: "Rasalgethi",
            character: "informative",
            best_for: "documentary, factual scripts, guided explainers",
            notes: "Use for neutral authority.",
        },
        Voice {
            name: "Sadachbia",
            character: "lively",
            best_for: "dynamic explainers, social clips, upbeat teaching",
            notes: "Good for motion without sounding frantic.",
        },
        Voice {
            name: "Sadaltager",
            character: "knowledgeable",
            best_for: "expert narration, lectures, technical material",
            notes: "Good for credible specialist voices.",
        },
        Voice {
            name: "Schedar",
            character: "even",
            best_for: "long-form narration, audiobooks, stable reads",
            notes: "Use when consistency is more important than color.",
        },
        Voice {
            name: "Sulafat",
            character: "warm",
            best_for: "reassuring explanations, patient-facing content",
            notes: "Good with empathy and calm tags.",
        },
        Voice {
            name: "Umbriel",
            character: "easy-going",
            best_for: "casual assistant, low-friction narration",
            notes: "Works well for simple status updates.",
        },
        Voice {
            name: "Vindemiatrix",
            character: "gentle",
            best_for: "careful guidance, reflective narration",
            notes: "Good for sensitive or supportive scripts.",
        },
        Voice {
            name: "Zephyr",
            character: "bright",
            best_for: "short upbeat messages, crisp announcements",
            notes: "A good alternative to Puck for lighter reads.",
        },
        Voice {
            name: "Zubenelgenubi",
            character: "casual",
            best_for: "natural social reads, relaxed demos",
            notes: "Use when a polished narrator would feel too stiff.",
        },
    ]
}

pub fn voice_names() -> Vec<&'static str> {
    voices().into_iter().map(|voice| voice.name).collect()
}

pub fn canonical_voice_name(name: &str) -> Option<&'static str> {
    let trimmed = name.trim();
    voices()
        .into_iter()
        .find(|voice| voice.name.eq_ignore_ascii_case(trimmed))
        .map(|voice| voice.name)
}

pub fn tags() -> Vec<Tag> {
    use TagCategory::*;
    vec![
        Tag {
            tag: "[amazed]",
            category: Emotion,
            use_for: "wonder, impressed surprise, reveal moments",
            example: "[amazed] I did not expect the result to sound this natural.",
        },
        Tag {
            tag: "[curious]",
            category: Emotion,
            use_for: "inquiring, open-ended, exploratory delivery",
            example: "[curious] What happens if we try the Italian line next?",
        },
        Tag {
            tag: "[excited]",
            category: Emotion,
            use_for: "documented positive excitement tag",
            example: "[excited] The audio is ready.",
        },
        Tag {
            tag: "[excitedly]",
            category: Emotion,
            use_for: "positive energy and launch momentum",
            example: "[excitedly] This is the part you have been waiting for.",
        },
        Tag {
            tag: "[bored]",
            category: Emotion,
            use_for: "flat boredom or reluctant attention",
            example: "[bored] Another status update.",
        },
        Tag {
            tag: "[reluctantly]",
            category: Emotion,
            use_for: "hesitation, resistance, or unwilling admission",
            example: "[reluctantly] Fine, the first version was not tested enough.",
        },
        Tag {
            tag: "[warmly]",
            category: Emotion,
            use_for: "kind, human, reassuring delivery",
            example: "[warmly] You did the right thing by checking first.",
        },
        Tag {
            tag: "[serious]",
            category: Emotion,
            use_for: "remove playfulness and focus attention",
            example: "[serious] This next step matters.",
        },
        Tag {
            tag: "[sarcastically]",
            category: Emotion,
            use_for: "dry emphasis or comedic contrast",
            example: "[sarcastically] Brilliant, another login screen.",
        },
        Tag {
            tag: "[sarcastic]",
            category: Emotion,
            use_for: "documented concise sarcasm tag",
            example: "[sarcastic] Perfect, the one command nobody tested.",
        },
        Tag {
            tag: "[mischievously]",
            category: Emotion,
            use_for: "playful reveal or character line",
            example: "[mischievously] I may have one more trick.",
        },
        Tag {
            tag: "[panicked]",
            category: Emotion,
            use_for: "high urgency, alarm, frantic dialogue",
            example: "[panicked] Wait, stop the upload.",
        },
        Tag {
            tag: "[tired]",
            category: Emotion,
            use_for: "fatigue, boredom, end-of-day delivery",
            example: "[tired] We can fix the build in the morning.",
        },
        Tag {
            tag: "[trembling]",
            category: Emotion,
            use_for: "fear, fragility, or unstable emotion",
            example: "[trembling] I heard the alert again.",
        },
        Tag {
            tag: "[crying]",
            category: Emotion,
            use_for: "sadness or overwhelmed character dialogue",
            example: "[crying] I did not think anyone remembered.",
        },
        Tag {
            tag: "[very fast]",
            category: Pace,
            use_for: "rapid promo, urgency, comedic speed",
            example: "[very fast] Terms apply, check the repo, ship the patch.",
        },
        Tag {
            tag: "[very slow]",
            category: Pace,
            use_for: "dramatic emphasis or careful instruction",
            example: "[very slow] Do not delete the production database.",
        },
        Tag {
            tag: "[one painfully slow word at a time]",
            category: Pace,
            use_for: "comic frustration or heavy emphasis",
            example: "[one painfully slow word at a time] Read. The. Error. Message.",
        },
        Tag {
            tag: "[shouting]",
            category: Volume,
            use_for: "projected emphasis, not necessarily anger",
            example: "[shouting] Turn this up.",
        },
        Tag {
            tag: "[whispers]",
            category: Volume,
            use_for: "quiet aside, suspense, intimacy",
            example: "[whispers] This is the hidden part.",
        },
        Tag {
            tag: "[whisper]",
            category: Volume,
            use_for: "documented singular whisper form",
            example: "[whisper] Keep this part quiet.",
        },
        Tag {
            tag: "[softly]",
            category: Volume,
            use_for: "gentle or close-mic delivery",
            example: "[softly] Take a breath before the next step.",
        },
        Tag {
            tag: "[short pause]",
            category: Pause,
            use_for: "small rhythmic pause",
            example: "First, check the key. [short pause] Then run doctor.",
        },
        Tag {
            tag: "[medium pause]",
            category: Pause,
            use_for: "sentence-level separation",
            example: "The audio is ready. [medium pause] Now listen once.",
        },
        Tag {
            tag: "[long pause]",
            category: Pause,
            use_for: "dramatic beat or section break",
            example: "Everything went quiet. [long pause] Then the alert fired.",
        },
        Tag {
            tag: "[laughs]",
            category: Nonverbal,
            use_for: "humanizing amusement",
            example: "[laughs] No, that is not how OAuth works.",
        },
        Tag {
            tag: "[giggles]",
            category: Nonverbal,
            use_for: "light, playful amusement",
            example: "[giggles] Fine, that one was actually clever.",
        },
        Tag {
            tag: "[sighs]",
            category: Nonverbal,
            use_for: "resignation, fatigue, realism",
            example: "[sighs] We need to update the token again.",
        },
        Tag {
            tag: "[gasp]",
            category: Nonverbal,
            use_for: "surprise or sudden realization",
            example: "[gasp] The preview deploy is public.",
        },
        Tag {
            tag: "[yawn]",
            category: Nonverbal,
            use_for: "tiredness, boredom, or sleepy character delivery",
            example: "[yawn] What is on the agenda today?",
        },
        Tag {
            tag: "[cough]",
            category: Nonverbal,
            use_for: "awkwardness, interruption, character texture",
            example: "[cough] Maybe do not ship that comment.",
        },
        Tag {
            tag: "[like a radio DJ]",
            category: Character,
            use_for: "high-energy announcer delivery",
            example: "[like a radio DJ] Good morning London.",
        },
        Tag {
            tag: "[like a documentary narrator]",
            category: Character,
            use_for: "measured informative storytelling",
            example: "[like a documentary narrator] The first commit seemed harmless.",
        },
        Tag {
            tag: "[like dracula]",
            category: Character,
            use_for: "gothic or theatrical character voice",
            example: "[like dracula] Welcome to the production logs.",
        },
        Tag {
            tag: "[British English accent]",
            category: Accent,
            use_for: "broad accent steer; specific accents work better",
            example: "[British English accent] The schedule has changed.",
        },
        Tag {
            tag: "[Croydon, England accent]",
            category: Accent,
            use_for: "specific regional accent direction",
            example: "[Croydon, England accent] Meet me at the station.",
        },
        Tag {
            tag: "[Southern California accent]",
            category: Accent,
            use_for: "bright US regional delivery",
            example: "[Southern California accent] That update is actually kind of perfect.",
        },
    ]
}

pub fn languages() -> Vec<Language> {
    let entries = [
        ("ar", "Arabic"),
        ("bn", "Bangla"),
        ("nl", "Dutch"),
        ("en", "English"),
        ("fr", "French"),
        ("de", "German"),
        ("hi", "Hindi"),
        ("id", "Indonesian"),
        ("it", "Italian"),
        ("ja", "Japanese"),
        ("ko", "Korean"),
        ("mr", "Marathi"),
        ("pl", "Polish"),
        ("pt", "Portuguese"),
        ("ro", "Romanian"),
        ("ru", "Russian"),
        ("es", "Spanish"),
        ("ta", "Tamil"),
        ("te", "Telugu"),
        ("th", "Thai"),
        ("tr", "Turkish"),
        ("uk", "Ukrainian"),
        ("vi", "Vietnamese"),
        ("af", "Afrikaans"),
        ("sq", "Albanian"),
        ("am", "Amharic"),
        ("hy", "Armenian"),
        ("az", "Azerbaijani"),
        ("eu", "Basque"),
        ("be", "Belarusian"),
        ("bg", "Bulgarian"),
        ("my", "Burmese"),
        ("ca", "Catalan"),
        ("ceb", "Cebuano"),
        ("cmn", "Chinese, Mandarin"),
        ("hr", "Croatian"),
        ("cs", "Czech"),
        ("da", "Danish"),
        ("et", "Estonian"),
        ("fil", "Filipino"),
        ("fi", "Finnish"),
        ("gl", "Galician"),
        ("ka", "Georgian"),
        ("el", "Greek"),
        ("gu", "Gujarati"),
        ("ht", "Haitian Creole"),
        ("he", "Hebrew"),
        ("hu", "Hungarian"),
        ("is", "Icelandic"),
        ("jv", "Javanese"),
        ("kn", "Kannada"),
        ("kok", "Konkani"),
        ("lo", "Lao"),
        ("la", "Latin"),
        ("lv", "Latvian"),
        ("lt", "Lithuanian"),
        ("lb", "Luxembourgish"),
        ("mk", "Macedonian"),
        ("mai", "Maithili"),
        ("mg", "Malagasy"),
        ("ms", "Malay"),
        ("ml", "Malayalam"),
        ("mn", "Mongolian"),
        ("ne", "Nepali"),
        ("nb", "Norwegian, Bokmal"),
        ("nn", "Norwegian, Nynorsk"),
        ("or", "Odia"),
        ("ps", "Pashto"),
        ("fa", "Persian"),
        ("pa", "Punjabi"),
        ("sr", "Serbian"),
        ("sd", "Sindhi"),
        ("si", "Sinhala"),
        ("sk", "Slovak"),
        ("sl", "Slovenian"),
        ("sw", "Swahili"),
        ("sv", "Swedish"),
        ("ur", "Urdu"),
    ];

    entries
        .into_iter()
        .map(|(code, name)| Language {
            code,
            name,
            hint: language_hint(code),
        })
        .collect()
}

fn language_hint(code: &str) -> &'static str {
    match code {
        "it" => {
            "Official Gemini TTS code is it. There are no Italian-specific voice IDs; use an Italian transcript plus accent/style direction."
        }
        "en" => {
            "Official Gemini TTS code is en. Add accent direction when regional English matters."
        }
        "cmn" => {
            "Official Gemini TTS code is cmn for Mandarin Chinese. Use Mandarin transcript and English tags."
        }
        _ => {
            "Official Gemini TTS language code. The model auto-detects transcript language; keep inline audio tags in English for best control."
        }
    }
}

pub fn recipes() -> Vec<Recipe> {
    vec![
        Recipe {
            name: "agent-status",
            use_case: "Short agent status update that sounds clear but not theatrical.",
            voice_hints: &["Kore", "Erinome", "Iapetus"],
            prompt: "# AUDIO PROFILE: Precise agent narrator\n### DIRECTOR'S NOTES\nStyle: calm, useful, concise.\nPacing: medium-fast, no drama.\nAccent: neutral.\n#### TRANSCRIPT\n[clearly] Build passed. [short pause] Two checks still need review.",
        },
        Recipe {
            name: "podcast-dialogue",
            use_case: "Two-speaker podcast or explainer conversation.",
            voice_hints: &["Kore", "Puck", "Laomedeia"],
            prompt: "Use --speaker Host=Kore --speaker Guest=Puck and format transcript lines as Host: ... / Guest: ...",
        },
        Recipe {
            name: "audiobook",
            use_case: "Long-form narration where stability matters.",
            voice_hints: &["Schedar", "Despina", "Achernar"],
            prompt: "Style: grounded, intimate, emotionally restrained. Pacing: slower than normal with clean paragraph pauses. Tags: [softly], [long pause], [whispers] only at key moments.",
        },
        Recipe {
            name: "promo",
            use_case: "Fast short-form announcement or product launch.",
            voice_hints: &["Fenrir", "Puck", "Zephyr", "Pulcherrima"],
            prompt: "Style: infectious enthusiasm and vocal smile. Pacing: fast, punchy, no dead air. Tags: [excitedly], [shouting], [very fast], [short pause].",
        },
        Recipe {
            name: "sensitive-guidance",
            use_case: "Medical, personal, or careful support content.",
            voice_hints: &["Sulafat", "Vindemiatrix", "Achernar"],
            prompt: "Style: warm, careful, non-judgmental. Pacing: slow enough to process. Avoid jokes. Tags: [warmly], [softly], [short pause].",
        },
    ]
}

pub fn filter_text(haystack: &str, query: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&query.to_ascii_lowercase())
}

pub fn recommend_voices(brief: &str, count: usize) -> Vec<Voice> {
    let words: Vec<String> = brief
        .to_ascii_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect();

    let mut scored: Vec<(i32, Voice)> = voices()
        .into_iter()
        .map(|voice| {
            let blob = format!(
                "{} {} {} {}",
                voice.name, voice.character, voice.best_for, voice.notes
            )
            .to_ascii_lowercase();
            let mut score = 0;
            for word in &words {
                if blob.contains(word) {
                    score += 3;
                }
            }
            score += heuristic_score(&blob, &words);
            (score, voice)
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.name.cmp(b.1.name)));
    scored
        .into_iter()
        .filter(|(score, _)| *score > 0)
        .map(|(_, voice)| voice)
        .take(count.max(1))
        .collect()
}

fn heuristic_score(blob: &str, words: &[String]) -> i32 {
    let mut score = 0;
    for word in words {
        let word = word.as_str();
        if matches!(
            word,
            "happy" | "excited" | "energetic" | "podcast" | "promo" | "launch"
        ) && any_contains(blob, &["upbeat", "excitable", "bright", "lively"])
        {
            score += 4;
        }
        if matches!(
            word,
            "calm" | "sleep" | "soft" | "gentle" | "warm" | "sensitive"
        ) && any_contains(blob, &["soft", "gentle", "warm", "smooth"])
        {
            score += 4;
        }
        if matches!(
            word,
            "technical" | "agent" | "clear" | "training" | "doctor" | "medical"
        ) && any_contains(blob, &["clear", "informative", "knowledgeable", "firm"])
        {
            score += 4;
        }
        if matches!(word, "dramatic" | "trailer" | "noir" | "character")
            && any_contains(blob, &["gravelly", "mature", "forward"])
        {
            score += 4;
        }
    }
    score
}

fn any_contains(blob: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| blob.contains(needle))
}
