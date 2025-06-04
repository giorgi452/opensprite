use druid::{Widget, WindowDesc, widget::Label};

use crate::app_state::AppState;

pub struct Error;

impl Error {
    pub fn new() -> WindowDesc<AppState> {
        WindowDesc::new(Error::build_ui()).title("Can't open window")
    }

    pub fn build_ui() -> impl Widget<AppState> {
        Label::new("Can't found")
    }
}
