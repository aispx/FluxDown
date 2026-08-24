use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

pub(crate) const APP_LOGO_PATH: &str = "fluxdown/logo.png";
const APP_LOGO: &[u8] = include_bytes!("../../../assets/logo/fluxdown_logo.png");

/// FluxDown 品牌资源与 gpui-component 内置图标的单一组合入口。
pub(crate) struct DesktopAssets;

impl AssetSource for DesktopAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path == APP_LOGO_PATH {
            return Ok(Some(Cow::Borrowed(APP_LOGO)));
        }
        gpui_component_assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut assets = gpui_component_assets::Assets.list(path)?;
        if APP_LOGO_PATH.starts_with(path) {
            assets.push(APP_LOGO_PATH.into());
        }
        Ok(assets)
    }
}

#[cfg(test)]
mod tests {
    use gpui::AssetSource;

    use super::{APP_LOGO, APP_LOGO_PATH, DesktopAssets};

    #[test]
    fn desktop_assets_preserve_brand_and_window_control_icons() -> gpui::Result<()> {
        let assets = DesktopAssets;

        assert_eq!(assets.load(APP_LOGO_PATH)?.as_deref(), Some(APP_LOGO));
        assert!(assets.load("icons/window-close.svg")?.is_some());
        Ok(())
    }
}
