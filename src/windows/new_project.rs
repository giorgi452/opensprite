use std::sync::Arc;

use druid::text::TextAlignment;
use druid::widget::{Align, SizedBox, TextBox};
use druid::{
    Color, FontDescriptor, FontFamily, LensExt, UnitPoint, WindowDesc, WindowId, commands,
};
use druid::{
    Widget, WidgetExt,
    widget::{Button, Flex, Label},
};

use crate::app_state::AppState;
use crate::controllers::canvas_controller::CanvasController;
use crate::controllers::np_controller::NPController;
use crate::controllers::project_controller::ProjectController;
use crate::helpers::usize_to_string::UsizeToString;
use crate::states::screen_state::Screen;

use super::main::Main;

pub struct NewProject {
    pub parent_window_id: WindowId,
}

impl NewProject {
    pub fn new(parent_window_id: WindowId) -> WindowDesc<AppState> {
        WindowDesc::new(NewProject::build_ui(parent_window_id))
            .title("OpenSprite")
            .window_size((300.0, 150.0))
    }

    pub fn build_ui(parent_window_id: WindowId) -> impl Widget<AppState> {
        Flex::column()
            .with_child(
                Label::new("New Project")
                    .with_font(
                        FontDescriptor::new(FontFamily::SYSTEM_UI)
                            .with_size(32.0)
                            .with_weight(druid::piet::FontWeight::BOLD),
                    )
                    .with_text_alignment(TextAlignment::Start)
                    .align_horizontal(UnitPoint::LEFT),
            )
            .with_spacer(15.0)
            .with_child(SizedBox::new(Align::horizontal(
                UnitPoint::LEFT,
                Flex::column()
                    .with_child(SizedBox::new(Align::horizontal(
                        UnitPoint::LEFT,
                        Label::new("Name"),
                    )))
                    .with_child(
                        TextBox::new()
                            .with_placeholder("Name")
                            .lens(AppState::project.then(ProjectController::name))
                            .expand_width()
                            .padding(4.0),
                    ),
            )))
            .with_spacer(10.0)
            .with_child(SizedBox::new(Align::horizontal(
                UnitPoint::LEFT,
                Flex::column()
                    .with_child(SizedBox::new(Align::horizontal(
                        UnitPoint::LEFT,
                        Label::new("Path"),
                    )))
                    .with_child(
                        TextBox::new()
                            .with_placeholder("Path")
                            .lens(AppState::project.then(ProjectController::path))
                            .expand_width()
                            .padding(4.0),
                    ),
            )))
            .with_spacer(10.0)
            .with_child(SizedBox::new(Align::horizontal(
                UnitPoint::LEFT,
                Flex::column()
                    .with_child(SizedBox::new(Align::horizontal(
                        UnitPoint::LEFT,
                        Label::new("Canvas Width"),
                    )))
                    .with_child(
                        TextBox::new()
                            .with_placeholder("Canvas Width")
                            .lens(
                                AppState::canvas
                                    .then(CanvasController::canvas_width)
                                    .then(UsizeToString),
                            )
                            .expand_width()
                            .padding(4.0),
                    ),
            )))
            .with_spacer(10.0)
            .with_child(SizedBox::new(Align::horizontal(
                UnitPoint::LEFT,
                Flex::column()
                    .with_child(SizedBox::new(Align::horizontal(
                        UnitPoint::LEFT,
                        Label::new("Canvas Height"),
                    )))
                    .with_child(
                        TextBox::new()
                            .with_placeholder("Canvas Height")
                            .lens(
                                AppState::canvas
                                    .then(CanvasController::canvas_height)
                                    .then(UsizeToString),
                            )
                            .expand_width()
                            .padding(4.0),
                    ),
            )))
            .with_spacer(12.0)
            .with_child(SizedBox::new(Align::horizontal(
                UnitPoint::BOTTOM_RIGHT,
                Flex::row()
                    .with_child(Button::new("Close").on_click(|ctx, _, _| ctx.window().close()))
                    .with_spacer(2.0)
                    .with_child(Button::new("Create Project").on_click(
                        move |ctx, data: &mut AppState, _| {
                            data.canvas.pixel_data =
                                Arc::new(vec![
                                    vec![Color::WHITE; data.canvas.canvas_width];
                                    data.canvas.canvas_height
                                ]);
                            data.canvas.frames_data =
                                Arc::new(vec![data.canvas.pixel_data.clone().to_vec()]);
                            data.screen = Screen::MAIN;
                            ctx.submit_command(commands::CLOSE_WINDOW.to(parent_window_id));
                            ctx.window().close();
                            ctx.new_window(Main::new());
                            ctx.set_handled();
                        },
                    )),
            )))
            .padding(10.0)
            .controller(NPController)
    }
}
