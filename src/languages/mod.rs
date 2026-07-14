//! UI translations.
//!
//! Each language lives in its own `language_<code>.rs` file so translators can
//! work on a single file without creating merge conflicts with other languages.
//! Adding a field to [`Tr`] forces every language file to provide it, so the
//! translations cannot silently fall out of sync.

mod language_da;
mod language_de;
mod language_en;
mod language_es;
mod language_it;
mod language_pt;
mod language_tr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    English,
    German,
    Spanish,
    Italian,
    Portuguese,
    Turkish,
    Danish,
}

impl Language {
    pub const ALL: [Language; 7] = [
        Language::English,
        Language::German,
        Language::Spanish,
        Language::Italian,
        Language::Portuguese,
        Language::Turkish,
        Language::Danish,
    ];

    pub fn native_name(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::German => "Deutsch",
            Language::Spanish => "Español",
            Language::Italian => "Italiano",
            Language::Portuguese => "Português",
            Language::Turkish => "Türkçe",
            Language::Danish => "Dansk",
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::Spanish => "es",
            Language::Italian => "it",
            Language::Portuguese => "pt",
            Language::Turkish => "tr",
            Language::Danish => "da",
        }
    }

    pub fn from_code(code: &str) -> Option<Language> {
        Language::ALL
            .into_iter()
            .find(|language| language.code() == code)
    }

    pub fn strings(self) -> &'static Tr {
        match self {
            Language::English => &language_en::EN,
            Language::German => &language_de::DE,
            Language::Spanish => &language_es::ES,
            Language::Italian => &language_it::IT,
            Language::Portuguese => &language_pt::PT,
            Language::Turkish => &language_tr::TR,
            Language::Danish => &language_da::DA,
        }
    }
}

/// A complete set of translated UI strings for one language.
///
/// Fields ending in a `{placeholder}` (e.g. `{name}`, `{count}`, `{path}`) are
/// templates filled in at runtime with `fill()`; keep the tokens intact when
/// translating.
pub struct Tr {
    pub language_label: &'static str,
    pub refresh: &'static str,
    pub restore_defaults: &'static str,
    pub import: &'static str,
    pub export_all: &'static str,
    pub download_fpsheaven: &'static str,
    pub youtube: &'static str,
    pub selected: &'static str,
    pub export: &'static str,
    pub delete: &'static str,
    pub power_plans: &'static str,
    pub no_power_plans_found: &'static str,
    pub select_a_power_plan: &'static str,
    pub guid: &'static str,
    pub name_and_description: &'static str,
    pub active_badge: &'static str,
    pub save: &'static str,
    pub description_hint: &'static str,
    pub activate: &'static str,
    pub duplicate: &'static str,
    pub activate_and_edit: &'static str,
    pub import_dialog_title: &'static str,
    pub import_dialog_prompt: &'static str,
    pub cancel: &'static str,
    pub reset_dialog_title: &'static str,
    pub reset_line_1: &'static str,
    pub reset_line_2: &'static str,
    pub reset_line_3: &'static str,
    pub original_error: &'static str,
    pub reset_anyway: &'static str,
    pub delete_dialog_title: &'static str,
    pub delete_confirm: &'static str,
    pub delete_removes_from_windows: &'static str,
    pub power_plan_filter: &'static str,
    pub import_power_plan_title: &'static str,
    pub export_power_plan_title: &'static str,
    pub choose_export_folder_title: &'static str,
    pub ready: &'static str,
    pub loaded_power_plans: &'static str,
    pub refreshed_power_plans: &'static str,
    pub import_busy: &'static str,
    pub imported_success: &'static str,
    pub imported_partial: &'static str,
    pub select_plan_first: &'static str,
    pub exported_to: &'static str,
    pub no_plans_to_export: &'static str,
    pub exported_all: &'static str,
    pub exported_partial: &'static str,
    pub activated: &'static str,
    pub duplicated: &'static str,
    pub plan_name_empty: &'static str,
    pub updated: &'static str,
    pub activate_before_delete: &'static str,
    pub plan_no_longer_exists: &'static str,
    pub deleted: &'static str,
    pub opened_windows_editor: &'static str,
    pub opened_youtube: &'static str,
    pub could_not_open_youtube: &'static str,
    pub defaults_already_available: &'static str,
    pub import_already_running: &'static str,
    pub downloading: &'static str,
    pub import_stopped: &'static str,
    pub imported_activated: &'static str,
    pub replaced_suffix: &'static str,
    pub windows_reset_canceled: &'static str,
}
