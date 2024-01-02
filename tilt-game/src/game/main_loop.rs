use std::str::FromStr;

use bevy::{prelude::*, window::close_on_esc};
use console::Term;

use classes::{Level, RoundResult};
use game_classes::{MapData, RoundStats};

use super::{init::init, logic::generate_map_str};
use crate::{assets::load_map_data, cli::Action, Error, Result};

#[derive(Resource)]
struct CurrentLevel(Level);

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayState {
    Playing,
    Lost,
    Won,
}

impl Default for PlayState {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Resource, Default)]
pub struct ExtraText(Vec<TextSection>);

#[derive(Resource)]
struct WaitingToLoadTimer(Timer);

pub fn run() -> Result<()> {
    init()?;

    let current_level = Level::Lv1;
    let mut stats = RoundStats::default();
    let map_data = load_level(current_level, &mut stats)?;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(map_data)
        .insert_resource(stats)
        .insert_resource(CurrentLevel(current_level))
        .insert_resource(WaitingToLoadTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )))
        .init_resource::<RoundStats>()
        .init_resource::<ExtraText>()
        .add_state::<PlayState>()
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, main_loop.run_if(in_state(PlayState::Playing)))
        .add_systems(
            Update,
            level_finished.run_if(move |current_state: Res<State<PlayState>>| {
                matches!(current_state.get(), PlayState::Lost | PlayState::Won)
            }),
        )
        .add_systems(Update, show_map);

    #[cfg(debug_assertions)] // debug/dev builds only
    {
        // use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
        // app.add_plugins(FrameTimeDiagnosticsPlugin);
        // app.add_plugins(LogDiagnosticsPlugin::default());
    }

    app.run();

    Ok(())
}

fn setup(mut commands: Commands) {
    // 2d camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection::new("", TextStyle::default())],
            alignment: TextAlignment::Center,
            ..Default::default()
        },
        transform: Transform::from_translation(100. * Vec3::Y),
        ..default()
    });
}

fn show_map(
    map_data: Res<MapData>,
    stats: Res<RoundStats>,
    asset_server: Res<AssetServer>,
    mut map_text: Query<&mut Text>,
    extra_text: Res<ExtraText>,
) {
    let mut map_text = map_text.single_mut();
    let font = asset_server.load("../../assets/FiraCodeNerdFont-Regular.ttf");

    map_text.sections = generate_map_str(&map_data, &stats)
        .map(|section| TextSection {
            style: TextStyle {
                font: font.clone(),
                font_size: 42.,
                ..section.style
            },
            ..section
        })
        .chain(extra_text.0.clone())
        .collect();
}

fn main_loop(
    mut map_data: ResMut<MapData>,
    mut stats: ResMut<RoundStats>,
    mut current_level: ResMut<CurrentLevel>,
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<PlayState>>,
    mut load_timer: ResMut<WaitingToLoadTimer>,
    mut extra_text: ResMut<ExtraText>,
) {
    run_main_loop(
        &Term::stdout(),
        &Term::stderr(),
        &mut current_level.0,
        &mut map_data,
        &mut stats,
        &mut state,
        &keyboard_input,
        &mut load_timer,
        &mut extra_text,
    )
    .unwrap();
}

fn run_main_loop(
    term: &Term,
    term_err: &Term,
    current_level: &mut Level,
    map_data: &mut MapData,
    stats: &mut RoundStats,
    state: &mut NextState<PlayState>,
    input: &Input<KeyCode>,
    load_timer: &mut WaitingToLoadTimer,
    extra_text: &mut ExtraText,
) -> Result<()> {
    let result = super::logic::play_level(term, map_data, stats, input);

    let action = match result {
        Err(err) => {
            term_err.write_line(&format!("{}", err))?;
            return Ok(());
        }
        Ok(None) => return Ok(()),
        Ok(Some(action)) => action,
    };

    match action {
        Action::LoadLevel(level) => {
            let Ok(level) = Level::from_str(&level) else {
                println!("{}", Error::LevelNotFound(level));
                return Ok(());
            };

            *map_data = load_level(level, stats)?;
            *current_level = level;
        }
        Action::Result(RoundResult::Won) => {
            state.set(PlayState::Won);
            load_timer.0.reset();

            extra_text.0 = vec![
                TextSection::new(
                    "Level won!",
                    TextStyle {
                        font: default(),
                        font_size: default(),
                        color: Color::GREEN,
                    },
                ),
                TextSection::new(
                    "\nContinuing to next level... (press \"r\" to restart)",
                    default(),
                ),
            ];
        }
        Action::Result(RoundResult::Lost(_reason)) => {
            state.set(PlayState::Lost);
            load_timer.0.reset();

            // TODO: print reason
            extra_text.0 = vec![
                TextSection::new(
                    "You lost!",
                    TextStyle {
                        font: default(),
                        font_size: default(),
                        color: Color::RED,
                    },
                ),
                TextSection::new("\nRestart level...", default()),
            ];
        }
        Action::RestartLevel => {
            *map_data = reload_level(*current_level, stats)?;
        }
        Action::Quit => todo!(),
    };

    Ok(())
}

fn level_finished(
    mut map_data: ResMut<MapData>,
    mut stats: ResMut<RoundStats>,
    mut current_level: ResMut<CurrentLevel>,
    keyboard_input: Res<Input<KeyCode>>,
    state: Res<State<PlayState>>,
    mut next_state: ResMut<NextState<PlayState>>,
    mut load_timer: ResMut<WaitingToLoadTimer>,
    mut extra_text: ResMut<ExtraText>,
    time: Res<Time>,
) {
    load_timer.0.tick(time.delta());

    if !load_timer.0.finished() || keyboard_input.get_just_pressed().next().is_none() {
        return;
    }

    match state.get() {
        PlayState::Lost => {
            *map_data = reload_level(current_level.0, &mut stats).unwrap();
        }
        PlayState::Won => {
            if keyboard_input.just_released(KeyCode::R) {
                *map_data = reload_level(current_level.0, &mut stats).unwrap();
            } else {
                let next_level = current_level.0.get_next_level();
                *map_data = load_level(next_level, &mut stats).unwrap();
                current_level.0 = next_level;
            }
        }
        _ => panic!(),
    }

    next_state.set(PlayState::Playing);
    extra_text.0 = Vec::new();
}

fn reload_level(current_level: Level, stats: &mut RoundStats) -> Result<MapData> {
    let map_data = load_map_data(current_level);
    *stats = RoundStats::default();

    Ok(map_data)
}

fn load_level(level: Level, stats: &mut RoundStats) -> Result<MapData> {
    let map_data = load_map_data(level);
    *stats = RoundStats::default();

    Ok(map_data)
}
