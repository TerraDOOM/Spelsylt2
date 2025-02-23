#![allow(unused_mut, unused_variables, unused_parens, non_camel_case_types)]

use bevy::{
    dev_tools::{self},
    input::common_conditions::*,
    prelude::*,
    winit::WinitSettings,
};

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

fn main() {
    App::new()
        .insert_resource(WinitSettings::game())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((xcom::xcom_plugin, touhou::touhou_plugin))
        .init_state::<GameState>()
        .add_systems(Startup, (global_setup, create_camera))
        .add_systems(PostStartup, enter_xcom)
        .add_systems(OnExit(GameState::Touhou), create_camera)
        .add_systems(OnEnter(GameState::Touhou), destroy_camera)
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
    next_state.set(GameState::Touhou);
}

#[derive(Component)]
struct MenuBG;

#[derive(Component)]
struct GlobalCamera;

fn global_setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
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

fn create_camera(mut commands: Commands) {
    commands.spawn((Camera2d, GlobalCamera));
}

fn destroy_camera(mut commands: Commands, global_camera: Single<Entity, With<GlobalCamera>>) {
    commands.entity(global_camera.into_inner()).despawn();
}
