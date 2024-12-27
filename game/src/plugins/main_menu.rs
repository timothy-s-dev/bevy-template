use std::time::Duration;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Resource)]
pub struct MainMenuMusic(Handle<AudioInstance>);

fn main_menu_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let instance = audio.play(asset_server.load("music/Awkward Meeting.mp3"))
        .looped()
        .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::OutPowi(2)))
        .with_volume(0.5)
        .handle();
    commands.insert_resource(MainMenuMusic(instance));

    commands.spawn((
        Name("main_menu_background"),
        StateScoped(GlobalState::MainMenu),
        Sprite::from_image(asset_server.load("splashscreen.png")),
    ));
    commands
        .spawn((
            Name("main_menu_container"),
            StateScoped(GlobalState::MainMenu),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        border: UiRect::all(Val::Px(5.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(Color::WHITE),
                ))
                .with_child((
                    Text::new("Play"),
                    TextFont {
                        font: asset_server.load("fonts/DarkPoint.ttf"),
                        font_size: 32.,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
        });
}

fn main_menu_cleanup_system(mut commands: Commands) {
    commands.remove_resource::<MainMenuMusic>();
}

fn main_menu_interaction_system(
    mut query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<Button>)>,
    mut transition_timer: Local<Timer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GlobalState>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    music: Res<MainMenuMusic>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    // The timer defaults to unpaused with 0 seconds remaining, and would trigger immediately
    // unless we pause it on the first run of the system here
    if *transition_timer == Timer::default() {
        transition_timer.pause();
    }
    if transition_timer.tick(time.delta()).just_finished() {
        next_state.set(GlobalState::Game);
    }

    for (interaction, mut border_color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *transition_timer = Timer::from_seconds(1., TimerMode::Once);
                border_color.0 = Color::srgb(0.75, 0.75, 0.75).into();
                audio.play(asset_server.load("sfx/menu/select.wav"));
                if let Some(instance) = audio_instances.get_mut(&music.0) {
                    instance.stop(AudioTween::new(
                        Duration::from_secs(2),
                        AudioEasing::OutPowi(2),
                    ));
                }
            }
            Interaction::Hovered => {
                border_color.0 = Color::srgb(0.5, 0.5, 0.5).into();
                audio.play(asset_server.load("sfx/menu/hover.wav"));
            }
            Interaction::None => {
                border_color.0 = Color::BLACK.into();
            }
        }
    }
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::MainMenu), main_menu_setup_system);
        app.add_systems(
            Update,
            main_menu_interaction_system.run_if(in_state(GlobalState::MainMenu)),
        );
        app.add_systems(OnExit(GlobalState::MainMenu), main_menu_cleanup_system);
    }
}
