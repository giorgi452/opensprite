use druid::{
    LocalizedString, Menu, MenuItem, UnitPoint, Widget, WidgetExt, WindowDesc,
    widget::{Align, Flex, SizedBox},
};

use crate::{
    app_state::AppState, commands::show_about::SHOW_ABOUT, components::{canvas_widget::CanvasWidget, toolbar::Toolbar}, controllers::widget_controller::WidgetController
};
pub struct Main;

impl Main {
    pub fn new() -> WindowDesc<AppState> {
        WindowDesc::new(Main::build_ui())
            .title(LocalizedString::new("opensprite").with_placeholder("OpenSprite"))
            .menu(|_window_id, _data: &AppState, _env| -> Menu<AppState> {
                Menu::new(LocalizedString::new("opensprite"))
                    .entry(
                        Menu::new(LocalizedString::new("File")).entry(
                            MenuItem::new(LocalizedString::new("Exit"))
                                .command(druid::commands::QUIT_APP),
                        ),
                    )
                    .separator()
                    .entry(
                        Menu::new(LocalizedString::new("Help")).entry(
                            MenuItem::new(LocalizedString::new("About")).command(SHOW_ABOUT),
                        ),
                    )
            })
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
