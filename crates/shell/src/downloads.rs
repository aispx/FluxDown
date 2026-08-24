use gpui::{
    AppContext as _, Context, Entity, IntoElement, KeyBinding, ParentElement, Render, Styled,
    Window, actions, div, px,
};
use gpui_component::{ResizableState, h_resizable, resizable_panel, table::TableState};
mod sidebar;
mod task_list;

use task_list::DownloadTableDelegate;

use crate::strings::DownloadStrings;

actions!(downloads, [SelectAllTasks]);

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    size_bytes: u64,
    speed_bytes_per_second: Option<u64>,
    eta_seconds: Option<u64>,
    created_order: u32,
    kind: TaskKind,
    progress: f32,
    progress_label: &'static str,
    state: TaskState,
}

pub(crate) struct DownloadView {
    strings: DownloadStrings,
    selected_item: SidebarItem,
    status_expanded: bool,
    queues_expanded: bool,
    categories_expanded: bool,
    table_state: Entity<TableState<DownloadTableDelegate>>,
    resizable_state: Entity<ResizableState>,
    resizable_state_initialized: bool,
}

impl DownloadView {
    pub(crate) fn new(
        strings: DownloadStrings,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.bind_keys([KeyBinding::new("ctrl-a", SelectAllTasks, Some("DataTable"))]);
        let items = vec![
            TaskPreview {
                id: 0,
                name: "rufus-4.15.exe",
                size: "1.9 MB",
                size_bytes: 1_900_000,
                speed_bytes_per_second: None,
                eta_seconds: None,
                created_order: 0,
                kind: TaskKind::Application,
                progress: 1.,
                progress_label: "100.0%",
                state: TaskState::Completed,
            },
            TaskPreview {
                id: 1,
                name: "cachyos-desktop-linux-260809.iso",
                size: "3.0 GB",
                size_bytes: 3_000_000_000,
                speed_bytes_per_second: None,
                eta_seconds: None,
                created_order: 1,
                progress: 1.,
                kind: TaskKind::DiskImage,
                progress_label: "100.0%",
                state: TaskState::Completed,
            },
            TaskPreview {
                id: 2,
                name: "Gopeed-v1.9.3-android-x86_64.apk",
                size: "25.4 MB",
                size_bytes: 25_400_000,
                speed_bytes_per_second: None,
                eta_seconds: None,
                created_order: 2,
                progress: 1.,
                progress_label: "100.0%",
                kind: TaskKind::Mobile,
                state: TaskState::Completed,
            },
            TaskPreview {
                id: 3,
                name: "Gopeed-v1.9.3-macos-amd64.dmg",
                size: "39.0 MB",
                size_bytes: 39_000_000,
                speed_bytes_per_second: None,
                eta_seconds: None,
                created_order: 3,
                progress: 0.666,
                progress_label: "66.6%",
                state: TaskState::Paused,
                kind: TaskKind::DiskImage,
            },
            TaskPreview {
                id: 4,
                name: "Gopeed-v1.9.3-windows-amd64.exe",
                size: "25.2 MB",
                size_bytes: 25_200_000,
                speed_bytes_per_second: None,
                eta_seconds: None,
                created_order: 4,
                progress: 0.718,
                progress_label: "71.8%",
                state: TaskState::Paused,
                kind: TaskKind::Application,
            },
        ];
        let table_state = cx.new(|cx| {
            TableState::new(
                DownloadTableDelegate::new(strings.clone(), items),
                window,
                cx,
            )
            .row_selectable(false)
            .col_selectable(false)
        });

        Self {
            strings,
            selected_item: SidebarItem::All,
            status_expanded: true,
            queues_expanded: true,
            categories_expanded: true,
            table_state,
            resizable_state: cx.new(|_| ResizableState::default()),
            resizable_state_initialized: false,
        }
    }

    pub(crate) fn set_strings(&mut self, strings: DownloadStrings, cx: &mut Context<Self>) {
        self.strings = strings.clone();
        self.table_state.update(cx, |table, cx| {
            table.delegate_mut().set_strings(strings);
            table.refresh(cx);
        });
        cx.notify();
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
