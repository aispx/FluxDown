use fluxdown_ui_components::sidebar_navigation_button;
use fluxdown_ui_theme::active_theme;
use gpui::{
    Context, Div, InteractiveElement as _, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement as _, Styled, div, prelude::FluentBuilder as _, px,
};
use gpui_component::{
    ActiveTheme as _, Icon, IconName, h_flex, scroll::ScrollableElement as _, v_flex,
};

use super::{DownloadView, SidebarItem, SidebarSection};

impl DownloadView {
    fn section_header(
        &self,
        id: &'static str,
        label: SharedString,
        section: SidebarSection,
        expanded: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let tokens = active_theme(cx).tokens();
        let chevron = if expanded {
            IconName::ChevronDown
        } else {
            IconName::ChevronRight
        };

        h_flex()
            .id(id)
            .h(px(28.))
            .px(tokens.spacing.sm)
            .items_center()
            .justify_between()
            .cursor_pointer()
            .rounded(tokens.radius.sm)
            .text_size(tokens.typography.xs.size)
            .font_weight(tokens.typography.xs.weight)
            .text_color(tokens.colors.muted_foreground)
            .hover(|style| style.bg(tokens.colors.muted))
            .on_click(cx.listener(move |this, _, _, cx| {
                match section {
                    SidebarSection::Status => {
                        this.status_expanded = !this.status_expanded;
                    }
                    SidebarSection::Queues => {
                        this.queues_expanded = !this.queues_expanded;
                    }
                    SidebarSection::Categories => {
                        this.categories_expanded = !this.categories_expanded;
                    }
                }
                cx.notify();
            }))
            .child(label)
            .child(Icon::new(chevron).size(px(12.)))
    }

    fn nav_item(
        &self,
        id: &'static str,
        item: SidebarItem,
        label: SharedString,
        icon: IconName,
        trailing: (&'static str, bool),
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let (count, show_dot) = trailing;
        let tokens = active_theme(cx).tokens();
        let dot_color = if item == SidebarItem::MainQueue {
            cx.theme().success
        } else {
            tokens.colors.muted_foreground
        };
        let trailing = h_flex()
            .flex_none()
            .items_center()
            .gap(tokens.spacing.xs)
            .when(show_dot, |this| {
                this.child(div().size(px(5.)).rounded_full().bg(dot_color))
            })
            .child(
                div()
                    .min_w(px(12.))
                    .text_right()
                    .text_size(tokens.typography.xs.size)
                    .child(count),
            );

        sidebar_navigation_button(
            id,
            label,
            Icon::new(icon).size(px(14.)),
            trailing,
            self.selected_item == item,
            cx,
        )
        .on_click(cx.listener(move |this, _, _, cx| {
            if this.selected_item != item {
                this.selected_item = item;
                cx.notify();
            }
        }))
    }

    fn render_status_section(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens().clone();
        v_flex()
            .px(tokens.spacing.sm)
            .pt(tokens.spacing.xs)
            .pb(tokens.spacing.xs)
            .border_b_1()
            .border_color(tokens.colors.border)
            .child(self.section_header(
                "download-status-toggle",
                self.strings.sidebar_status.clone(),
                SidebarSection::Status,
                self.status_expanded,
                cx,
            ))
            .when(self.status_expanded, |this| {
                this.child(self.nav_item(
                    "download-nav-all",
                    SidebarItem::All,
                    self.strings.status_all.clone(),
                    IconName::LayoutDashboard,
                    ("5", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-downloading",
                    SidebarItem::Downloading,
                    self.strings.status_downloading.clone(),
                    IconName::HardDrive,
                    ("0", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-completed",
                    SidebarItem::Completed,
                    self.strings.status_completed.clone(),
                    IconName::CircleCheck,
                    ("3", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-paused",
                    SidebarItem::Paused,
                    self.strings.status_paused.clone(),
                    IconName::Pause,
                    ("2", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-failed",
                    SidebarItem::Failed,
                    self.strings.status_error.clone(),
                    IconName::TriangleAlert,
                    ("0", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-seeding",
                    SidebarItem::Seeding,
                    self.strings.status_seeding.clone(),
                    IconName::ArrowUp,
                    ("0", false),
                    cx,
                ))
            })
    }

    fn render_queue_section(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens().clone();
        v_flex()
            .px(tokens.spacing.sm)
            .pt(tokens.spacing.xs)
            .pb(tokens.spacing.xs)
            .border_b_1()
            .border_color(tokens.colors.border)
            .child(self.section_header(
                "download-queue-toggle",
                self.strings.sidebar_queues.clone(),
                SidebarSection::Queues,
                self.queues_expanded,
                cx,
            ))
            .when(self.queues_expanded, |this| {
                this.child(self.nav_item(
                    "download-nav-main-queue",
                    SidebarItem::MainQueue,
                    self.strings.main_queue.clone(),
                    IconName::GalleryVerticalEnd,
                    ("5", true),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-later-queue",
                    SidebarItem::LaterQueue,
                    self.strings.later_queue.clone(),
                    IconName::Pause,
                    ("0", true),
                    cx,
                ))
            })
    }

    fn render_category_section(&self, cx: &mut Context<Self>) -> Div {
        let tokens = active_theme(cx).tokens().clone();
        v_flex()
            .px(tokens.spacing.sm)
            .pt(tokens.spacing.xs)
            .pb(tokens.spacing.md)
            .child(self.section_header(
                "download-category-toggle",
                self.strings.sidebar_category.clone(),
                SidebarSection::Categories,
                self.categories_expanded,
                cx,
            ))
            .when(self.categories_expanded, |this| {
                this.child(self.nav_item(
                    "download-nav-all-files",
                    SidebarItem::AllFiles,
                    self.strings.category_all.clone(),
                    IconName::Folder,
                    ("5", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-video",
                    SidebarItem::Video,
                    self.strings.category_video.clone(),
                    IconName::GalleryVerticalEnd,
                    ("0", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-audio",
                    SidebarItem::Audio,
                    self.strings.category_audio.clone(),
                    IconName::File,
                    ("0", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-document",
                    SidebarItem::Document,
                    self.strings.category_document.clone(),
                    IconName::File,
                    ("0", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-image",
                    SidebarItem::Image,
                    self.strings.category_image.clone(),
                    IconName::GalleryVerticalEnd,
                    ("0", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-program",
                    SidebarItem::Program,
                    self.strings.category_program.clone(),
                    IconName::HardDrive,
                    ("4", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-archive",
                    SidebarItem::Archive,
                    self.strings.category_archive.clone(),
                    IconName::Inbox,
                    ("1", false),
                    cx,
                ))
                .child(self.nav_item(
                    "download-nav-other",
                    SidebarItem::Other,
                    self.strings.category_other.clone(),
                    IconName::File,
                    ("0", false),
                    cx,
                ))
            })
    }

    pub(super) fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let surface = active_theme(cx).tokens().colors.surface;
        v_flex()
            .size_full()
            .min_w_0()
            .overflow_y_scrollbar()
            .bg(surface)
            .child(self.render_status_section(cx))
            .child(self.render_queue_section(cx))
            .child(self.render_category_section(cx))
    }
}
