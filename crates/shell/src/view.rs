use fluxdown_ui_components::{
    ButtonVariant, activity_button as activity_bar_button, button, card, navigation_button,
};
use fluxdown_ui_i18n::Translator;
use fluxdown_ui_theme::{active_theme, toggle_theme};
use gpui::{
    AppContext as _, Context, Div, Entity, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement, Render, SharedString, StatefulInteractiveElement as _, Styled, Window, div, img,
    px,
};
use gpui_component::{
    Icon, IconName, TitleBar,
    button::{Button, ButtonVariants as _},
    h_flex, h_resizable,
    menu::DropdownMenu as _,
    resizable_panel,
    tooltip::Tooltip,
    v_flex,
};

use crate::{
    assets::{APP_LOGO_PATH, DOWNLOAD_ICON_PATH},
    downloads::DownloadView,
    strings::{DownloadStrings, ShellStrings},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Activity {
    Downloads,
    Settings,
}

pub(crate) struct ShellView {
    translator: Translator,
    strings: ShellStrings,
    active_activity: Activity,
    downloads: Entity<DownloadView>,
}

impl ShellView {
    pub(crate) fn new(translator: Translator, cx: &mut Context<Self>) -> Self {
        let strings = ShellStrings::from_translator(&translator);
        let downloads =
            cx.new(|_| DownloadView::new(DownloadStrings::from_translator(&translator)));
        Self {
            translator,
            strings,
            active_activity: Activity::Downloads,
            downloads,
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
                                    .cursor_pointer()
                                    .dropdown_menu(move |menu, _, _| {
                                        menu.min_w(140.).label(placeholder.clone())
                                    })
                            })),
                    )
                    .child(div().flex_1()),
            )
    }

    fn activity_button(
        &self,
        id: &'static str,
        activity: Activity,
        icon: Icon,
        label: SharedString,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let selected = self.active_activity == activity;
        let tooltip_label = label.clone();
        let tooltip_id = match activity {
            Activity::Downloads => "activity-downloads-tooltip",
            Activity::Settings => "activity-settings-tooltip",
        };

        div()
            .id(tooltip_id)
            .size(px(48.))
            .flex()
            .items_center()
            .justify_center()
            .tooltip(move |window, cx| Tooltip::new(tooltip_label.clone()).build(window, cx))
            .child(
                activity_bar_button(id, label, icon.size(px(21.)), selected, cx).on_click(
                    cx.listener(move |this, _, _, cx| {
                        if this.active_activity != activity {
                            this.active_activity = activity;
                            cx.notify();
                        }
                    }),
                ),
            )
    }

    fn render_activity_bar(&self, cx: &mut Context<Self>) -> Div {
        let colors = active_theme(cx).tokens().colors;
        let bottom_padding = active_theme(cx).tokens().spacing.xs;
        v_flex()
            .h_full()
            .w(px(48.))
            .flex_none()
            .justify_between()
            .bg(colors.surface)
            .border_r_1()
            .border_color(colors.border)
            .child(self.activity_button(
                "activity-downloads",
                Activity::Downloads,
                Icon::empty().path(DOWNLOAD_ICON_PATH),
                self.strings.downloads.clone(),
                cx,
            ))
            .child(v_flex().pb(bottom_padding).child(self.activity_button(
                "activity-settings",
                Activity::Settings,
                Icon::new(IconName::Settings),
                self.strings.settings.clone(),
                cx,
            )))
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
                                let download_strings =
                                    DownloadStrings::from_translator(&this.translator);
                                this.downloads.update(cx, |downloads, cx| {
                                    downloads.set_strings(download_strings, cx);
                                });
                                cx.notify();
                            }
                        }),
                    ),
                ),
        )
    }

    fn render_settings_sidebar(&self, cx: &mut Context<Self>) -> Div {
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

    fn render_settings(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens().clone();
        div().size_full().min_w_0().min_h_0().child(
            h_resizable("settings-content")
                .child(
                    resizable_panel()
                        .size(px(232.))
                        .size_range(px(184.)..px(360.))
                        .child(self.render_settings_sidebar(cx)),
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

impl Render for ShellView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = active_theme(cx).tokens().colors;
        let content = match self.active_activity {
            Activity::Downloads => self.downloads.clone().into_any_element(),
            Activity::Settings => self.render_settings(cx).into_any_element(),
        };

        v_flex()
            .size_full()
            .bg(colors.background)
            .text_color(colors.foreground)
            .child(self.render_title_bar(cx))
            .child(
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .items_stretch()
                    .child(self.render_activity_bar(cx))
                    .child(div().h_full().flex_1().min_w_0().min_h_0().child(content)),
            )
    }
}

pub(crate) fn component_locale(locale: &str) -> &str {
    if locale == "zh" { "zh-CN" } else { "en" }
}
