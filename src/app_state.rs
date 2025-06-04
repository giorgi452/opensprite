use std::sync::Arc;

use druid::{Data, Lens};

use crate::{controllers::canvas_controller::CanvasController, states::screen_state::Screen};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub canvas: CanvasController,
    #[data(ignore)]
    pub screen: Arc<Screen>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            screen: Arc::new(Screen::INDEX),
            canvas: CanvasController::new(),
        }
    }
}
