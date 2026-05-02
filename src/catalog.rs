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

pub fn tags() -> Vec<Tag> {
    use TagCategory::*;
    vec![
        Tag {
            tag: "[excitedly]",
            category: Emotion,
            use_for: "positive energy and launch momentum",
            example: "[excitedly] This is the part you have been waiting for.",
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
    vec![
        Language {
            code: "af-za",
            name: "Afrikaans",
            hint: "Use English tags with Afrikaans transcript.",
        },
        Language {
            code: "am-et",
            name: "Amharic",
            hint: "Keep tags in English for best control.",
        },
        Language {
            code: "ar-001",
            name: "Arabic",
            hint: "General Arabic locale hint.",
        },
        Language {
            code: "ar-eg",
            name: "Arabic (Egypt)",
            hint: "Specify Egyptian Arabic if dialect matters.",
        },
        Language {
            code: "az-az",
            name: "Azerbaijani",
            hint: "Use a clear language note in director notes.",
        },
        Language {
            code: "bg-bg",
            name: "Bulgarian",
            hint: "Use English control tags.",
        },
        Language {
            code: "bn-bd",
            name: "Bengali",
            hint: "Use English tags with Bengali text.",
        },
        Language {
            code: "ca-es",
            name: "Catalan",
            hint: "Add region if accent matters.",
        },
        Language {
            code: "cmn-cn",
            name: "Chinese Mandarin (China)",
            hint: "Use Mandarin Chinese transcript.",
        },
        Language {
            code: "cmn-tw",
            name: "Chinese Mandarin (Taiwan)",
            hint: "Use Taiwan Mandarin phrasing.",
        },
        Language {
            code: "cs-cz",
            name: "Czech",
            hint: "Use English tags.",
        },
        Language {
            code: "da-dk",
            name: "Danish",
            hint: "Add Danish language note.",
        },
        Language {
            code: "de-de",
            name: "German",
            hint: "Specify standard German or regional accent.",
        },
        Language {
            code: "el-gr",
            name: "Greek",
            hint: "Use Greek transcript with English tags.",
        },
        Language {
            code: "en-au",
            name: "English (Australia)",
            hint: "Useful for Australian accent direction.",
        },
        Language {
            code: "en-gb",
            name: "English (United Kingdom)",
            hint: "Use with specific accent notes such as London or Croydon.",
        },
        Language {
            code: "en-in",
            name: "English (India)",
            hint: "Use with Indian English accent direction.",
        },
        Language {
            code: "en-us",
            name: "English (United States)",
            hint: "Default US English hint.",
        },
        Language {
            code: "es-419",
            name: "Spanish (Latin America)",
            hint: "General Latin American Spanish.",
        },
        Language {
            code: "es-es",
            name: "Spanish (Spain)",
            hint: "Castilian Spanish direction.",
        },
        Language {
            code: "es-mx",
            name: "Spanish (Mexico)",
            hint: "Mexican Spanish direction.",
        },
        Language {
            code: "es-us",
            name: "Spanish (United States)",
            hint: "US Spanish direction.",
        },
        Language {
            code: "fa-ir",
            name: "Persian",
            hint: "Use Persian transcript with English tags.",
        },
        Language {
            code: "fi-fi",
            name: "Finnish",
            hint: "Use English tags.",
        },
        Language {
            code: "fil-ph",
            name: "Filipino",
            hint: "Specify Filipino or Tagalog phrasing.",
        },
        Language {
            code: "fr-ca",
            name: "French (Canada)",
            hint: "Canadian French direction.",
        },
        Language {
            code: "fr-fr",
            name: "French (France)",
            hint: "French from France direction.",
        },
        Language {
            code: "he-il",
            name: "Hebrew",
            hint: "Use Hebrew transcript with English tags.",
        },
        Language {
            code: "hi-in",
            name: "Hindi",
            hint: "Hindi transcript with English tags.",
        },
        Language {
            code: "id-id",
            name: "Indonesian",
            hint: "Use Indonesian transcript.",
        },
        Language {
            code: "it-it",
            name: "Italian",
            hint: "Standard Italian direction.",
        },
        Language {
            code: "ja-jp",
            name: "Japanese",
            hint: "Japanese transcript with English tags.",
        },
        Language {
            code: "ko-kr",
            name: "Korean",
            hint: "Korean transcript with English tags.",
        },
        Language {
            code: "nl-nl",
            name: "Dutch",
            hint: "Dutch transcript.",
        },
        Language {
            code: "pl-pl",
            name: "Polish",
            hint: "Polish transcript.",
        },
        Language {
            code: "pt-br",
            name: "Portuguese (Brazil)",
            hint: "Brazilian Portuguese direction.",
        },
        Language {
            code: "pt-pt",
            name: "Portuguese (Portugal)",
            hint: "European Portuguese direction.",
        },
        Language {
            code: "ro-ro",
            name: "Romanian",
            hint: "Romanian transcript.",
        },
        Language {
            code: "ru-ru",
            name: "Russian",
            hint: "Russian transcript.",
        },
        Language {
            code: "sv-se",
            name: "Swedish",
            hint: "Swedish transcript.",
        },
        Language {
            code: "ta-in",
            name: "Tamil",
            hint: "Tamil transcript with English tags.",
        },
        Language {
            code: "te-in",
            name: "Telugu",
            hint: "Telugu transcript with English tags.",
        },
        Language {
            code: "th-th",
            name: "Thai",
            hint: "Thai transcript with English tags.",
        },
        Language {
            code: "tr-tr",
            name: "Turkish",
            hint: "Turkish transcript.",
        },
        Language {
            code: "uk-ua",
            name: "Ukrainian",
            hint: "Ukrainian transcript.",
        },
        Language {
            code: "ur-pk",
            name: "Urdu",
            hint: "Urdu transcript with English tags.",
        },
        Language {
            code: "vi-vn",
            name: "Vietnamese",
            hint: "Vietnamese transcript.",
        },
    ]
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
