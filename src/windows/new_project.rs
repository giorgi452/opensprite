use druid::text::TextAlignment;
use druid::widget::{Align, SizedBox, TextBox};
use druid::{FontDescriptor, FontFamily, LensExt, UnitPoint, WindowConfig};
use druid::{
    Widget, WidgetExt,
    widget::{Button, Flex, Label},
};

use crate::app_state::AppState;
use crate::controllers::canvas_controller::CanvasController;
use crate::controllers::np_controller::NPController;
use crate::controllers::project_controller::ProjectController;
use crate::helpers::usize_to_string::UsizeToString;

use super::main::Main;

pub struct NewProject;

impl NewProject {
    pub fn build_ui() -> impl Widget<AppState> {
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
                    .with_child(Button::new("Create Project").on_click(|ctx, _, _| {
                        ctx.window().close();
                        ctx.new_window(Main::new());
                        ctx.set_handled();
                    })),
            )))
            .padding(10.0)
            .controller(NPController)
    }
}
