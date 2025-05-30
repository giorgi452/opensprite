use crate::AppState;
use druid::{Color, Env, EventCtx, widget::Flex};
use std::sync::Arc;

use super::buttons::pencil::PencilButton;

#[derive(Clone)]
pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Flex<AppState> {
        Flex::row()
            .with_child(
                PencilButton::new().on_click(Toolbar::pencil),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    data.brush_size = (data.brush_size + 1).min(5);
                }),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    data.brush_size = (data.brush_size - 1).max(1);
                }),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    let mut created_frames = (*data.frames_data).clone();
                    created_frames.push(vec![
                        vec![Color::WHITE; data.canvas_width];
                        data.canvas_height
                    ]);
                    data.curr_frame = created_frames.len() - 1;
                    data.frames_data = Arc::new(created_frames);
                    data.pixel_data = Arc::new(data.frames_data[data.curr_frame].clone());
                }),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    if data.curr_frame > 0 {
                        data.curr_frame -= 1;
                        data.pixel_data = Arc::new(data.frames_data[data.curr_frame].clone());
                    }
                }),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    if data.curr_frame < data.frames_data.len() - 1 {
                        data.curr_frame += 1;
                        data.pixel_data = Arc::new(data.frames_data[data.curr_frame].clone());
                    }
                }),
            )
            .with_child(
                PencilButton::new().on_click(|_ctx, data: &mut AppState, _env| {
                    data.save_img("output.png");
                }),
            )
    }

    fn pencil(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.brush_color = Color::BLACK;
    }
}
