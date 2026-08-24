use fluxdown_ui_i18n::{Translator, keys};
use gpui::SharedString;

#[derive(Clone)]
pub(crate) struct ShellStrings {
    pub(crate) category_all: SharedString,
    pub(crate) empty_title: SharedString,
    pub(crate) downloads: SharedString,
    pub(crate) language: SharedString,
    pub(crate) language_chinese: SharedString,
    pub(crate) language_desc: SharedString,
    pub(crate) language_english: SharedString,
    pub(crate) menu_file: SharedString,
    pub(crate) menu_help: SharedString,
    pub(crate) menu_items_pending: SharedString,
    pub(crate) menu_tasks: SharedString,
    pub(crate) menu_tools: SharedString,
    pub(crate) settings: SharedString,
    pub(crate) settings_appearance: SharedString,
    pub(crate) sidebar_status: SharedString,
    pub(crate) status_completed: SharedString,
    pub(crate) status_downloading: SharedString,
    pub(crate) theme_mode: SharedString,
    pub(crate) theme_mode_dark: SharedString,
    pub(crate) theme_mode_desc: SharedString,
    pub(crate) theme_mode_light: SharedString,
}

impl ShellStrings {
    pub(crate) fn from_translator(translator: &Translator) -> Self {
        Self {
            category_all: shared(translator.text(keys::CATEGORY_ALL)),
            downloads: shared(translator.text(keys::MOBILE_NAV_DOWNLOADS)),
            empty_title: shared(translator.text(keys::EMPTY_TITLE)),
            language: shared(translator.text(keys::LANGUAGE)),
            language_chinese: shared(translator.text(keys::LANGUAGE_CHINESE)),
            language_desc: shared(translator.text(keys::LANGUAGE_DESC)),
            language_english: shared(translator.text(keys::LANGUAGE_ENGLISH)),
            menu_file: shared(translator.text(keys::MENU_FILE)),
            menu_help: shared(translator.text(keys::MENU_HELP)),
            menu_items_pending: shared(translator.text(keys::MENU_ITEMS_PENDING)),
            menu_tasks: shared(translator.text(keys::MENU_TASKS)),
            menu_tools: shared(translator.text(keys::MENU_TOOLS)),
            settings: shared(translator.text(keys::SETTINGS)),
            settings_appearance: shared(translator.text(keys::SETTINGS_CAT_APPEARANCE)),
            sidebar_status: shared(translator.text(keys::SIDEBAR_STATUS)),
            status_completed: shared(translator.text(keys::STATUS_COMPLETED)),
            status_downloading: shared(translator.text(keys::STATUS_DOWNLOADING)),
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
