use std::sync::Arc;

use app_state::AppState;
use canvas_widget::CanvasWidget;
use druid::{
    AppLauncher, Color, PlatformError, Widget, WidgetExt, WindowDesc,
    widget::{Button, Flex},
};

pub mod app_state;
pub mod canvas_widget;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    AppLauncher::with_window(main_window).launch(AppState::new())
}

fn ui_builder() -> impl Widget<AppState> {
    let canvas = CanvasWidget;

    let toolbar = Flex::row()
        .with_child(
            Button::new("Black").on_click(|_ctx, data: &mut AppState, _env| {
                data.brush_color = Color::BLACK;
            }),
        )
        .with_child(
            Button::new("Red").on_click(|_ctx, data: &mut AppState, _env| {
                data.brush_color = Color::rgb8(255, 0, 0);
            }),
        )
        .with_child(Button::new("Increase Brush Size").on_click(
            |_ctx, data: &mut AppState, _env| {
                data.brush_size = (data.brush_size + 1).min(5);
            },
        ))
        .with_child(Button::new("Decrease Brush Size").on_click(
            |_ctx, data: &mut AppState, _env| {
                data.brush_size = (data.brush_size - 1).max(1);
            },
        ))
        .with_child(
            Button::new("Add Frame").on_click(|_ctx, data: &mut AppState, _env| {
                let mut created_frames = (*data.frames_data).clone();
                created_frames.push(vec![
                    vec![Color::WHITE; data.canvas_width];
                    data.canvas_height
                ]);
                data.curr_frame = created_frames.len() - 1; // Update index before moving
                data.frames_data = Arc::new(created_frames);
                data.pixel_data = Arc::new(data.frames_data[data.curr_frame].clone());
            }),
        )
        .with_child(
            Button::new("Previous Frame").on_click(|_ctx, data: &mut AppState, _env| {
                if data.curr_frame > 0 {
                    data.curr_frame -= 1;
                    data.pixel_data = Arc::new(data.frames_data[data.curr_frame].clone());
                }
            }),
        )
        .with_child(
            Button::new("Next Frame").on_click(|_ctx, data: &mut AppState, _env| {
                if data.curr_frame < data.frames_data.len() - 1 {
                    data.curr_frame += 1;
                    data.pixel_data = Arc::new(data.frames_data[data.curr_frame].clone());
                }
            }),
        )
        .with_child(
            Button::new("Save PNG").on_click(|_ctx, data: &mut AppState, _env| {
                data.save_img("output.png");
            }),
        );

    Flex::column()
        .with_child(toolbar)
        .with_child(canvas)
        .padding(10.0)
}
