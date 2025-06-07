use druid::{Env, Event, EventCtx, Widget};

use crate::{
    app_state::AppState,
    commands::{goto_index::GOTO_INDEX, show_about::SHOW_ABOUT},
    windows::{about::About, index::Index},
};

pub struct WidgetController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for WidgetController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::Command(cmd) = event {
            if cmd.is(SHOW_ABOUT) {
                ctx.new_window(About::new());
                ctx.set_handled();
                return;
            }
            if cmd.is(GOTO_INDEX) {
                ctx.new_window(Index::new());
                ctx.window().close();
                ctx.set_handled();
            }
        }
        child.event(ctx, event, data, env);
    }
}
