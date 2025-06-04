use crate::AppState;
use druid::{Color, Env, EventCtx, widget::Flex};
use std::sync::Arc;

use super::buttons::{brush_size::BrushSizeButton, pencil::PencilButton};

#[derive(Clone)]
pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Flex<AppState> {
        Flex::column()
            .with_child(PencilButton::new().on_click(Toolbar::pencil))
            .with_child(BrushSizeButton::new().on_click(Toolbar::brush_size))
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    data.canvas.brush_size = (data.canvas.brush_size - 1).max(1);
                }),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    let mut created_frames = (*data.canvas.frames_data).clone();
                    created_frames.push(vec![
                        vec![Color::WHITE; data.canvas.canvas_width];
                        data.canvas.canvas_height
                    ]);
                    data.canvas.curr_frame = created_frames.len() - 1;
                    data.canvas.frames_data = Arc::new(created_frames);
                    data.canvas.pixel_data =
                        Arc::new(data.canvas.frames_data[data.canvas.curr_frame].clone());
                }),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    if data.canvas.curr_frame > 0 {
                        data.canvas.curr_frame -= 1;
                        data.canvas.pixel_data =
                            Arc::new(data.canvas.frames_data[data.canvas.curr_frame].clone());
                    }
                }),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    if data.canvas.curr_frame < data.canvas.frames_data.len() - 1 {
                        data.canvas.curr_frame += 1;
                        data.canvas.pixel_data = Arc::new(data.canvas.frames_data[data.canvas.curr_frame].clone());
                    }
                }),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    data.canvas.save_img("output.png");
                }),
            )
    }

    fn pencil(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.canvas.brush_color = Color::BLACK;
    }
    fn brush_size(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.canvas.brush_size = (data.canvas.brush_size + 1).min(5);
    }
}
