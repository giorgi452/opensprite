use druid::{Env, LocalizedString, Menu, MenuItem, WindowId};

use crate::{
    app_state::AppState,
    commands::{goto_index::GOTO_INDEX, show_about::SHOW_ABOUT},
    states::screen_state::Screen,
};

pub struct MenuC;

impl MenuC {
    pub fn basic(_window_id: Option<WindowId>, data: &AppState, _env: &Env) -> Menu<AppState> {
        let mut file_menu = Menu::new(LocalizedString::new("File"));

        match data.screen {
            Screen::MAIN => {
                file_menu = file_menu
                    .entry(MenuItem::new(LocalizedString::new("Go to Start")).command(GOTO_INDEX));
            }
            _ => {}
        }

        file_menu = file_menu
            .entry(MenuItem::new(LocalizedString::new("Exit")).command(druid::commands::QUIT_APP));

        Menu::new(LocalizedString::new("opensprite"))
            .entry(file_menu)
            .separator()
            .entry(
                Menu::new(LocalizedString::new("Help"))
                    .entry(MenuItem::new(LocalizedString::new("About")).command(SHOW_ABOUT)),
            )
    }
}
