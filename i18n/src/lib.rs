//! Embedded localization for Smudgy's first-party client text.
//!
//! English is the authoritative fallback. The active locale is process-wide so
//! core-generated client feedback and every window use one consistent language.

use std::collections::HashMap;
use std::fmt;
use std::sync::{LazyLock, Mutex};
use std::sync::atomic::{AtomicU8, Ordering};

use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource};
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

const EN_US_SOURCE: &str = include_str!("../locales/en-US/main.ftl");
const ZH_TW_SOURCE: &str = include_str!("../locales/zh-TW/main.ftl");

/// The language choice persisted in `settings.json`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalePreference {
    /// Follow the desktop language, falling back to English when unsupported.
    #[default]
    System,
    English,
    TraditionalChinese,
}

impl LocalePreference {
    /// Every available choice in picker order.
    pub const ALL: [Self; 3] = [Self::System, Self::TraditionalChinese, Self::English];

    /// Stable BCP-47-like configuration value for diagnostics and CLI use.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::English => "en-US",
            Self::TraditionalChinese => "zh-TW",
        }
    }
}

impl fmt::Display for LocalePreference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = match self {
            Self::System => "locale-system",
            Self::English => "locale-english",
            Self::TraditionalChinese => "locale-traditional-chinese",
        };
        formatter.write_str(&translate(key))
    }
}

/// The concrete catalog currently used for rendering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Locale {
    #[default]
    English = 0,
    TraditionalChinese = 1,
}

impl Locale {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::English => "en-US",
            Self::TraditionalChinese => "zh-TW",
        }
    }
}

static ACTIVE_LOCALE: AtomicU8 = AtomicU8::new(Locale::English as u8);
static STATIC_TRANSLATIONS: LazyLock<Mutex<HashMap<(u8, String), &'static str>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static EN_US: LazyLock<FluentBundle<FluentResource>> =
    LazyLock::new(|| make_bundle("en-US", EN_US_SOURCE));
static ZH_TW: LazyLock<FluentBundle<FluentResource>> =
    LazyLock::new(|| make_bundle("zh-TW", ZH_TW_SOURCE));

fn make_bundle(tag: &str, source: &str) -> FluentBundle<FluentResource> {
    let language: LanguageIdentifier = tag.parse().expect("built-in locale tag is valid");
    let resource = FluentResource::try_new(source.to_string()).unwrap_or_else(|(_, errors)| {
        panic!("invalid built-in Fluent resource for {tag}: {errors:?}")
    });
    let mut bundle = FluentBundle::new_concurrent(vec![language]);
    // Smudgy renders terminal-style UI text where invisible bidi isolation
    // marks would leak into copy/paste and cursor offsets. The supported
    // catalogs are both left-to-right, so interpolation can stay literal.
    bundle.set_use_isolating(false);
    bundle
        .add_resource(resource)
        .unwrap_or_else(|errors| panic!("duplicate Fluent messages in {tag}: {errors:?}"));
    bundle
}

/// Activate a persisted preference before the first application window renders.
///
/// `SMUDGY_LOCALE` is an optional launch override. For `System`, `LANGUAGE` is
/// considered before the POSIX `LC_*` variables because desktop sessions may
/// deliberately carry `LC_ALL=C.UTF-8` while still declaring `zh_TW` as their
/// display language.
pub fn activate(preference: LocalePreference) -> Locale {
    let locale = std::env::var("SMUDGY_LOCALE")
        .ok()
        .and_then(|value| locale_from_tag(&value))
        .unwrap_or_else(|| match preference {
            LocalePreference::System => detect_system_locale(),
            LocalePreference::English => Locale::English,
            LocalePreference::TraditionalChinese => Locale::TraditionalChinese,
        });
    ACTIVE_LOCALE.store(locale as u8, Ordering::Release);
    locale
}

/// The resolved concrete locale currently in use.
#[must_use]
pub fn current_locale() -> Locale {
    match ACTIVE_LOCALE.load(Ordering::Acquire) {
        1 => Locale::TraditionalChinese,
        _ => Locale::English,
    }
}

/// Resolve supported desktop language variables from the current process.
#[must_use]
pub fn detect_system_locale() -> Locale {
    ["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"]
        .into_iter()
        .filter_map(|key| std::env::var(key).ok())
        .flat_map(|value| value.split(':').map(str::to_owned).collect::<Vec<String>>())
        .find_map(|tag| locale_from_tag(&tag))
        .unwrap_or(Locale::English)
}

/// Resolve a supported locale tag. Unsupported tags return `None`, allowing a
/// later item in a desktop fallback list to be considered.
#[must_use]
pub fn locale_from_tag(tag: &str) -> Option<Locale> {
    let normalized = tag
        .trim()
        .split(['.', '@'])
        .next()
        .unwrap_or_default()
        .replace('_', "-")
        .to_ascii_lowercase();

    if normalized == "en" || normalized.starts_with("en-") {
        return Some(Locale::English);
    }
    if matches!(normalized.as_str(), "zh-tw" | "zh-hk" | "zh-mo" | "zh-hant")
        || normalized.starts_with("zh-hant-")
    {
        return Some(Locale::TraditionalChinese);
    }
    None
}

/// Translate a message without variables, with English fallback.
#[must_use]
pub fn translate(id: &str) -> String {
    translate_with(id, &[])
}

/// Translate a non-interpolated message into a process-lifetime string.
///
/// Some Iced controls borrow placeholders instead of owning them. This cache
/// leaks each `(locale, message id)` at most once, so runtime locale switches
/// remain safe without leaking once per rendered frame.
#[must_use]
pub fn translate_static(id: &str) -> &'static str {
    let locale = current_locale() as u8;
    let key = (locale, id.to_string());
    let mut cache = STATIC_TRANSLATIONS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(value) = cache.get(&key) {
        return value;
    }
    let value: &'static str = Box::leak(translate(id).into_boxed_str());
    cache.insert(key, value);
    value
}

/// Translate a message with string variables, with English fallback.
#[must_use]
pub fn translate_with(id: &str, values: &[(&str, String)]) -> String {
    let mut args = FluentArgs::new();
    for (name, value) in values {
        args.set(*name, value.as_str());
    }

    let primary = match current_locale() {
        Locale::English => &*EN_US,
        Locale::TraditionalChinese => &*ZH_TW,
    };
    format_message(primary, id, &args)
        .or_else(|| format_message(&EN_US, id, &args))
        .unwrap_or_else(|| format!("⟦{id}⟧"))
}

fn format_message(
    bundle: &FluentBundle<FluentResource>,
    id: &str,
    args: &FluentArgs<'_>,
) -> Option<String> {
    let message = bundle.get_message(id)?;
    let pattern = message.value()?;
    let mut errors = Vec::new();
    let rendered = bundle.format_pattern(pattern, Some(args), &mut errors);
    errors.is_empty().then(|| rendered.into_owned())
}

/// Build a translated message while keeping call sites compact and variable
/// names explicit in the catalog.
#[macro_export]
macro_rules! t {
    ($id:literal $(,)?) => {
        $crate::translate($id)
    };
    ($id:literal, $($name:literal => $value:expr),+ $(,)?) => {{
        let values = [$(($name, ($value).to_string())),+];
        $crate::translate_with($id, &values)
    }};
}

/// Translate a non-interpolated message for an API that requires a borrowed
/// process-lifetime string.
#[macro_export]
macro_rules! ts {
    ($id:literal $(,)?) => {
        $crate::translate_static($id)
    };
}

#[cfg(test)]
fn catalog_ids(source: &str) -> std::collections::BTreeSet<&str> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim_end();
            if line.starts_with(char::is_whitespace) || line.starts_with('#') {
                return None;
            }
            line.split_once(" = ").map(|(id, _)| id)
        })
        .collect()
}

#[cfg(test)]
fn catalog_variables(
    source: &str,
) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    use regex::Regex;
    use std::collections::{BTreeMap, BTreeSet};

    let variable = Regex::new(r"\{\s*\$([A-Za-z][A-Za-z0-9_-]*)").unwrap();
    let mut messages: BTreeMap<String, String> = BTreeMap::new();
    let mut current: Option<String> = None;

    for line in source.lines() {
        if !line.starts_with(char::is_whitespace)
            && !line.starts_with('#')
            && let Some((id, value)) = line.split_once(" = ")
        {
            current = Some(id.to_string());
            messages.insert(id.to_string(), value.to_string());
            continue;
        }
        if line.starts_with(char::is_whitespace)
            && let Some(id) = &current
            && let Some(value) = messages.get_mut(id)
        {
            value.push('\n');
            value.push_str(line);
        }
    }

    messages
        .into_iter()
        .map(|(id, value)| {
            let variables: BTreeSet<String> = variable
                .captures_iter(&value)
                .map(|capture| capture[1].to_string())
                .collect();
            (id, variables)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::path::{Path, PathBuf};

    static LOCALE_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn locale_tags_distinguish_traditional_and_simplified_chinese() {
        assert_eq!(
            locale_from_tag("zh_TW.UTF-8"),
            Some(Locale::TraditionalChinese)
        );
        assert_eq!(
            locale_from_tag("zh-Hant-HK"),
            Some(Locale::TraditionalChinese)
        );
        assert_eq!(locale_from_tag("en_US.UTF-8"), Some(Locale::English));
        assert_eq!(locale_from_tag("zh_CN.UTF-8"), None);
        assert_eq!(locale_from_tag("C.UTF-8"), None);
    }

    #[test]
    fn traditional_chinese_catalog_matches_english_keys() {
        assert_eq!(catalog_ids(EN_US_SOURCE), catalog_ids(ZH_TW_SOURCE));
    }

    #[test]
    fn traditional_chinese_catalog_matches_english_variables() {
        assert_eq!(
            catalog_variables(EN_US_SOURCE),
            catalog_variables(ZH_TW_SOURCE)
        );
    }

    fn rust_sources(root: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<_> = std::fs::read_dir(root)
            .unwrap_or_else(|error| panic!("read {}: {error}", root.display()))
            .map(|entry| entry.expect("read source entry").path())
            .collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                rust_sources(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push(path);
            }
        }
    }

    fn workspace_sources() -> Vec<PathBuf> {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("i18n crate has workspace parent");
        let mut sources = Vec::new();
        for root in ["i18n/src", "ui/src", "core/src", "cloud/src", "widgets/src", "map_widget/src"] {
            rust_sources(&workspace.join(root), &mut sources);
        }
        sources
    }

    #[test]
    fn every_translation_macro_references_a_catalog_message() {
        let known = catalog_ids(EN_US_SOURCE);
        let invocation = Regex::new(r#"(?:smudgy_i18n::)?t(?:s)?!\s*\(\s*\"([a-z][a-z0-9-]*)\""#)
            .unwrap();
        let mut missing = Vec::new();
        for path in workspace_sources() {
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            for capture in invocation.captures_iter(&source) {
                let id = &capture[1];
                if !known.contains(id) {
                    missing.push(format!("{}: {id}", path.display()));
                }
            }
        }
        assert!(missing.is_empty(), "translation keys missing from catalogs:\n{}", missing.join("\n"));
    }

    #[test]
    fn first_party_visible_literals_are_classified() {
        // This intentionally targets display constructors and session-output methods rather than
        // every Rust string. Protocol tokens, logs, tests, paths, and user-authored content are
        // different ownership classes and must not be fed through the UI catalog.
        let visible_literal = Regex::new(
            r#"(?s)(?:\btext|\btext_input|\bsection_label|\berror_bar|\bshow_toast|\.label|\.placeholder|echo_str_sync|echo_warn_str_sync|StyledLine::from_echo_str)\s*\(\s*\"([A-Za-z][^\"]*)\""#,
        )
        .unwrap();
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("i18n crate has workspace parent");
        // Technical/symbolic literals that are intentionally rendered verbatim. Keep this list
        // narrow and explain each entry so an exception cannot become an English-copy dumping zone.
        let allowlist = [
            ("widgets/src/widget.rs", "x", "symbolic test/default widget content"),
        ];
        let mut unclassified = Vec::new();
        for path in workspace_sources() {
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            let production = source.split("#[cfg(test)]").next().unwrap_or(&source);
            let relative = path.strip_prefix(workspace).unwrap_or(&path).to_string_lossy();
            for capture in visible_literal.captures_iter(production) {
                let literal = &capture[1];
                let allowed = allowlist
                    .iter()
                    .any(|(file, value, _reason)| relative == *file && literal == *value);
                if !allowed {
                    let line = production[..capture.get(0).unwrap().start()].lines().count() + 1;
                    unclassified.push(format!("{relative}:{line}: {literal:?}"));
                }
            }
        }
        assert!(
            unclassified.is_empty(),
            "unclassified first-party visible literals:\n{}",
            unclassified.join("\n")
        );
    }

    #[test]
    fn interpolation_and_locale_switching_work() {
        let _guard = LOCALE_TEST_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        activate(LocalePreference::English);
        assert_eq!(t!("welcome-name", "name" => "Mira"), "Welcome, Mira.");
        activate(LocalePreference::TraditionalChinese);
        assert_eq!(t!("welcome-name", "name" => "Mira"), "歡迎，Mira。");
    }

    #[test]
    fn technical_placeholder_keeps_newline_and_password_token() {
        let _guard = LOCALE_TEST_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        activate(LocalePreference::TraditionalChinese);
        assert_eq!(t!("profile-on-connect-example"), "Gandalf\n$PASSWORD");
    }

    #[test]
    fn static_translations_are_cached_per_locale() {
        let _guard = LOCALE_TEST_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        activate(LocalePreference::English);
        let english = ts!("action-cancel");
        activate(LocalePreference::TraditionalChinese);
        let traditional_chinese = ts!("action-cancel");

        assert_eq!(english, "Cancel");
        assert_eq!(traditional_chinese, "取消");
        assert_ne!(english, traditional_chinese);
    }

    #[test]
    fn unknown_message_is_visible_instead_of_silently_blank() {
        let _guard = LOCALE_TEST_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        activate(LocalePreference::English);
        assert_eq!(translate("not-a-real-message"), "⟦not-a-real-message⟧");
    }
}
