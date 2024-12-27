use crate::prelude::*;
use bevy::prelude::*;

fn game_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name("game_background"),
        StateScoped(GlobalState::Game),
        Sprite::from_image(asset_server.load("splashscreen.png")),
    ));
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::Game), game_setup_system);
    }
}
