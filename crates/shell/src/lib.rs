//! FluxDown GPUI 桌面窗口 shell。
//!
//! 本 crate 只组装窗口、主题、语言与顶层导航状态；下载领域能力后续按 feature
//! crate 接入，避免把业务、组件和二进制入口重新堆回单一编译单元。

mod assets;
mod downloads;
mod strings;
mod view;

use std::{borrow::Cow, env, sync::Arc};

use fluxdown_ui_i18n::{I18nCatalog, I18nError};
use gpui::{AppContext, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_component::{Root, TitleBar};
use thiserror::Error;

use assets::DesktopAssets;
use view::{ShellView, component_locale};
const MI_SANS_REGULAR: &[u8] = include_bytes!("../../../assets/fonts/MiSans-Regular.ttf");
const MI_SANS_MEDIUM: &[u8] = include_bytes!("../../../assets/fonts/MiSans-Medium.ttf");
const MI_SANS_SEMIBOLD: &[u8] = include_bytes!("../../../assets/fonts/MiSans-Semibold.ttf");

/// 桌面 shell 启动失败。
#[derive(Debug, Error)]
pub enum ShellError {
    #[error(transparent)]
    I18n(#[from] I18nError),
}

/// 启动 GPUI 桌面客户端并阻塞到应用退出。
pub fn run() -> Result<(), ShellError> {
    let catalog = Arc::new(I18nCatalog::load_embedded()?);
    let translator = catalog.translator(&system_locale());
    let locale = component_locale(translator.locale()).to_owned();

    gpui_platform::application()
        .with_assets(DesktopAssets)
        .run(move |cx| {
            if let Err(error) = cx.text_system().add_fonts(vec![
                Cow::Borrowed(MI_SANS_REGULAR),
                Cow::Borrowed(MI_SANS_MEDIUM),
                Cow::Borrowed(MI_SANS_SEMIBOLD),
            ]) {
                eprintln!("failed to load FluxDown UI fonts: {error:#}");
                return;
            }

            gpui_component::init(cx);
            fluxdown_ui_theme::init(cx);
            gpui_component::set_locale(&locale);

            let bounds = Bounds::centered(None, size(px(1120.), px(760.)), cx);
            let mut options = main_window_options();
            options.window_bounds = Some(WindowBounds::Windowed(bounds));

            if let Err(error) = cx.open_window(options, |window, cx| {
                let shell = cx.new(|cx| ShellView::new(translator, window, cx));
                cx.new(|cx| Root::new(shell, window, cx))
            }) {
                eprintln!("failed to open FluxDown desktop window: {error:#}");
                return;
            }

            cx.activate(true);
        });

    Ok(())
}

fn main_window_options() -> WindowOptions {
    WindowOptions {
        window_min_size: Some(size(px(720.), px(520.))),
        ..TitleBar::window_options()
    }
}

fn system_locale() -> String {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(locale) = env::var(key)
            && !locale.trim().is_empty()
        {
            return locale;
        }
    }
    "en".to_owned()
}

#[cfg(test)]
mod tests {
    use gpui::{point, px, size};

    use super::main_window_options;

    #[test]
    fn main_window_preserves_custom_titlebar_platform_contract() {
        let options = main_window_options();

        assert!(options.app_owns_titlebar_drag);
        assert_eq!(
            options
                .titlebar
                .as_ref()
                .and_then(|titlebar| titlebar.traffic_light_position),
            Some(point(px(9.), px(9.)))
        );
        assert_eq!(
            options
                .titlebar
                .as_ref()
                .and_then(|titlebar| titlebar.title.as_deref()),
            None
        );
        assert_eq!(options.window_min_size, Some(size(px(720.), px(520.))));
    }
}
