//! `fluxdown-desktop` 的薄入口；窗口与 UI 状态由 `fluxdown_ui_shell` 负责。

use std::process::ExitCode;

fn main() -> ExitCode {
    match fluxdown_ui_shell::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("failed to start FluxDown desktop client: {error:#}");
            ExitCode::FAILURE
        }
    }
}
