use app_state::AppState;
use components::{canvas_widget::CanvasWidget, toolbar::Toolbar};
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc, widget::Flex};

pub mod app_state;
pub mod components;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    AppLauncher::with_window(main_window).launch(AppState::new())
}

fn ui_builder() -> impl Widget<AppState> {
    let canvas = CanvasWidget;
    let toolbar = Toolbar::new();

    Flex::column()
        .with_child(toolbar)
        .with_child(canvas)
        .padding(10.0)
}
