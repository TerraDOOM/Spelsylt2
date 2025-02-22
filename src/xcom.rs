use crate::prelude::*;
use std::collections::HashMap;
use ResourceType::*;
pub fn xcom_plugin(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_systems(OnEnter(GameState::Xcom), on_xcom)
        .add_systems(
            Update,
            (update, button_system).run_if(in_state(GameState::Xcom)),
        );

    app.init_state::<Focus>();

    app.add_systems(OnEnter(Focus::Science), on_science);
}

#[derive(Component)]
struct ScienceScreen;

pub fn on_science(mut science_query: Query<&mut Node, With<ScienceScreen>>) {
    println!("woo");
    for mut science_node in &mut science_query {
        science_node.display = Display::Flex;
    }
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
    button_normal_big: Handle<Image>,
    button_green: Handle<Image>,
    button_green_hover: Handle<Image>,
    font: Handle<Font>,
}

#[derive(Resource)]
pub struct XcomState {
    time: usize,
    research: Vec<Research>,
    selected_research: Option<Research>,
    resources: HashMap<ResourceType, Resources>,
    assets: XcomResources,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ButtonPath {
    MainMenu,
    ScienceMenu,
    ProductionMenu,
    MissionMenu,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum Focus {
    #[default]
    Map,
    Science,
    Production,
    Mission,
}

#[derive(Component)]
struct ButtonLink(ButtonPath);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode, &ButtonLink, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    context: ResMut<XcomState>,
    mut next_state: ResMut<NextState<Focus>>,
) {
    for (interaction, mut sprite, link, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // Depending on button flag, do something
                //                log::info!(link);
                sprite.image = context.assets.button_normal_hover.clone();

                match (link).0 {
                    ButtonPath::MainMenu => {
                        **text = "Main menu".to_string();
                    }
                    ButtonPath::ScienceMenu => {
                        next_state.set(Focus::Science);
                    }
                    ButtonPath::ProductionMenu => {
                        **text = "Prodcution menu".to_string();
                    }
                    ButtonPath::MissionMenu => {
                        **text = "Mission menu".to_string();
                    }
                }
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
    let assets = load_xcom_assets(&asset_server);
    commands.insert_resource(XcomState {
        time: 0,
        research: vec![],
        selected_research: None,
        resources: vec![Resources {
            name: Scientists,
            description: "A talented researcher of the arcane".to_string(),
            amount: 5,
        }]
        .into_iter()
        .map(|r| (r.name.clone(), r))
        .collect(),
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
                    Node {
                        width: Val::Px(256.0),
                        height: Val::Px(256.0),
                        // horizontally center child text
                        justify_content: JustifyContent::FlexEnd,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ImageNode::new(context.assets.button_normal_big.clone()),
                ))
                .with_child((
                    Text::new("1985\nApr 5th\n10:49"),
                    TextFont {
                        font: context.assets.font.clone(),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.9)),
                ));

            make_button(
                parent,
                "Research".to_string(),
                ButtonPath::ScienceMenu,
                &(*context),
            );
            make_button(
                parent,
                "Production".to_string(),
                ButtonPath::ProductionMenu,
                &*context,
            );
            make_button(parent, "Save".to_string(), ButtonPath::MainMenu, &*context);
            make_button(parent, "Load".to_string(), ButtonPath::MainMenu, &*context);
        });

    //ScienceHud
    commands
        .spawn((
            ScienceScreen,
            Node {
                width: Val::Percent(70.0),
                height: Val::Percent(70.0),
                right: Val::Vw(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                display: Display::None,
                ..default()
            },
            ImageNode::new(context.assets.button_normal_big.clone()),
        ))
        .with_children(|parent| {
            //Top 30% of the screen for found research and icons
            parent
                .spawn(
                    (Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(30.0),
                        top: Val::Vh(0.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    }),
                )
                .with_children(|research_icon| make_icon(research_icon, &(*context)));

            parent
                .spawn(
                    (Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(70.0),
                        bottom: Val::Vh(0.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    }),
                )
                .with_children(|option_box| {
                    //Make the research dynamic? TODO
                    make_button(
                        option_box,
                        "Heavy Frame".to_string(),
                        ButtonPath::ScienceMenu,
                        &(*context),
                    );
                    make_button(
                        option_box,
                        "Hover Magic".to_string(),
                        ButtonPath::ScienceMenu,
                        &(*context),
                    );
                    make_button(
                        option_box,
                        "Ace Frame".to_string(),
                        ButtonPath::ProductionMenu,
                        &*context,
                    );
                    make_button(
                        option_box,
                        "Bomb".to_string(),
                        ButtonPath::ProductionMenu,
                        &*context,
                    );
                });
        });
}

fn make_icon(parent: &mut ChildBuilder, context: &XcomState) {
    parent
        .spawn((
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::new(context.assets.button_green.clone()),
        ))
        .with_child((
            Text::new("ICON"),
            TextFont {
                font: context.assets.font.clone(),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.9)),
        ));
}

fn make_button(parent: &mut ChildBuilder, text: String, link_id: ButtonPath, context: &XcomState) {
    parent
        .spawn((
            Button,
            ButtonLink(link_id),
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
            TextColor(Color::srgb(0.7, 0.7, 0.9)),
        ));
}

fn off_xcom() {}

fn load_xcom_assets(asset_server: &Res<AssetServer>) -> XcomResources {
    XcomResources {
        geo_map: asset_server.load("placeholder_geomap.png"),
        placeholder: asset_server.load("mascot.png"),
        button_normal: asset_server.load("Xcom_hud/Main_button_clicked.png"),
        button_normal_hover: asset_server.load("Xcom_hud/Main_button_unclicked.png"),
        button_normal_big: asset_server.load("Xcom_hud/clock.png"),
        button_green: asset_server.load("Xcom_hud/Icon_button_clicked.png"),
        button_green_hover: asset_server.load("Xcom_hud/Icon_button_unclicked.png"),
        font: asset_server.load("fonts/Pixelfont/slkscr.ttf"),
    }
}

fn update(mut context: ResMut<XcomState>, real_time: Res<Time>) {
    let scientists: usize = context.resources[&Scientists].amount.clone();
    context.time += 1;
    if let Some(selected_research) = &mut context.selected_research {
        selected_research.progress += scientists;
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
