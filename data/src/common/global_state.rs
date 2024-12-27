use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalState {
    #[default]
    Splashscreen,
    MainMenu,
    Game,
}
