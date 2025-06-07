use druid::{
    LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc,
    widget::{Align, Flex, SizedBox},
};

use crate::{
    app_state::AppState,
    components::{canvas_widget::CanvasWidget, menu::MenuC, toolbar::Toolbar},
    controllers::widget_controller::WidgetController,
};
pub struct Main;

impl Main {
    pub fn new() -> WindowDesc<AppState> {
        WindowDesc::new(Main::build_ui())
            .title(LocalizedString::new("opensprite").with_placeholder("OpenSprite"))
            .menu(MenuC::basic)
            .show_titlebar(true)
    }

    pub fn build_ui() -> impl Widget<AppState> {
        let toolbar = Toolbar::new();

        let canvas = SizedBox::new(Align::horizontal(UnitPoint::CENTER, CanvasWidget))
            .height(f64::INFINITY)
            .width(f64::INFINITY);

        Flex::row()
            .with_child(toolbar)
            .with_spacer(50.0)
            .with_flex_child(canvas, 1.0)
            .must_fill_main_axis(true)
            .padding(10.0)
            .controller(WidgetController)
    }
}
