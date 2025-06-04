use app_state::AppState;
use druid::{AppLauncher, PlatformError};
use windows::index::Index;

pub mod app_state;
pub mod commands;
pub mod components;
pub mod controllers;
pub mod states;
pub mod windows;

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(Index::new()).launch(AppState::new())
}
