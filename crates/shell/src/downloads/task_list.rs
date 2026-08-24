use fluxdown_ui_components::{primary_icon_button, toolbar_action_button};
use fluxdown_ui_theme::active_theme;
use gpui::{
    Context, Div, InteractiveElement as _, IntoElement, MouseButton, MouseDownEvent, ParentElement,
    SharedString, StatefulInteractiveElement as _, Styled, div, prelude::FluentBuilder as _, px,
    relative,
};
use gpui_component::{
    ActiveTheme as _, FocusableExt as _, Icon, IconName, Sizable as _, Size,
    checkbox::Checkbox,
    h_flex,
    scroll::{Scrollbar, ScrollbarMode},
    tooltip::Tooltip,
    v_flex, v_virtual_list,
};

use super::{
    CREATED_COLUMN_WIDTH, DownloadView, ETA_COLUMN_WIDTH, SELECTION_COLUMN_WIDTH,
    SIZE_COLUMN_WIDTH, SPEED_COLUMN_WIDTH, STATUS_COLUMN_WIDTH, TASK_ROW_HEIGHT, TaskKind,
    TaskPreview, TaskState,
};

impl DownloadView {
    fn column_label(&self, label: SharedString, width: f32, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        h_flex()
            .w(px(width))
            .flex_none()
            .items_center()
            .text_size(tokens.typography.xs.size)
            .text_color(tokens.colors.muted_foreground)
            .child(
                div()
                    .h(px(16.))
                    .w(px(1.))
                    .mr(tokens.spacing.sm)
                    .bg(tokens.colors.border),
            )
            .child(label)
    }

    fn render_table_header(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        let all_tasks_selected = !self.items.is_empty()
            && self
                .items
                .iter()
                .all(|task| self.selected_tasks.contains(&task.id));

        h_flex()
            .h(px(34.))
            .mx(px(4.))
            .flex_none()
            .items_center()
            .px(tokens.spacing.lg)
            .rounded(tokens.radius.md)
            .overflow_hidden()
            .bg(tokens.colors.muted)
            .child(
                h_flex()
                    .w(px(SELECTION_COLUMN_WIDTH))
                    .flex_none()
                    .items_center()
                    .child(
                        Checkbox::new("select-all-download-tasks")
                            .with_size(Size::XSmall)
                            .checked(all_tasks_selected)
                            .focus_ring(false)
                            .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                if *checked {
                                    for task in &this.items {
                                        this.selected_tasks.insert(task.id);
                                    }
                                } else {
                                    this.selected_tasks.clear();
                                }
                                this.selection_anchor = None;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .w(px(188.))
                    .min_w(px(140.))
                    .max_w(px(188.))
                    .text_size(tokens.typography.xs.size)
                    .text_color(tokens.colors.muted_foreground)
                    .child(self.strings.col_file_name.clone()),
            )
            .child(self.column_label(self.strings.col_size.clone(), SIZE_COLUMN_WIDTH, cx))
            .child(self.column_label(self.strings.col_status.clone(), STATUS_COLUMN_WIDTH, cx))
            .child(self.column_label(self.strings.col_speed.clone(), SPEED_COLUMN_WIDTH, cx))
            .child(self.column_label(self.strings.col_eta.clone(), ETA_COLUMN_WIDTH, cx))
            .child(self.column_label(self.strings.col_created.clone(), CREATED_COLUMN_WIDTH, cx))
    }

    fn render_status(
        &self,
        task: TaskPreview,
        status: SharedString,
        color: gpui::Hsla,
        cx: &mut Context<Self>,
    ) -> Div {
        let tokens = active_theme(cx).tokens();
        v_flex()
            .w(px(STATUS_COLUMN_WIDTH))
            .flex_none()
            .gap(px(2.))
            .child(
                h_flex()
                    .gap(tokens.spacing.xs)
                    .text_size(tokens.typography.xs.size)
                    .line_height(relative(1.))
                    .font_weight(tokens.typography.xs.weight)
                    .child(task.progress_label)
                    .child(status),
            )
            .child(
                div()
                    .relative()
                    .h(px(4.))
                    .w(px(110.))
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
    }

    fn render_task_row(&self, task: TaskPreview, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens().clone();
        let task_id = task.id;
        let selected = self.selected_tasks.contains(&task_id);
        let hover_background = if selected {
            tokens.colors.accent
        } else {
            tokens.colors.muted
        };
        let (status, status_color) = match task.state {
            TaskState::Completed => (self.strings.status_completed.clone(), cx.theme().success),
            TaskState::Paused => (self.strings.status_paused.clone(), cx.theme().warning),
        };
        let (file_icon, file_icon_color, category) = match task.kind {
            TaskKind::Application => (
                IconName::HardDrive,
                tokens.colors.muted_foreground,
                self.strings.category_program.clone(),
            ),
            TaskKind::Mobile => (
                IconName::MemoryStick,
                tokens.colors.muted_foreground,
                self.strings.category_program.clone(),
            ),
            TaskKind::DiskImage => (
                IconName::Inbox,
                cx.theme().warning,
                self.strings.category_archive.clone(),
            ),
        };

        h_flex()
            .h(px(TASK_ROW_HEIGHT))
            .w_full()
            .flex_none()
            .items_center()
            .px(tokens.spacing.lg)
            .rounded(tokens.radius.sm)
            .cursor_pointer()
            .when(selected, |this| this.bg(tokens.colors.accent))
            .hover(|style| style.bg(hover_background))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                    this.select_task(task_id, event.modifiers);
                    cx.notify();
                }),
            )
            .child(
                h_flex()
                    .id(("download-task-multi-select-slot", task_id))
                    .w(px(SELECTION_COLUMN_WIDTH))
                    .flex_none()
                    .items_center()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(
                        Checkbox::new(("download-task-multi-select", task_id))
                            .with_size(Size::XSmall)
                            .checked(selected)
                            .focus_ring(false)
                            .on_click(cx.listener(move |this, checked: &bool, _, cx| {
                                if *checked {
                                    this.selected_tasks.insert(task_id);
                                } else {
                                    this.selected_tasks.remove(&task_id);
                                }
                                this.selection_anchor = Some(task_id);
                                cx.notify();
                            })),
                    ),
            )
            .child(
                h_flex()
                    .w(px(188.))
                    .min_w(px(140.))
                    .max_w(px(188.))
                    .gap(tokens.spacing.sm)
                    .child(
                        div()
                            .flex_none()
                            .text_color(file_icon_color)
                            .child(Icon::new(file_icon).size(px(14.))),
                    )
                    .child(
                        v_flex()
                            .min_w_0()
                            .gap(px(1.))
                            .child(
                                div()
                                    .min_w_0()
                                    .truncate()
                                    .text_size(px(12.))
                                    .line_height(relative(1.))
                                    .font_weight(tokens.typography.sm.weight)
                                    .child(task.name),
                            )
                            .child(
                                div()
                                    .text_size(px(8.))
                                    .line_height(relative(1.))
                                    .text_color(tokens.colors.muted_foreground)
                                    .child(category),
                            ),
                    ),
            )
            .child(
                div()
                    .w(px(SIZE_COLUMN_WIDTH))
                    .flex_none()
                    .text_size(tokens.typography.xs.size)
                    .text_color(tokens.colors.muted_foreground)
                    .child(task.size),
            )
            .child(self.render_status(task, status, status_color, cx))
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
                div()
                    .w(px(CREATED_COLUMN_WIDTH))
                    .flex_none()
                    .text_size(tokens.typography.xs.size)
                    .text_color(tokens.colors.muted_foreground)
                    .child(self.strings.today.clone()),
            )
    }

    fn render_list_item(&self, index: usize, cx: &mut Context<Self>) -> Div {
        self.render_task_row(self.items[index], cx)
    }

    fn render_task_list(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens();
        div()
            .relative()
            .mx(px(4.))
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
            .px(px(4.))
            .mb(px(4.))
            .gap(tokens.spacing.xxs)
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
