use bevy::{input::common_conditions::*, prelude::*, winit::WinitSettings};

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
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(xcom::xcom_plugin)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_state(GameState)
        .add_systems(Startup, global_setup)
        .add_systems(
            Update,
            (
                enter_xcom.run_if(input_just_pressed(KeyCode::KeyX)),
                enter_touhou
                    .run_if(input_just_pressed(KeyCode::KeyT))
                    .run_if(in_state(GameState::Menu)),
            ),
        )
        .add_systems(OnExit(GameState::Menu), destroy_bg)
        .run();
}

fn enter_xcom(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Xcom)
}

fn enter_touhou(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Touhou)
}

#[derive(Component)]
struct MenuBG;

fn global_setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

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
