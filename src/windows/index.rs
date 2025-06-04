use std::{path::Path, sync::Arc};

use druid::{
    ImageBuf, UnitPoint, Widget, WindowDesc,
    widget::{Align, Button, Flex, Image, SizedBox},
};
use image::ImageReader;

use crate::{app_state::AppState, states::screen_state::Screen};

use super::main::Main;
pub struct Index;

impl Index {
    pub fn new() -> WindowDesc<AppState> {
        WindowDesc::new(Index::build_ui())
            .title("OpenSprite")
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
            .with_child(
                Button::new("new project").on_click(|ctx, data: &mut AppState, _env| {
                    data.screen = Arc::new(Screen::MAIN);
                    ctx.window().close();
                    ctx.new_window(Main::new());
                    ctx.set_handled();
                }),
            )
    }
}
