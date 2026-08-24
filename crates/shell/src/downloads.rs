use std::rc::Rc;

use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, px, size};
use gpui_component::{VirtualListScrollHandle, h_resizable, resizable_panel};
mod sidebar;
mod task_list;

use crate::strings::DownloadStrings;

const PROGRESS_COLUMN_WIDTH: f32 = 150.;
const SPEED_COLUMN_WIDTH: f32 = 90.;
const ETA_COLUMN_WIDTH: f32 = 84.;
const STATUS_COLUMN_WIDTH: f32 = 88.;
const GROUP_ROW_HEIGHT: f32 = 32.;
const TRAILING_COLUMNS_WIDTH: f32 =
    PROGRESS_COLUMN_WIDTH + SPEED_COLUMN_WIDTH + ETA_COLUMN_WIDTH + STATUS_COLUMN_WIDTH + 14.;

const TASK_ROW_HEIGHT: f32 = 64.;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SidebarItem {
    All,
    Downloading,
    Completed,
    Paused,
    Failed,
    Seeding,
    MainQueue,
    LaterQueue,
    AllFiles,
    Video,
    Audio,
    Document,
    Image,
    Program,
    Archive,
    Other,
}
#[derive(Debug, Clone, Copy)]
enum SidebarSection {
    Status,
    Queues,
    Categories,
}

#[derive(Clone, Copy)]
enum TaskState {
    Completed,
    Paused,
}
#[derive(Clone, Copy)]
enum TaskKind {
    Application,
    DiskImage,
    Mobile,
}

#[derive(Clone, Copy)]
struct TaskPreview {
    name: &'static str,
    size: &'static str,
    protocol: &'static str,
    progress: f32,
    kind: TaskKind,
    progress_label: &'static str,
    state: TaskState,
}

#[derive(Clone, Copy)]
enum ListItem {
    Group,
    Task(TaskPreview),
}

pub(crate) struct DownloadView {
    strings: DownloadStrings,
    selected_item: SidebarItem,
    status_expanded: bool,
    queues_expanded: bool,
    categories_expanded: bool,
    items: Vec<ListItem>,
    item_sizes: Rc<Vec<gpui::Size<gpui::Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
}

impl DownloadView {
    pub(crate) fn new(strings: DownloadStrings) -> Self {
        let items = vec![
            ListItem::Group,
            ListItem::Task(TaskPreview {
                name: "rufus-4.15.exe",
                size: "1.9 MB",
                protocol: "HTTP",
                kind: TaskKind::Application,
                progress: 1.,
                progress_label: "100.0%",
                state: TaskState::Completed,
            }),
            ListItem::Task(TaskPreview {
                name: "cachyos-desktop-linux-260809.iso",
                size: "3.0 GB",
                protocol: "HTTP",
                progress: 1.,
                kind: TaskKind::DiskImage,
                progress_label: "100.0%",
                state: TaskState::Completed,
            }),
            ListItem::Task(TaskPreview {
                name: "Gopeed-v1.9.3-android-x86_64.apk",
                size: "25.4 MB",
                protocol: "HTTP",
                progress: 1.,
                progress_label: "100.0%",
                kind: TaskKind::Mobile,
                state: TaskState::Completed,
            }),
            ListItem::Task(TaskPreview {
                name: "Gopeed-v1.9.3-macos-amd64.dmg",
                size: "39.0 MB",
                protocol: "HTTP",
                progress: 0.666,
                progress_label: "66.6%",
                state: TaskState::Paused,
                kind: TaskKind::DiskImage,
            }),
            ListItem::Task(TaskPreview {
                name: "Gopeed-v1.9.3-windows-amd64.exe",
                size: "25.2 MB",
                protocol: "HTTP",
                progress: 0.718,
                progress_label: "71.8%",
                state: TaskState::Paused,
                kind: TaskKind::Application,
            }),
        ];
        let item_sizes = Rc::new(
            items
                .iter()
                .map(|item| match item {
                    ListItem::Group => size(px(1.), px(GROUP_ROW_HEIGHT)),
                    ListItem::Task(_) => size(px(1.), px(TASK_ROW_HEIGHT)),
                })
                .collect(),
        );

        Self {
            strings,
            selected_item: SidebarItem::All,
            status_expanded: true,
            queues_expanded: true,
            categories_expanded: true,
            items,
            item_sizes,
            scroll_handle: VirtualListScrollHandle::new(),
        }
    }

    pub(crate) fn set_strings(&mut self, strings: DownloadStrings, cx: &mut Context<Self>) {
        self.strings = strings;
        cx.notify();
    }
}

impl Render for DownloadView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().min_w_0().min_h_0().child(
            h_resizable("downloads-content")
                .child(
                    resizable_panel()
                        .size(px(204.))
                        .size_range(px(184.)..px(320.))
                        .child(self.render_sidebar(cx)),
                )
                .child(resizable_panel().child(self.render_main(cx))),
        )
    }
}
