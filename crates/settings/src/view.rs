use fluxdown_ui_components::navigation_button;
use fluxdown_ui_i18n::Translator;
use fluxdown_ui_theme::active_theme;
use gpui::{Context, Div, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px};
use gpui_component::{h_flex, h_resizable, resizable_panel, v_flex};

use crate::strings::SettingsStrings;

/// 设置能力的顶层页面。
pub struct SettingsView {
    pub(crate) translator: Entity<Translator>,
    pub(crate) strings: SettingsStrings,
}

impl SettingsView {
    /// 创建设置页面，并订阅共享翻译状态。
    pub fn new(translator: Entity<Translator>, cx: &mut Context<Self>) -> Self {
        let strings = SettingsStrings::from_translator(translator.read(cx));
        cx.observe(&translator, |this, translator, cx| {
            this.strings = SettingsStrings::from_translator(translator.read(cx));
            cx.notify();
        })
        .detach();
        Self {
            translator,
            strings,
        }
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        v_flex()
            .size_full()
            .min_w_0()
            .p(tokens.spacing.md)
            .gap(tokens.spacing.sm)
            .bg(tokens.colors.surface)
            .child(
                div()
                    .px(tokens.spacing.sm)
                    .pb(tokens.spacing.xs)
                    .text_size(tokens.typography.xs.size)
                    .font_weight(tokens.typography.xs.weight)
                    .text_color(tokens.colors.muted_foreground)
                    .child(self.strings.settings.clone()),
            )
            .child(navigation_button(
                "nav-appearance",
                self.strings.settings_appearance.clone(),
                true,
                cx,
            ))
    }
}

impl Render for SettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tokens = active_theme(cx).tokens().clone();
        div().size_full().min_w_0().min_h_0().child(
            h_resizable("settings-content")
                .child(
                    resizable_panel()
                        .size(px(232.))
                        .size_range(px(184.)..px(360.))
                        .child(self.render_sidebar(cx)),
                )
                .child(
                    resizable_panel().child(
                        v_flex()
                            .size_full()
                            .min_w_0()
                            .p(tokens.spacing.xl)
                            .gap(tokens.spacing.lg)
                            .child(
                                div()
                                    .text_size(tokens.typography.xl.size)
                                    .font_weight(tokens.typography.xl.weight)
                                    .child(self.strings.settings.clone()),
                            )
                            .child(
                                h_flex()
                                    .items_stretch()
                                    .gap(tokens.spacing.lg)
                                    .child(self.render_theme_card(cx))
                                    .child(self.render_language_card(cx)),
                            ),
                    ),
                ),
        )
    }
}
