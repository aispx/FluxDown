use std::{collections::HashSet, rc::Rc};

use gpui::{
    AppContext as _, Context, Entity, IntoElement, Modifiers, ParentElement, Render, Styled,
    Window, div, px, size,
};
use gpui_component::{ResizableState, VirtualListScrollHandle, h_resizable, resizable_panel};
mod sidebar;
mod task_list;

use crate::strings::DownloadStrings;

const SIZE_COLUMN_WIDTH: f32 = 72.;
const STATUS_COLUMN_WIDTH: f32 = 118.;
const SPEED_COLUMN_WIDTH: f32 = 82.;
const ETA_COLUMN_WIDTH: f32 = 98.;
const CREATED_COLUMN_WIDTH: f32 = 100.;
const SELECTION_COLUMN_WIDTH: f32 = 28.;
const TASK_ROW_HEIGHT: f32 = 38.;

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
    id: usize,
    name: &'static str,
    size: &'static str,
    kind: TaskKind,
    progress: f32,
    progress_label: &'static str,
    state: TaskState,
}

pub(crate) struct DownloadView {
    strings: DownloadStrings,
    selected_item: SidebarItem,
    selected_tasks: HashSet<usize>,
    selection_anchor: Option<usize>,
    status_expanded: bool,
    queues_expanded: bool,
    categories_expanded: bool,
    items: Vec<TaskPreview>,
    item_sizes: Rc<Vec<gpui::Size<gpui::Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
    resizable_state: Entity<ResizableState>,
    resizable_state_initialized: bool,
}

impl DownloadView {
    pub(crate) fn new(strings: DownloadStrings, cx: &mut Context<Self>) -> Self {
        let items = vec![
            TaskPreview {
                id: 0,
                name: "rufus-4.15.exe",
                size: "1.9 MB",
                kind: TaskKind::Application,
                progress: 1.,
                progress_label: "100.0%",
                state: TaskState::Completed,
            },
            TaskPreview {
                id: 1,
                name: "cachyos-desktop-linux-260809.iso",
                size: "3.0 GB",
                progress: 1.,
                kind: TaskKind::DiskImage,
                progress_label: "100.0%",
                state: TaskState::Completed,
            },
            TaskPreview {
                id: 2,
                name: "Gopeed-v1.9.3-android-x86_64.apk",
                size: "25.4 MB",
                progress: 1.,
                progress_label: "100.0%",
                kind: TaskKind::Mobile,
                state: TaskState::Completed,
            },
            TaskPreview {
                id: 3,
                name: "Gopeed-v1.9.3-macos-amd64.dmg",
                size: "39.0 MB",
                progress: 0.666,
                progress_label: "66.6%",
                state: TaskState::Paused,
                kind: TaskKind::DiskImage,
            },
            TaskPreview {
                id: 4,
                name: "Gopeed-v1.9.3-windows-amd64.exe",
                size: "25.2 MB",
                progress: 0.718,
                progress_label: "71.8%",
                state: TaskState::Paused,
                kind: TaskKind::Application,
            },
        ];
        let item_sizes = Rc::new(
            items
                .iter()
                .map(|_| size(px(1.), px(TASK_ROW_HEIGHT)))
                .collect(),
        );

        Self {
            strings,
            selected_item: SidebarItem::All,
            selected_tasks: HashSet::new(),
            selection_anchor: None,
            status_expanded: true,
            queues_expanded: true,
            categories_expanded: true,
            items,
            item_sizes,
            scroll_handle: VirtualListScrollHandle::new(),
            resizable_state: cx.new(|_| ResizableState::default()),
            resizable_state_initialized: false,
        }
    }

    pub(crate) fn set_strings(&mut self, strings: DownloadStrings, cx: &mut Context<Self>) {
        self.strings = strings;
        cx.notify();
    }

    fn select_task(&mut self, task_id: usize, modifiers: Modifiers) {
        if modifiers.shift
            && let Some(anchor) = self.selection_anchor
            && let Some(anchor_index) = self.items.iter().position(|task| task.id == anchor)
            && let Some(task_index) = self.items.iter().position(|task| task.id == task_id)
        {
            let (start, end) = if anchor_index <= task_index {
                (anchor_index, task_index)
            } else {
                (task_index, anchor_index)
            };
            self.selected_tasks.clear();
            self.selected_tasks
                .extend(self.items[start..=end].iter().map(|task| task.id));
            return;
        }

        if modifiers.secondary() {
            if !self.selected_tasks.remove(&task_id) {
                self.selected_tasks.insert(task_id);
            }
        } else {
            self.selected_tasks.clear();
            self.selected_tasks.insert(task_id);
        }
        self.selection_anchor = Some(task_id);
    }
}

impl Render for DownloadView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let main_panel_measured = self
            .resizable_state
            .read(cx)
            .sizes()
            .get(1)
            .is_some_and(|size| *size > px(1.));
        if !self.resizable_state_initialized && main_panel_measured {
            self.resizable_state
                .update(cx, |state, cx| state.reset_panel(1, cx));
            self.resizable_state_initialized = true;
        }

        div().size_full().min_w_0().min_h_0().child(
            h_resizable("downloads-content")
                .with_state(&self.resizable_state)
                .on_resize(|state, _, cx| {
                    state.update(cx, |state, cx| state.reset_panel(1, cx));
                })
                .child(
                    resizable_panel()
                        .size(px(160.))
                        .flex_none()
                        .size_range(px(148.)..px(280.))
                        .child(self.render_sidebar(cx)),
                )
                .child(resizable_panel().child(self.render_main(cx))),
        )
    }
}
