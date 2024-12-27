use crate::prelude::*;
use bevy::prelude::*;
use data::splashscreen::*;
use std::time::Duration;

fn splashscreen_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name("splashscreen_timer"),
        StateScoped(GlobalState::Splashscreen),
        SplashscreenTimer(Timer::new(Duration::from_secs(3), TimerMode::Once)),
    ));
    commands.spawn((
        Name("splashscreen_background"),
        StateScoped(GlobalState::Splashscreen),
        Sprite::from_image(asset_server.load("splashscreen.png")),
    ));
}

fn splashscreen_update_system(
    time: Res<Time>,
    mut timer_query: Query<&mut SplashscreenTimer>,
    mut next_global_state: ResMut<NextState<GlobalState>>,
) {
    let mut timer = get_single_mut_or_return!(timer_query);
    timer.0.tick(time.delta());
    if timer.0.finished() {
        next_global_state.set(GlobalState::MainMenu);
    }
}

pub struct SplashscreenPlugin;
impl Plugin for SplashscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GlobalState::Splashscreen),
            splashscreen_setup_system,
        );
        app.add_systems(
            Update,
            splashscreen_update_system.run_if(in_state(GlobalState::Splashscreen)),
        );
    }
}
