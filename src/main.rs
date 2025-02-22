#![allow(unused_mut)]

use bevy::{
    dev_tools::{self, DevToolsPlugin},
    input::common_conditions::*,
    prelude::*,
    winit::{UpdateMode, WinitSettings},
};
use std::time::Duration;

mod prelude;
mod touhou;
mod types;
mod xcom;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Xcom,
    Touhou,
}

fn toggle_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut options: ResMut<bevy::dev_tools::ui_debug_overlay::UiDebugOptions>,
) {
    if input.just_pressed(KeyCode::Space) {
        // The toggle method will enable the debug_overlay if disabled and disable if enabled
        options.toggle();
    }
}

fn main() {
    App::new()
        .insert_resource(WinitSettings::game())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(dev_tools::ui_debug_overlay::DebugUiPlugin)
        .add_plugins((xcom::xcom_plugin, touhou::touhou_plugin))
        .init_state::<GameState>()
        .add_systems(Startup, global_setup)
        .add_systems(Update, toggle_overlay)
        .add_systems(
            Update,
            (
                enter_xcom.run_if(input_just_pressed(KeyCode::KeyX)),
                enter_touhou.run_if(input_just_pressed(KeyCode::KeyT)),
            )
                .run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), destroy_bg)
        .run();
}

fn enter_xcom(mut next_state: ResMut<NextState<GameState>>, mut winit: ResMut<WinitSettings>) {
    //set_winit_xcom(winit);
    next_state.set(GameState::Xcom)
}

fn enter_touhou(
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut winit: ResMut<WinitSettings>,
    global_camera: Single<Entity, With<GlobalCamera>>,
) {
    set_winit_touhou(winit);
    next_state.set(GameState::Touhou);
    commands.entity(global_camera.into_inner()).despawn();
}

fn set_winit_xcom(mut winit: ResMut<WinitSettings>) {
    winit.focused_mode = UpdateMode::reactive_low_power(Duration::from_secs(1));
    winit.unfocused_mode = UpdateMode::reactive_low_power(Duration::from_secs(1));
}

fn set_winit_touhou(mut winit: ResMut<WinitSettings>) {
    winit.focused_mode = UpdateMode::Continuous;
    winit.unfocused_mode = UpdateMode::Continuous;
}

#[derive(Component)]
struct MenuBG;

#[derive(Component)]
struct GlobalCamera;

fn global_setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((Camera2d::default(), GlobalCamera));

    commands.spawn((
        Sprite {
            image: asset_server.load("menu.png"),
            ..Default::default()
        },
        MenuBG,
    ));
}

fn destroy_bg(mut commands: Commands, bg: Single<Entity, With<MenuBG>>) {
    commands.entity(*bg).despawn();
}
