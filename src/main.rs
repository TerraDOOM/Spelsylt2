use bevy::{prelude::*, winit::WinitSettings};

mod touhou;
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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .run();
}
