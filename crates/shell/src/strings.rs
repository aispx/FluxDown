use fluxdown_ui_i18n::{Translator, keys};
use gpui::SharedString;

#[derive(Clone)]
pub(crate) struct ShellStrings {
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
    pub(crate) theme_mode: SharedString,
    pub(crate) theme_mode_dark: SharedString,
    pub(crate) theme_mode_desc: SharedString,
    pub(crate) theme_mode_light: SharedString,
}

impl ShellStrings {
    pub(crate) fn from_translator(translator: &Translator) -> Self {
        Self {
            downloads: shared(translator.text(keys::MOBILE_NAV_DOWNLOADS)),
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
            theme_mode: shared(translator.text(keys::THEME_MODE)),
            theme_mode_dark: shared(translator.text(keys::THEME_MODE_DARK)),
            theme_mode_desc: shared(translator.text(keys::THEME_MODE_DESC)),
            theme_mode_light: shared(translator.text(keys::THEME_MODE_LIGHT)),
        }
    }
}

#[derive(Clone)]
pub(crate) struct DownloadStrings {
    pub(crate) category_all: SharedString,
    pub(crate) category_archive: SharedString,
    pub(crate) category_audio: SharedString,
    pub(crate) category_document: SharedString,
    pub(crate) category_image: SharedString,
    pub(crate) category_other: SharedString,
    pub(crate) category_program: SharedString,
    pub(crate) category_video: SharedString,
    pub(crate) col_created: SharedString,
    pub(crate) col_eta: SharedString,
    pub(crate) col_file_name: SharedString,
    pub(crate) col_size: SharedString,
    pub(crate) col_speed: SharedString,
    pub(crate) col_status: SharedString,
    pub(crate) delete: SharedString,
    pub(crate) later_queue: SharedString,
    pub(crate) new_download: SharedString,
    pub(crate) pause: SharedString,
    pub(crate) resume: SharedString,
    pub(crate) main_queue: SharedString,
    pub(crate) sidebar_category: SharedString,
    pub(crate) sidebar_queues: SharedString,
    pub(crate) sidebar_status: SharedString,
    pub(crate) status_all: SharedString,
    pub(crate) status_completed: SharedString,
    pub(crate) status_downloading: SharedString,
    pub(crate) status_error: SharedString,
    pub(crate) status_paused: SharedString,
    pub(crate) status_seeding: SharedString,
    pub(crate) stop_all: SharedString,
    pub(crate) today: SharedString,
    pub(crate) view_columns_at_least_one: SharedString,
    pub(crate) view_columns_menu_title: SharedString,
    pub(crate) view_columns_reset_action: SharedString,
}

impl DownloadStrings {
    pub(crate) fn from_translator(translator: &Translator) -> Self {
        Self {
            category_all: shared(translator.text(keys::CATEGORY_ALL)),
            category_archive: shared(translator.text(keys::CATEGORY_ARCHIVE)),
            category_audio: shared(translator.text(keys::CATEGORY_AUDIO)),
            category_document: shared(translator.text(keys::CATEGORY_DOCUMENT)),
            category_image: shared(translator.text(keys::CATEGORY_IMAGE)),
            category_other: shared(translator.text(keys::CATEGORY_OTHER)),
            category_program: shared(translator.text(keys::CATEGORY_PROGRAM)),
            category_video: shared(translator.text(keys::CATEGORY_VIDEO)),
            col_created: shared(translator.text(keys::COL_CREATED)),
            col_eta: shared(translator.text(keys::COL_ETA)),
            col_file_name: shared(translator.text(keys::COL_FILE_NAME)),
            col_size: shared(translator.text(keys::COL_SIZE)),
            col_speed: shared(translator.text(keys::COL_SPEED)),
            col_status: shared(translator.text(keys::COL_STATUS)),
            later_queue: shared(translator.text(keys::LATER_QUEUE)),
            delete: shared(translator.text(keys::DELETE)),
            main_queue: shared(translator.text(keys::MAIN_QUEUE)),
            new_download: shared(translator.text(keys::NEW_DOWNLOAD)),
            pause: shared(translator.text(keys::PAUSE)),
            resume: shared(translator.text(keys::RESUME)),
            sidebar_category: shared(translator.text(keys::SIDEBAR_CATEGORY)),
            sidebar_queues: shared(translator.text(keys::SIDEBAR_QUEUES)),
            sidebar_status: shared(translator.text(keys::SIDEBAR_STATUS)),
            status_all: shared(translator.text(keys::TAB_ALL)),
            status_completed: shared(translator.text(keys::STATUS_COMPLETED)),
            status_downloading: shared(translator.text(keys::STATUS_DOWNLOADING)),
            status_error: shared(translator.text(keys::STATUS_ERROR)),
            status_paused: shared(translator.text(keys::STATUS_PAUSED)),
            status_seeding: shared(translator.text(keys::STATUS_SEEDING)),
            stop_all: shared(translator.text(keys::STOP_ALL)),
            today: shared(translator.text(keys::TODAY)),
            view_columns_at_least_one: shared(translator.text(keys::VIEW_COLUMNS_AT_LEAST_ONE)),
            view_columns_menu_title: shared(translator.text(keys::VIEW_COLUMNS_MENU_TITLE)),
            view_columns_reset_action: shared(translator.text(keys::VIEW_COLUMNS_RESET_ACTION)),
        }
    }
}

fn shared(value: &str) -> SharedString {
    SharedString::from(value.to_owned())
}
