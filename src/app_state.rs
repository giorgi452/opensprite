use druid::{Data, Lens};

use crate::{
    controllers::{canvas_controller::CanvasController, project_controller::ProjectController},
    states::screen_state::Screen,
};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub canvas: CanvasController,
    pub project: ProjectController,
    #[data(ignore)]
    pub screen: Screen,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            project: ProjectController::new(),
            screen: Screen::INDEX,
            canvas: CanvasController::new(),
        }
    }
}
