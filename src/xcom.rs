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
    app.add_systems(OnExit(Focus::Science), off_science);
}

#[derive(Component)]
struct ScienceScreen;

pub fn on_science(mut science_query: Query<&mut Node, With<ScienceScreen>>) {
    for mut science_node in &mut science_query {
        science_node.display = Display::Flex;
    }
}

pub fn off_science(mut science_query: Query<&mut Node, With<ScienceScreen>>) {
    for mut science_node in &mut science_query {
        science_node.display = Display::None;
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
    backpanel: Handle<Image>,
    icons: HashMap<Tech, Handle<Image>>,
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

#[derive(Component)]
struct BackDropFade;

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
                        next_state.set(Focus::Map);
                    }
                    ButtonPath::ScienceMenu => {
                        next_state.set(Focus::Science);
                    }
                    ButtonPath::ProductionMenu => {
                        next_state.set(Focus::Production);
                    }
                    ButtonPath::MissionMenu => {
                        next_state.set(Focus::Mission);
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

fn quit_hud_element_system(
    mut interaction_query: Query<
        //Shoul be single
        (&Interaction),
        (Changed<Interaction>, With<BackDropFade>),
    >,
    mut next_state: ResMut<NextState<Focus>>,
) {
    for (interaction) in &mut interaction_query {
        next_state.set(Focus::Map);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = load_xcom_assets(&asset_server);
    commands.insert_resource(XcomState {
        time: 0,
        research: vec![Research {
            id: Tech::HeavyBody,
            name: "Heavy Body".to_string(),
            description: "A much heavier chassi, allowing the craft to take upwards of 3 hits"
                .to_string(),
            cost: 50,
            prerequisites: vec![],
            progress: 50,
        }],

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
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                right: Val::Vw(0.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ZIndex(2),
        ))
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
                Val::Px(256.0),
                Val::Px(64.0),
            );
            make_button(
                parent,
                "Production".to_string(),
                ButtonPath::ProductionMenu,
                &*context,
                Val::Px(256.0),
                Val::Px(64.0),
            );
            make_button(
                parent,
                "Save".to_string(),
                ButtonPath::MainMenu,
                &*context,
                Val::Px(256.0),
                Val::Px(64.0),
            );
            make_button(
                parent,
                "Load".to_string(),
                ButtonPath::MainMenu,
                &*context,
                Val::Px(256.0),
                Val::Px(64.0),
            );
        });

    //ScienceHud
    commands
        .spawn((
<<<<<<< HEAD
            ScienceScreen, //The fade backdrop. Will also be a button out
            Button,
            BackDropFade,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                right: Val::Vw(0.0),
=======
            ScienceScreen, //Science screen top. Grayed out background
            /*Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
>>>>>>> a6959a9 (morning)
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                display: Display::None,

                ..default()
            },
<<<<<<< HEAD
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            ZIndex(3),
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn((
                    Node {
                        width: Val::Percent(70.0),
=======
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.4)),
        ))
        .with_children */(
            //Backdrop
            Node {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                left: Val::Vh(10.0),
                top: Val::Vh(10.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ImageNode::new(context.assets.backpanel.clone()),
        )
        .with_children(|parent| {
            //Top part
            //Top 30% of the screen for found research and icons
            parent
                .spawn(
                    (Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(30.0),
                        top: Val::Vh(0.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    }),
                )
                .with_children(|research_icon| {
                    //TODO, researched techs
                    make_icon(research_icon, &(*context));
                    make_icon(research_icon, &(*context));
                    make_icon(research_icon, &(*context));
                    make_icon(research_icon, &(*context));
                    make_icon(research_icon, &(*context));
                });

            parent //Currently researchable techs
                .spawn(
                    (Node {
                        width: Val::Percent(50.0),
>>>>>>> a6959a9 (morning)
                        height: Val::Percent(70.0),
                        right: Val::Vw(0.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ImageNode::new(context.assets.backpanel.clone()),
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
                                flex_direction: FlexDirection::Row,
                                ..default()
                            }),
                        )
                        .with_children(|research_icon| {
                            for unlocked_technology in &context.research {
                                let icon = context.assets.icons[&unlocked_technology.id].clone();
                                make_icon(research_icon, icon, &(*context));
                            }
                        });

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
                                Val::Percent(80.0),
                                Val::Percent(20.0),
                            );
                            make_button(
                                option_box,
                                "Hover Magic".to_string(),
                                ButtonPath::ScienceMenu,
                                &(*context),
                                Val::Percent(80.0),
                                Val::Percent(20.0),
                            );
                            make_button(
                                option_box,
                                "Ace Frame".to_string(),
                                ButtonPath::ProductionMenu,
                                &*context,
                                Val::Percent(80.0),
                                Val::Percent(20.0),
                            );
                            make_button(
                                option_box,
                                "Bomb".to_string(),
                                ButtonPath::ProductionMenu,
                                &*context,
                                Val::Percent(80.0),
                                Val::Percent(20.0),
                            );
                        });
                });
        });
}

fn make_icon(parent: &mut ChildBuilder, image_handler: Handle<Image>, context: &XcomState) {
    parent
        .spawn((
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Px(8.0),
                    right: Val::Px(8.0),
                    top: Val::Px(8.0),
                    bottom: Val::Px(8.0),
                },
                ..default()
            },
            ImageNode::new(context.assets.button_green.clone()),
        ))
        .with_child((
            Node {
<<<<<<< HEAD
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(image_handler),
=======
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::new(context.assets.ship1.clone()),
>>>>>>> a6959a9 (morning)
        ));
}

fn make_button(
    parent: &mut ChildBuilder,
    text: String,
    link_id: ButtonPath,
    context: &XcomState,
    width: Val,
    height: Val,
) {
    parent
        .spawn((
            Button,
            ButtonLink(link_id),
            Node {
                width,
                height,
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
    let mut icons = HashMap::new();
    icons.insert(Tech::HeavyBody, asset_server.load("Xcom_hud/Flight.png"));

    XcomResources {
        geo_map: asset_server.load("placeholder_geomap.png"),
        placeholder: asset_server.load("mascot.png"),
        button_normal: asset_server.load("Xcom_hud/Main_button_clicked.png"),
        button_normal_hover: asset_server.load("Xcom_hud/Main_button_unclicked.png"),
        button_normal_big: asset_server.load("Xcom_hud/clock.png"),
        button_green: asset_server.load("Xcom_hud/Icon_button_clicked.png"),
        button_green_hover: asset_server.load("Xcom_hud/Icon_button_unclicked.png"),
        backpanel: asset_server.load("Xcom_hud/Backpanel.png"),
<<<<<<< HEAD
        icons,
=======
>>>>>>> a6959a9 (morning)
        font: asset_server.load("fonts/Pixelfont/slkscr.ttf"),
        ship1: asset_server.load("Xcom_hud/Flight.png"),
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
