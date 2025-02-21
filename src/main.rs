use bevy::{prelude::*, winit::WinitSettings};

mod touhou;
mod xcom;

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .run();
}
