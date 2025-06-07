use druid::{
    Env, Event, EventCtx, LifeCycleCtx, UpdateCtx,
    widget::{Controller, Widget},
};
use std::env;

use crate::app_state::AppState;

pub struct NPController;

impl<W: Widget<AppState>> Controller<AppState, W> for NPController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        // Save old values
        let old_name = data.project.name.clone();
        let old_path = data.project.path.clone();

        // Forward event to children (TextBox, etc.)
        child.event(ctx, event, data, env);

        // Detect path manually changed
        if old_path != data.project.path && old_name == data.project.name {
            data.project.path_overridden = true;
        }

        // If name changed, and path not overridden, sync path
        if old_name != data.project.name && !data.project.path_overridden {
            let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
            data.project.path = format!("/home/{}/Pictures/{}", username, data.project.name);
        }
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env);
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env);
    }
}
