#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SidebarItem {
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
pub(crate) enum SidebarSection {
    Status,
    Queues,
    Categories,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TaskState {
    Completed,
    Paused,
}

#[derive(Clone, Copy)]
pub(crate) enum TaskKind {
    Application,
    DiskImage,
    Mobile,
}

#[derive(Clone, Copy)]
pub(crate) struct TaskPreview {
    pub(crate) id: usize,
    pub(crate) name: &'static str,
    pub(crate) size: &'static str,
    pub(crate) size_bytes: u64,
    pub(crate) speed_bytes_per_second: Option<u64>,
    pub(crate) eta_seconds: Option<u64>,
    pub(crate) created_order: u32,
    pub(crate) kind: TaskKind,
    pub(crate) progress: f32,
    pub(crate) progress_label: &'static str,
    pub(crate) state: TaskState,
}

pub(crate) fn preview_tasks() -> Vec<TaskPreview> {
    vec![
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
    ]
}
