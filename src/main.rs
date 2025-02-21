use bevy::{prelude::*, winit::WinitSettings};

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
        .add_systems(Setup, global_setup)
        .run();
}

pub fn global_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default())
}
