use fluxdown_ui_components::{primary_icon_button, toolbar_action_button};
use fluxdown_ui_theme::active_theme;
use gpui::{
    Context, Div, InteractiveElement as _, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement as _, Styled, div, prelude::FluentBuilder as _, px,
};
use gpui_component::{
    ActiveTheme as _, Icon, IconName, h_flex,
    scroll::{Scrollbar, ScrollbarMode},
    tooltip::Tooltip,
    v_flex, v_virtual_list,
};

use super::{
    DownloadView, ETA_COLUMN_WIDTH, GROUP_ROW_HEIGHT, ListItem, PROGRESS_COLUMN_WIDTH,
    SPEED_COLUMN_WIDTH, STATUS_COLUMN_WIDTH, TASK_ROW_HEIGHT, TRAILING_COLUMNS_WIDTH, TaskKind,
    TaskPreview, TaskState,
};

impl DownloadView {
    fn column_label(&self, label: SharedString, width: f32, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        div()
            .w(px(width))
            .flex_none()
            .text_size(tokens.typography.xs.size)
            .text_color(tokens.colors.muted_foreground)
            .child(label)
    }

    fn render_table_header(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        h_flex()
            .h(px(36.))
            .w_full()
            .flex_none()
            .items_center()
            .px(tokens.spacing.lg)
            .border_b_1()
            .border_color(tokens.colors.border)
            .child(
                div()
                    .min_w(px(240.))
                    .flex_1()
                    .text_size(tokens.typography.xs.size)
                    .text_color(tokens.colors.muted_foreground)
                    .child(self.strings.col_file_name.clone()),
            )
            .child(self.column_label(self.strings.col_progress.clone(), PROGRESS_COLUMN_WIDTH, cx))
            .child(self.column_label(self.strings.col_speed.clone(), SPEED_COLUMN_WIDTH, cx))
            .child(self.column_label(self.strings.col_eta.clone(), ETA_COLUMN_WIDTH, cx))
            .child(self.column_label(self.strings.col_status.clone(), STATUS_COLUMN_WIDTH, cx))
            .child(Icon::new(IconName::Ellipsis).size(px(14.)))
    }

    fn render_group_row(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        h_flex()
            .h(px(GROUP_ROW_HEIGHT))
            .w_full()
            .flex_none()
            .items_center()
            .px(tokens.spacing.lg)
            .gap(tokens.spacing.xs)
            .border_b_1()
            .border_color(tokens.colors.border)
            .text_size(tokens.typography.xs.size)
            .text_color(tokens.colors.muted_foreground)
            .child(Icon::new(IconName::ChevronDown).size(px(12.)))
            .child(self.strings.today.clone())
            .child("5")
            .child(div().flex_1())
            .child("3.1 GB")
            .child(div().w(px(TRAILING_COLUMNS_WIDTH)).flex_none())
    }

    fn render_progress(&self, task: TaskPreview, color: gpui::Hsla, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        h_flex()
            .w(px(PROGRESS_COLUMN_WIDTH))
            .flex_none()
            .items_center()
            .gap(tokens.spacing.sm)
            .child(
                div()
                    .relative()
                    .h(px(3.))
                    .w(px(86.))
                    .overflow_hidden()
                    .rounded_full()
                    .bg(tokens.colors.muted)
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .w(gpui::relative(task.progress))
                            .bg(color),
                    ),
            )
            .child(
                div()
                    .text_size(tokens.typography.xs.size)
                    .text_color(tokens.colors.muted_foreground)
                    .child(task.progress_label),
            )
    }

    fn render_task_row(&self, task: TaskPreview, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens().clone();
        let (status, status_icon, status_color) = match task.state {
            TaskState::Completed => (
                self.strings.status_completed.clone(),
                IconName::CircleCheck,
                cx.theme().success,
            ),
            TaskState::Paused => (
                self.strings.status_paused.clone(),
                IconName::Pause,
                cx.theme().warning,
            ),
        };
        let (file_icon, file_icon_color) = match task.kind {
            TaskKind::Application => (IconName::HardDrive, tokens.colors.muted_foreground),
            TaskKind::DiskImage => (IconName::Inbox, cx.theme().warning),
            TaskKind::Mobile => (IconName::MemoryStick, tokens.colors.muted_foreground),
        };

        h_flex()
            .h(px(TASK_ROW_HEIGHT))
            .w_full()
            .flex_none()
            .items_center()
            .px(tokens.spacing.lg)
            .border_b_1()
            .border_color(tokens.colors.border)
            .hover(|style| style.bg(tokens.colors.muted))
            .child(
                h_flex()
                    .min_w(px(240.))
                    .flex_1()
                    .min_w_0()
                    .gap(tokens.spacing.md)
                    .child(
                        div()
                            .size(px(34.))
                            .flex_none()
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(tokens.radius.md)
                            .bg(tokens.colors.muted)
                            .text_color(file_icon_color)
                            .child(Icon::new(file_icon).size(px(16.))),
                    )
                    .child(
                        v_flex()
                            .min_w_0()
                            .gap(tokens.spacing.xxs)
                            .child(
                                h_flex()
                                    .min_w_0()
                                    .gap(tokens.spacing.xs)
                                    .child(
                                        div()
                                            .min_w_0()
                                            .truncate()
                                            .text_size(tokens.typography.sm.size)
                                            .font_weight(tokens.typography.sm.weight)
                                            .child(task.name),
                                    )
                                    .child(
                                        div()
                                            .flex_none()
                                            .px(tokens.spacing.xs)
                                            .rounded(tokens.radius.sm)
                                            .bg(tokens.colors.muted)
                                            .text_size(tokens.typography.xs.size)
                                            .text_color(tokens.colors.muted_foreground)
                                            .child(task.protocol),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .gap(tokens.spacing.xs)
                                    .text_size(tokens.typography.xs.size)
                                    .text_color(tokens.colors.muted_foreground)
                                    .child(task.size)
                                    .when(matches!(task.state, TaskState::Paused), |this| {
                                        this.child("·").child(status.clone())
                                    }),
                            ),
                    ),
            )
            .child(self.render_progress(task, status_color, cx))
            .child(
                div()
                    .w(px(SPEED_COLUMN_WIDTH))
                    .flex_none()
                    .text_size(tokens.typography.xs.size)
                    .text_color(tokens.colors.muted_foreground)
                    .child("—"),
            )
            .child(
                div()
                    .w(px(ETA_COLUMN_WIDTH))
                    .flex_none()
                    .text_size(tokens.typography.xs.size)
                    .text_color(tokens.colors.muted_foreground)
                    .child("—"),
            )
            .child(
                h_flex()
                    .w(px(STATUS_COLUMN_WIDTH))
                    .flex_none()
                    .items_center()
                    .gap(tokens.spacing.xs)
                    .text_size(tokens.typography.xs.size)
                    .font_weight(tokens.typography.xs.weight)
                    .text_color(status_color)
                    .child(Icon::new(status_icon).size(px(13.)))
                    .child(status),
            )
            .child(div().w(px(14.)).flex_none())
    }

    fn render_list_item(&self, index: usize, cx: &mut Context<Self>) -> Div {
        match self.items[index] {
            ListItem::Group => self.render_group_row(cx),
            ListItem::Task(task) => self.render_task_row(task, cx),
        }
    }

    fn render_task_list(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        div()
            .relative()
            .flex_1()
            .min_h_0()
            .overflow_hidden()
            .bg(tokens.colors.surface)
            .child(
                v_virtual_list(
                    cx.entity(),
                    "download-task-list",
                    self.item_sizes.clone(),
                    |this, visible_range, _, cx| {
                        visible_range
                            .map(|index| this.render_list_item(index, cx))
                            .collect()
                    },
                )
                .track_scroll(&self.scroll_handle)
                .size_full(),
            )
            .child(Scrollbar::vertical(&self.scroll_handle).mode(ScrollbarMode::Always))
    }
    fn toolbar_icon_action(
        &self,
        tooltip_id: &'static str,
        button_id: &'static str,
        label: SharedString,
        icon: IconName,
        destructive: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let tooltip_label = label.clone();
        div()
            .id(tooltip_id)
            .size(px(30.))
            .tooltip(move |window, cx| Tooltip::new(tooltip_label.clone()).build(window, cx))
            .child(toolbar_action_button(
                button_id,
                label,
                Icon::new(icon).size(px(15.)),
                destructive,
                cx,
            ))
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens().clone();
        let separator = || {
            div()
                .h(px(24.))
                .w(px(1.))
                .mx(tokens.spacing.xs)
                .bg(tokens.colors.border)
        };

        h_flex()
            .h(px(40.))
            .w_full()
            .flex_none()
            .items_center()
            .px(tokens.spacing.lg)
            .gap(tokens.spacing.xxs)
            .border_b_1()
            .border_color(tokens.colors.border)
            .child(primary_icon_button(
                "download-new",
                self.strings.new_download.clone(),
                Icon::new(IconName::Plus).size(px(15.)),
                cx,
            ))
            .child(separator())
            .child(self.toolbar_icon_action(
                "download-resume-tooltip",
                "download-resume",
                self.strings.resume.clone(),
                IconName::Play,
                false,
                cx,
            ))
            .child(self.toolbar_icon_action(
                "download-pause-tooltip",
                "download-pause",
                self.strings.pause.clone(),
                IconName::Pause,
                false,
                cx,
            ))
            .child(separator())
            .child(self.toolbar_icon_action(
                "download-stop-all-tooltip",
                "download-stop-all",
                self.strings.stop_all.clone(),
                IconName::CircleX,
                false,
                cx,
            ))
            .child(separator())
            .child(self.toolbar_icon_action(
                "download-delete-tooltip",
                "download-delete",
                self.strings.delete.clone(),
                IconName::Delete,
                true,
                cx,
            ))
    }

    pub(super) fn render_main(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        v_flex()
            .size_full()
            .min_w_0()
            .min_h_0()
            .bg(tokens.colors.surface)
            .child(self.render_toolbar(cx))
            .child(self.render_table_header(cx))
            .child(self.render_task_list(cx))
    }
}
