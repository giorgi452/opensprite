use druid::{
    Widget, WidgetExt, WindowDesc,
    widget::{Button, Flex, Label},
};

use crate::app_state::AppState;

pub struct About;

impl About {
    pub fn new() -> WindowDesc<AppState> {
        WindowDesc::new(About::build_ui())
            .title("About OpenSprite")
            .window_size((300.0, 150.0))
    }

    pub fn build_ui() -> impl Widget<AppState> {
        Flex::column()
            .with_child(Label::new("OpenSprite v1.0"))
            .with_spacer(8.0)
            .with_child(Label::new("© 2025 OpenSprite"))
            .with_spacer(12.0)
            .with_child(Button::new("Close").on_click(|ctx, _, _| ctx.window().close()))
            .padding(10.0)
    }
}
