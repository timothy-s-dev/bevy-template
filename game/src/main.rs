mod common;
mod plugins;
mod prelude;

use crate::plugins::game::GamePlugin;
use crate::prelude::*;
use bevy::log::LogPlugin;
use bevy::render::camera::ScalingMode;
use plugins::main_menu::MainMenuPlugin;
use plugins::splashscreen::SplashscreenPlugin;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let has_args = args.len() > 1;

    let mut app = App::new();

    if has_args && cfg!(feature = "dev") {
        app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
        app.add_plugins(bevy_mod_debugdump::CommandLineArgs);
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app.add_plugins(AudioPlugin);

    app.init_state::<GlobalState>()
        .enable_state_scoped_entities::<GlobalState>()
        .add_systems(Startup, setup)
        .add_plugins((SplashscreenPlugin, MainMenuPlugin, GamePlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1080.0,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}
