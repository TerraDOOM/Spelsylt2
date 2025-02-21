use crate::prelude::*;
pub fn xcom_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct XcomObject;

#[derive(Resource)]
pub struct XcomSprites {
    geo_map: Handle<Image>,
    placeholder: Handle<Image>,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    let width = window.resolution.width();
    let height = window.resolution.height();

    let assets = load_Xcom_assets(&asset_server);

    let background_size = Some(Vec2::new(width, height));
    let background_position = Vec2::new(0.0, 0.0);
    commands.spawn((
        dbg!(Sprite {
            image: assets.geo_map,
            custom_size: background_size,
            ..Default::default()
        }),
        Transform::from_translation(background_position.extend(-1.0)),
        Background,
        XcomObject,
    ));

    commands.insert_resource(XcomState {
        time: 0,
        research: vec![],
        selected_research: None,
        resources: vec![],
        assets,
    });
}

fn load_Xcom_assets(asset_server: &Res<AssetServer>) -> XcomSprites {
    XcomSprites {
        geo_map: asset_server.load("assets/placeholder_geomap.jpg"),
        placeholder: asset_server.load("assets/placeholder_geomap.jpg"),
    }
}

/*fn on_xcom_sim(
    mut commands: Commands,
    mut tmp: ResMut<NextState<DatingState>>,
    mut did_init: Local<bool>,
    mut context: ResMut<DatingContext>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    let font = asset_server.load("fonts/Pixelfont/slkscr.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };

    //Cursor initialisation
    if let Some(mut background) = background.map(Single::into_inner) {
        background.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
    } else {
        let background_size = Some(Vec2::new(width, height));
        let background_position = Vec2::new(0.0, 0.0);
        commands.spawn((
            dbg!(Sprite {
                image: asset_server.load("Backgrounds/deeper_deeper_base.png"),
                custom_size: background_size,
                ..Default::default()
            }),
            Transform::from_translation(background_position.extend(-1.0)),
            Background,
            DatingObj,
        ));
    }

    let cursor_size = Vec2::new(width / 8.0, width / 8.0);
    let cursor_position = Vec2::new(0.0, 250.0);
    let enc = commands.spawn((
        Sprite::from_color(Color::srgb(0.25, 0.75, 0.25), cursor_size),
        Transform::from_translation(cursor_position.extend(-0.1)),
        Cursor(0),
        Portrait,
        DatingObj,
    ));
}*/
