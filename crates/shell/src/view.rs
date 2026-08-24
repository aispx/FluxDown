use fluxdown_ui_components::{ButtonVariant, button, card, navigation_button};
use fluxdown_ui_i18n::Translator;
use fluxdown_ui_theme::{active_theme, toggle_theme};
use gpui::{
    Context, Div, InteractiveElement as _, IntoElement, MouseButton, ParentElement, Render,
    SharedString, Styled, Window, div, img, px,
};
use gpui_component::{
    TitleBar,
    button::{Button, ButtonVariants as _},
    h_flex,
    menu::DropdownMenu as _,
    v_flex,
};

use crate::{assets::APP_LOGO_PATH, strings::ShellStrings};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    All,
    Downloading,
    Completed,
    Settings,
}

pub(crate) struct ShellView {
    translator: Translator,
    strings: ShellStrings,
    selected: Section,
}

impl ShellView {
    pub(crate) fn new(translator: Translator) -> Self {
        let strings = ShellStrings::from_translator(&translator);
        Self {
            translator,
            strings,
            selected: Section::Settings,
        }
    }

    fn section_label(&self, section: Section) -> SharedString {
        match section {
            Section::All => self.strings.category_all.clone(),
            Section::Downloading => self.strings.status_downloading.clone(),
            Section::Completed => self.strings.status_completed.clone(),
            Section::Settings => self.strings.settings.clone(),
        }
    }

    fn render_title_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let tokens = active_theme(cx).tokens();
        let colors = tokens.colors;
        let spacing = tokens.spacing;
        let typography = tokens.typography.clone();
        let menu_items = [
            ("title-menu-file", self.strings.menu_file.clone()),
            ("title-menu-tasks", self.strings.menu_tasks.clone()),
            ("title-menu-tools", self.strings.menu_tools.clone()),
            ("title-menu-help", self.strings.menu_help.clone()),
        ];
        let menu_placeholder = self.strings.menu_items_pending.clone();
        let title_bar = TitleBar::new();
        // macOS 保留 TitleBar 为原生交通灯预留的 80px。
        #[cfg(not(target_os = "macos"))]
        let title_bar = title_bar.pl(spacing.sm);

        title_bar
            .h(px(40.))
            .bg(colors.surface)
            .border_color(colors.border)
            .child(
                h_flex()
                    .size_full()
                    .min_w_0()
                    .items_center()
                    .gap(spacing.sm)
                    .pl(spacing.xxs)
                    .pr(spacing.md)
                    .child(img(APP_LOGO_PATH).size(px(16.)))
                    .child(
                        h_flex()
                            .h_full()
                            .items_center()
                            .text_size(typography.sm.size)
                            .font_weight(typography.sm.weight)
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .children(menu_items.into_iter().map(|(id, label)| {
                                let placeholder = menu_placeholder.clone();
                                Button::new(id)
                                    .label(label)
                                    .text()
                                    .compact()
                                    .h_full()
                                    .px(spacing.sm)
                                    .dropdown_menu(move |menu, _, _| {
                                        menu.min_w(140.).label(placeholder.clone())
                                    })
                            })),
                    )
                    .child(div().flex_1()),
            )
    }

    fn navigation_item(
        &self,
        id: &'static str,
        section: Section,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        navigation_button(
            id,
            self.section_label(section),
            self.selected == section,
            cx,
        )
        .on_click(cx.listener(move |this, _, _, cx| {
            if this.selected != section {
                this.selected = section;
                cx.notify();
            }
        }))
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        v_flex()
            .h_full()
            .w(px(232.))
            .flex_none()
            .p(tokens.spacing.md)
            .gap(tokens.spacing.sm)
            .bg(tokens.colors.surface)
            .border_r_1()
            .border_color(tokens.colors.border)
            .child(
                div()
                    .px(tokens.spacing.sm)
                    .pb(tokens.spacing.xs)
                    .text_size(tokens.typography.xs.size)
                    .font_weight(tokens.typography.xs.weight)
                    .text_color(tokens.colors.muted_foreground)
                    .child(self.strings.sidebar_status.clone()),
            )
            .child(self.navigation_item("nav-all", Section::All, cx))
            .child(self.navigation_item("nav-downloading", Section::Downloading, cx))
            .child(self.navigation_item("nav-completed", Section::Completed, cx))
            .child(self.navigation_item("nav-settings", Section::Settings, cx))
    }

    fn render_theme_card(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        let next_mode = if active_theme(cx).mode().is_dark() {
            self.strings.theme_mode_light.clone()
        } else {
            self.strings.theme_mode_dark.clone()
        };

        card(cx).p(tokens.spacing.lg).flex_1().child(
            h_flex()
                .items_center()
                .justify_between()
                .gap(tokens.spacing.lg)
                .child(
                    v_flex()
                        .gap(tokens.spacing.xxs)
                        .child(
                            div()
                                .text_size(tokens.typography.md.size)
                                .font_weight(tokens.typography.md.weight)
                                .child(self.strings.theme_mode.clone()),
                        )
                        .child(
                            div()
                                .text_size(tokens.typography.sm.size)
                                .text_color(tokens.colors.muted_foreground)
                                .child(self.strings.theme_mode_desc.clone()),
                        ),
                )
                .child(
                    button("toggle-theme", next_mode, ButtonVariant::Primary, cx).on_click(
                        cx.listener(|_, _, window, cx| {
                            toggle_theme(window, cx);
                            cx.notify();
                        }),
                    ),
                ),
        )
    }

    fn render_language_card(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        let next_locale = if self.translator.locale() == "zh" {
            self.strings.language_english.clone()
        } else {
            self.strings.language_chinese.clone()
        };

        card(cx).p(tokens.spacing.lg).flex_1().child(
            h_flex()
                .items_center()
                .justify_between()
                .gap(tokens.spacing.lg)
                .child(
                    v_flex()
                        .gap(tokens.spacing.xxs)
                        .child(
                            div()
                                .text_size(tokens.typography.md.size)
                                .font_weight(tokens.typography.md.weight)
                                .child(self.strings.language.clone()),
                        )
                        .child(
                            div()
                                .text_size(tokens.typography.sm.size)
                                .text_color(tokens.colors.muted_foreground)
                                .child(self.strings.language_desc.clone()),
                        ),
                )
                .child(
                    button("toggle-language", next_locale, ButtonVariant::Secondary, cx).on_click(
                        cx.listener(|this, _, _, cx| {
                            let next = if this.translator.locale() == "zh" {
                                "en"
                            } else {
                                "zh"
                            };
                            if this.translator.set_locale(next) {
                                gpui_component::set_locale(component_locale(next));
                                this.strings = ShellStrings::from_translator(&this.translator);
                                cx.notify();
                            }
                        }),
                    ),
                ),
        )
    }

    fn render_section_body(&self, cx: &mut Context<Self>) -> Div {
        let theme = active_theme(cx).tokens();
        let colors = theme.colors;
        let spacing = theme.spacing;
        let typography = theme.typography.clone();

        if self.selected == Section::Settings {
            h_flex()
                .items_stretch()
                .gap(spacing.lg)
                .child(self.render_theme_card(cx))
                .child(self.render_language_card(cx))
        } else {
            card(cx)
                .flex_1()
                .min_h(px(240.))
                .p(spacing.xl)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    v_flex()
                        .items_center()
                        .gap(spacing.xs)
                        .child(
                            div()
                                .text_size(typography.lg.size)
                                .font_weight(typography.lg.weight)
                                .child(self.strings.empty_title.clone()),
                        )
                        .child(
                            div()
                                .text_size(typography.sm.size)
                                .text_color(colors.muted_foreground)
                                .child(self.section_label(self.selected)),
                        ),
                )
        }
    }
}

impl Render for ShellView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = active_theme(cx).tokens();
        let colors = theme.colors;
        let spacing = theme.spacing;
        let typography = theme.typography.clone();

        v_flex()
            .size_full()
            .bg(colors.background)
            .text_color(colors.foreground)
            .child(self.render_title_bar(cx))
            .child(
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .child(self.render_sidebar(cx))
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w_0()
                            .h_full()
                            .p(spacing.xl)
                            .gap(spacing.lg)
                            .child(
                                div()
                                    .text_size(typography.xl.size)
                                    .font_weight(typography.xl.weight)
                                    .child(self.section_label(self.selected)),
                            )
                            .child(self.render_section_body(cx)),
                    ),
            )
    }
}

pub(crate) fn component_locale(locale: &str) -> &str {
    if locale == "zh" { "zh-CN" } else { "en" }
}
