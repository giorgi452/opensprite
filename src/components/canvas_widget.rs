use crate::app_state::AppState;
use druid::kurbo::Rect;
use druid::piet::RenderContext;
use druid::{Color, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, Widget};

pub struct CanvasWidget;

impl Widget<AppState> for CanvasWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(mouse) | Event::MouseMove(mouse) if mouse.buttons.has_left() => {
                let pos = mouse.pos;
                let x = (pos.x / data.canvas.pixel_size).floor() as usize;
                let y = (pos.y / data.canvas.pixel_size).floor() as usize;
                // Add bounds check to prevent invalid access
                if x < data.canvas.canvas_width as usize && y < data.canvas.canvas_height as usize {
                    data.canvas.draw_pixel(x, y);
                    ctx.request_paint();
                }
            }
            Event::Wheel(mouse) => {
                let delta = mouse.wheel_delta.y;
                if delta > 0.0 {
                    data.canvas.zoom(1.0);
                    ctx.set_handled();
                } else if delta < 0.0 {
                    data.canvas.zoom(-1.0);
                    ctx.set_handled();
                }
            }
            _ => {}
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let pixel_size = data.canvas.pixel_size;
        let canvas_width = data.canvas.canvas_width as usize; // 32
        let canvas_height = data.canvas.canvas_height as usize; // 8

        // Ensure curr_frame is valid
        if data.canvas.curr_frame >= data.canvas.frames_data.len() {
            println!("Invalid frame index: {}", data.canvas.curr_frame);
            return;
        }

        for y in 0..canvas_height {
            // 0..8 for rows
            for x in 0..canvas_width {
                // 0..32 for columns
                // Safe access to pixel_data
                if let Some(color) = data.canvas.frames_data[data.canvas.curr_frame]
                    .get(y)
                    .and_then(|row| row.get(x))
                {
                    let rect = Rect::new(
                        x as f64 * pixel_size, // x for width
                        y as f64 * pixel_size, // y for height
                        (x + 1) as f64 * pixel_size,
                        (y + 1) as f64 * pixel_size,
                    );
                    // Convert Color to PietColor
                    ctx.fill(rect, color);
                    ctx.stroke(rect, &Color::GRAY, 0.5);
                }
            }
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &AppState,
        _data: &AppState,
        _env: &Env,
    ) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        data: &AppState,
        _env: &Env,
    ) -> Size {
        Size::new(
            data.canvas.canvas_width as f64 * data.canvas.pixel_size,
            data.canvas.canvas_height as f64 * data.canvas.pixel_size,
        )
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }
}
