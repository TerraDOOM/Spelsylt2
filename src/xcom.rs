use crate::prelude::*;
pub fn xcom_plugin(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_systems(OnEnter(GameState::Xcom), on_xcom)
        .add_systems(
            Update,
            (update, button_system).run_if(in_state(GameState::Xcom)),
        );
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct XcomObject;

#[derive(Resource)]
pub struct XcomResources {
    geo_map: Handle<Image>,
    placeholder: Handle<Image>,
    button_normal: Handle<Image>,
    button_normal_hover: Handle<Image>,
    font: Handle<Font>,
}

#[derive(Resource)]
pub struct XcomState {
    time: usize,
    research: Vec<Research>,
    selected_research: Option<Research>,
    resources: Vec<Resources>,
    assets: XcomResources,
}

#[derive(Component)]
struct ButtonLink(String);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    context: ResMut<XcomState>,
) {
    for (interaction, mut sprite, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // Depending on button flag, do something
                sprite.image = context.assets.button_normal_hover.clone();
            }
            Interaction::Hovered => {
                sprite.image = context.assets.button_normal_hover.clone();
            }
            Interaction::None => {
                sprite.image = context.assets.button_normal.clone();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = load_Xcom_assets(&asset_server);
    commands.insert_resource(XcomState {
        time: 0,
        research: vec![],
        selected_research: None,
        resources: vec![Resources {
            name: "Scientists".to_string(),
            description: "A talented researcher of the arcane".to_string(),
            amount: 5,
        }],
        assets,
    });
}

fn on_xcom(
    mut commands: Commands,
    context: ResMut<XcomState>,
    window: Single<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    let width = window.resolution.width();
    let height = window.resolution.height();

    let background_size = Some(Vec2::new(width, height));
    let background_position = Vec2::new(0.0, 0.0);

    commands.spawn((
        Sprite {
            image: context.assets.geo_map.clone(),
            custom_size: background_size,
            ..Default::default()
        },
        Transform::from_translation(background_position.extend(-1.0)),
        Background,
        XcomObject,
    ));
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            right: Val::Vw(0.0),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexEnd,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent //The clock button
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(256.0),
                        height: Val::Px(256.0),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ImageNode::new(context.assets.button_normal.clone()),
                ))
                .with_child((
                    Text::new("1985\n Apr 5th \n 10:49"),
                    TextFont {
                        font: context.assets.font.clone(),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            make_button(
                parent,
                "Research".to_string(),
                "TODO".to_string(),
                &(*context),
            );
            make_button(
                parent,
                "Production".to_string(),
                "TODO".to_string(),
                &*context,
            );
            make_button(parent, "Save".to_string(), "TODO".to_string(), &*context);
            make_button(parent, "Load".to_string(), "TODO".to_string(), &*context);
        });
}

fn make_button(parent: &mut ChildBuilder, text: String, link_id: String, context: &XcomState) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(256.0),
                height: Val::Px(64.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::new(context.assets.button_normal.clone()),
        ))
        .with_child((
            Text::new(text),
            TextFont {
                font: context.assets.font.clone(),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
}

fn off_xcom() {}

fn load_Xcom_assets(asset_server: &Res<AssetServer>) -> XcomResources {
    XcomResources {
        geo_map: asset_server.load("placeholder_geomap.png"),
        placeholder: asset_server.load("mascot.png"),
        button_normal: asset_server.load("Xcom_hud/Main_button_clicked.png"),
        button_normal_hover: asset_server.load("Xcom_hud/Main_button_unclicked.png"),
        font: asset_server.load("fonts/Pixelfont/slkscr.ttf"),
    }
}

fn update(mut context: ResMut<XcomState>, real_time: Res<Time>) {
    context.time += 1;
    if let Some(selected_research) = &mut context.selected_research {
        selected_research.progress += 1;
    }
}

fn on_time_tick(context: ResMut<XcomState>, delta_time: usize) {
    //Chance for invasion/mission TODO
    //Tick research and production

    //Change hud
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
