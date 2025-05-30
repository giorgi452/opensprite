use crate::app_state::AppState;
use druid::kurbo::Rect;
use druid::piet::RenderContext;
use druid::{Color, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, PaintCtx, Widget};

pub struct CanvasWidget;

impl Widget<AppState> for CanvasWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(mouse) | Event::MouseMove(mouse) if mouse.buttons.has_left() => {
                let pos = mouse.pos;
                let x = (pos.x / data.pixel_size) as usize;
                let y = (pos.y / data.pixel_size) as usize;
                data.draw_pixel(x, y);
                ctx.request_paint();
            }
            Event::Wheel(mouse) => {
                let delta = mouse.wheel_delta.y;
                if delta > 0.0 {
                    data.zoom(1.0);
                    ctx.set_handled();
                } else if delta < 0.0 {
                    data.zoom(-1.0);
                    ctx.set_handled();
                }
            }
            _ => {}
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let pixel_size = data.pixel_size;

        for y in 0..data.canvas_width {
            for x in 0..data.canvas_height {
                let rect = Rect::new(
                    x as f64 * pixel_size,
                    y as f64 * pixel_size,
                    (x + 1) as f64 * pixel_size,
                    (y + 1) as f64 * pixel_size,
                );
                ctx.fill(rect, &data.pixel_data[y][x]);
                ctx.stroke(rect, &Color::GRAY, 0.5);
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
    ) -> druid::Size {
        druid::Size::new(
            data.canvas_width as f64 * data.pixel_size,
            data.canvas_height as f64 * data.pixel_size,
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
