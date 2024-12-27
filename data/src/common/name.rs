use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Name(pub &'static str);
