use druid::{
    Color, Data, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, Point, RenderContext, Size, Widget,
    kurbo::{Line, Rect, RoundedRect},
    piet::{LinearGradient, UnitPoint},
    widget::{Label, Painter, WidgetExt},
};

pub struct PencilButton<T> {
    label: Box<dyn Widget<T>>,
}

impl<T: Data> PencilButton<T> {
    pub fn new() -> Self {
        let label = Label::new("")
            .with_text_color(Color::WHITE)
            .center()
            .fix_size(50.0, 30.0)
            .background(Painter::new(|ctx, _data, _env| {
                let rect = ctx.size().to_rect();
                let rect_inner = rect.inset(-1.0);

                let gradient = LinearGradient::new(
                    UnitPoint::TOP,
                    UnitPoint::BOTTOM,
                    (Color::rgb8(180, 180, 180), Color::rgb8(120, 120, 120)),
                );
                ctx.fill(rect_inner, &gradient);

                let shadow_rect = rect_inner.inset(-2.0).with_origin(Point::new(2.0, 2.0));
                ctx.fill(shadow_rect, &Color::rgba8(0, 0, 0, 50));

                let rounded_rect = RoundedRect::from_rect(rect, 4.0);
                ctx.stroke(rounded_rect, &Color::BLACK, 2.0);

                if ctx.is_hot() {
                    ctx.fill(rect_inner, &Color::rgba8(255, 255, 255, 30));
                }

                let center = rect.center();
                let scale = 8.0;
                let offset_x = center.x - 20.0;
                let offset_y = center.y - 10.0;

                let body = [
                    Rect::new(offset_x, offset_y, offset_x + scale, offset_y + scale),
                    Rect::new(
                        offset_x + scale,
                        offset_y,
                        offset_x + 2.0 * scale,
                        offset_y + scale,
                    ),
                    Rect::new(
                        offset_x,
                        offset_y + scale,
                        offset_x + scale,
                        offset_y + 2.0 * scale,
                    ),
                    Rect::new(
                        offset_x + 2.0 * scale,
                        offset_y + scale,
                        offset_x + 3.0 * scale,
                        offset_y + 2.0 * scale,
                    ),
                    Rect::new(
                        offset_x + scale,
                        offset_y + 2.0 * scale,
                        offset_x + 2.0 * scale,
                        offset_y + 3.0 * scale,
                    ),
                ];
                for r in body.iter() {
                    ctx.fill(*r, &Color::BLACK);
                }

                let accents = [Rect::new(
                    offset_x + scale,
                    offset_y + scale,
                    offset_x + 2.0 * scale,
                    offset_y + 2.0 * scale,
                )];
                for r in accents.iter() {
                    ctx.fill(*r, &Color::WHITE);
                }

                let fuse_start = Point::new(offset_x + 2.0 * scale, offset_y);
                let fuse_end = Point::new(offset_x + 3.0 * scale, offset_y - scale);
                let fuse = Line::new(fuse_start, fuse_end);
                ctx.stroke(fuse, &Color::BLACK, 2.0);
            }));
        PencilButton {
            label: Box::new(label),
        }
    }

    pub fn on_click<F>(self, f: F) -> impl Widget<T>
    where
        F: Fn(&mut EventCtx, &mut T, &Env) + 'static,
    {
        PencilButtonController {
            widget: self.label,
            on_click: Box::new(f),
        }
    }
}

struct PencilButtonController<T, F> {
    widget: Box<dyn Widget<T>>,
    on_click: Box<F>,
}

impl<T: Data, F: Fn(&mut EventCtx, &mut T, &Env)> Widget<T> for PencilButtonController<T, F> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && ctx.is_hot() {
                    ctx.set_active(false);
                    (self.on_click)(ctx, data, env);
                    ctx.request_paint();
                }
            }
            _ => {}
        }
        self.widget.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.widget.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.widget.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        self.widget.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &Env) {
        self.widget.paint(ctx, data, env);
    }
}
