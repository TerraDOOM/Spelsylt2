use crate::prelude::*;
use std::collections::HashMap;
use ResourceType::*;

use crate::uispawner::*;

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

    app.add_systems(OnEnter(Focus::Production), on_prod);
    app.add_systems(OnExit(Focus::Production), off_prod);

    app.add_systems(OnEnter(Focus::Mission), on_mission);
    app.add_systems(OnExit(Focus::Mission), off_mission);
}

#[derive(Component)]
pub struct ScienceScreen;
#[derive(Component)]
pub struct ProdScreen;

#[derive(Component)]
pub struct MissionScreen;

pub fn on_science(mut science_query: Query<&mut Node, With<ScienceScreen>>) {
    println!("On_science");
    for mut science_node in &mut science_query {
        science_node.display = Display::Flex;
    }
}

pub fn off_science(mut science_query: Query<&mut Node, With<ScienceScreen>>) {
    println!("_science");
    for mut science_node in &mut science_query {
        science_node.display = Display::None;
    }
}

pub fn on_prod(mut prod_query: Query<&mut Node, With<ProdScreen>>) {
    println!("On_prod");
    for mut prod_node in &mut prod_query {
        println!("lol");
        prod_node.display = Display::Flex;
    }
}

pub fn off_prod(mut prod_query: Query<&mut Node, With<ProdScreen>>) {
    for mut prod_node in &mut prod_query {
        prod_node.display = Display::None;
    }
}

pub fn on_mission(mut mission_query: Query<&mut Node, With<MissionScreen>>) {
    println!("TODO make mission parameters fit");
    for mut mission_node in &mut mission_query {
        mission_node.display = Display::Flex;
    }
}

pub fn off_mission(mut mission_query: Query<&mut Node, With<MissionScreen>>) {
    for mut mission_node in &mut mission_query {
        mission_node.display = Display::None;
    }
}

#[derive(Component)]
struct Background;

#[derive(Component)]
pub struct XcomObject;

#[derive(Resource)]
pub struct XcomResources {
    pub geo_map: Handle<Image>,
    pub placeholder: Handle<Image>,
    pub button_normal: Handle<Image>,
    pub button_normal_hover: Handle<Image>,
    pub button_normal_big: Handle<Image>,
    pub button_green: Handle<Image>,
    pub button_green_hover: Handle<Image>,
    pub backpanel: Handle<Image>,
    pub icons: HashMap<Tech, Handle<Image>>,
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct XcomState {
    pub time: usize,
    pub research: Vec<Research>,
    pub selected_research: Option<Research>,
    pub resources: HashMap<ResourceType, Resources>,
    pub assets: XcomResources,
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
pub enum Focus {
    #[default]
    Map,
    Science,
    Production,
    Mission,
}

#[derive(Component)]
pub struct ButtonLink(pub ButtonPath);

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
                println!("Pressed a button");

                match (link).0 {
                    ButtonPath::MainMenu => {
                        next_state.set(Focus::Map);
                    }
                    ButtonPath::ScienceMenu => {
                        next_state.set(Focus::Science);
                    }
                    ButtonPath::ProductionMenu => {
                        println!("Entering Production");
                        next_state.set(Focus::Production);
                    }
                    ButtonPath::MissionMenu => {
                        println!("Entering Mission");
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
    interaction_query: Option<Single<(&Interaction), (Changed<Interaction>, With<BackDropFade>)>>,
    mut next_state: ResMut<NextState<Focus>>,
) {
    if let Some(interaction) = interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Awesome");
                next_state.set(Focus::Map);
            }
            Interaction::Hovered => {
                println!("Hover");
            }
            Interaction::None => {
                println!("Chilling");
            }
        }
    } else {
        println!("Fucked up");
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

fn spawn_mission(commands: &mut Commands, context: &ResMut<XcomState>) {
    commands.spawn((
        Button,
        ButtonLink(ButtonPath::MissionMenu),
        Node {
            width: Val::Px(150.0),
            height: Val::Px(150.0),
            border: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(Color::srgb(0.9, 0.6, 0.6)),
        ZIndex(1),
    ));
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

    spawn_mission(&mut commands, &context);

    //Map hud
    SpawnGeoHUD(&mut commands, &context);

    //ScienceHud
    SpawnScienceHUD(&mut commands, &context);

    //ProductionHud
    SpawnManufacturingHUD(&mut commands, &context);

    //SpawnMissionHud
    SpawnMissionHUD(&mut commands, &context);
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
        icons,
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
