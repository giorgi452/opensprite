use std::path::Path;

use druid::{
    ImageBuf, UnitPoint, Widget, WidgetExt, WindowConfig, WindowDesc,
    widget::{Align, Button, Flex, Image, SizedBox},
};
use image::ImageReader;

use crate::{
    app_state::AppState, components::menu::MenuC, controllers::widget_controller::WidgetController,
};

use super::new_project::NewProject;

pub struct Index;

impl Index {
    pub fn new() -> WindowDesc<AppState> {
        WindowDesc::new(Index::build_ui())
            .title("OpenSprite")
            .menu(MenuC::basic)
            .show_titlebar(true)
            .window_size((300.0, 150.0))
    }

    pub fn load_image() -> ImageBuf {
        let dyn_img = ImageReader::open(Path::new("assets/logo.png"))
            .expect("File not found")
            .decode()
            .expect("Failed to decode image")
            .to_rgba8();

        let (width, height) = dyn_img.dimensions();
        let raw = dyn_img.into_raw();

        ImageBuf::from_raw(
            raw,
            druid::piet::ImageFormat::RgbaSeparate,
            width as usize,
            height as usize,
        )
    }

    pub fn build_ui() -> impl Widget<AppState> {
        let image = Self::load_image();
        Flex::column()
            .with_child(SizedBox::new(Align::horizontal(
                UnitPoint::CENTER,
                Image::new(image),
            )))
            .with_spacer(30.0)
            .with_child(
                Flex::column()
                    .with_child(
                        Button::new("New Project")
                            .on_click(|ctx, data: &mut AppState, env| {
                                ctx.new_sub_window(
                                    WindowConfig::default()
                                        .set_title("Sub Window")
                                        .set_size((300.0, 120.0)),
                                    NewProject::build_ui(),
                                    AppState,
                                    env,
                                );

                                ctx.set_handled();
                            })
                            .fix_height(30.0)
                            .expand_width(),
                    )
                    .with_spacer(10.0)
                    .with_child(
                        Button::new("Open Recents")
                            .on_click(|ctx, _data: &mut AppState, _env| {
                                ctx.submit_command(druid::commands::QUIT_APP);
                            })
                            .fix_height(30.0)
                            .expand_width(),
                    )
                    .with_spacer(10.0)
                    .with_child(
                        Button::new("Exit Program")
                            .on_click(|ctx, _data: &mut AppState, _env| {
                                ctx.submit_command(druid::commands::QUIT_APP);
                            })
                            .fix_height(30.0)
                            .expand_width(),
                    )
                    .fix_width(200.0),
            )
            .controller(WidgetController)
    }
}
