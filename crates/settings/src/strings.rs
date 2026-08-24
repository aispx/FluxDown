use fluxdown_ui_i18n::{Translator, keys};
use gpui::SharedString;

#[derive(Clone)]
pub(crate) struct SettingsStrings {
    pub(crate) language: SharedString,
    pub(crate) language_chinese: SharedString,
    pub(crate) language_desc: SharedString,
    pub(crate) language_english: SharedString,
    pub(crate) settings: SharedString,
    pub(crate) settings_appearance: SharedString,
    pub(crate) theme_mode: SharedString,
    pub(crate) theme_mode_dark: SharedString,
    pub(crate) theme_mode_desc: SharedString,
    pub(crate) theme_mode_light: SharedString,
}

impl SettingsStrings {
    pub(crate) fn from_translator(translator: &Translator) -> Self {
        Self {
            language: shared(translator.text(keys::LANGUAGE)),
            language_chinese: shared(translator.text(keys::LANGUAGE_CHINESE)),
            language_desc: shared(translator.text(keys::LANGUAGE_DESC)),
            language_english: shared(translator.text(keys::LANGUAGE_ENGLISH)),
            settings: shared(translator.text(keys::SETTINGS)),
            settings_appearance: shared(translator.text(keys::SETTINGS_CAT_APPEARANCE)),
            theme_mode: shared(translator.text(keys::THEME_MODE)),
            theme_mode_dark: shared(translator.text(keys::THEME_MODE_DARK)),
            theme_mode_desc: shared(translator.text(keys::THEME_MODE_DESC)),
            theme_mode_light: shared(translator.text(keys::THEME_MODE_LIGHT)),
        }
    }
}

fn shared(value: &str) -> SharedString {
    SharedString::from(value.to_owned())
}
