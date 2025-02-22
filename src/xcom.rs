use crate::prelude::*;
use std::collections::HashMap;
use std::{f32::consts::PI, time::Duration};
use ResourceType::*;

mod uispawner;

use uispawner::*;

pub fn xcom_plugin(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_systems(OnEnter(GameState::Xcom), on_xcom)
        .add_systems(Update, (button_system).run_if(in_state(GameState::Xcom)))
        .add_systems(
            Update,
            (update).run_if(in_state(GameState::Xcom).and(in_state(Focus::Map))),
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

#[derive(Component)]
pub struct Clock;

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
    pub loadout: Handle<Image>,
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct XcomState {
    pub time: usize,
    pub research: Vec<Research>,
    pub selected_research: Option<Research>,
    pub selected_production: Option<ResourceType>,
    pub resources: HashMap<ResourceType, Resources>,
    pub assets: XcomResources,
    pub active_missions: Vec<Mission>,
    pub timer: Timer,
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
        (&Interaction, &mut ImageNode, &ButtonLink),
        (Changed<Interaction>, With<Button>),
    >,
    context: ResMut<XcomState>,
    mut next_state: ResMut<NextState<Focus>>,
) {
    for (interaction, mut sprite, link) in &mut interaction_query {
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
        active_missions: vec![],
        selected_research: None,
        selected_production: None,
        timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating),
        resources: vec![
            Resources {
                name: Scientists,
                description: "A talented researcher of the near arcane".to_string(),
                amount: 5,
            },
            Resources {
                name: Engineer,
                description: "A talented craftsman of the near arcane".to_string(),
                amount: 5,
            },
        ]
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
        ImageNode::new(context.assets.button_normal.clone()),
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
    spawn_geo_hud(&mut commands, &context);

    //ScienceHud
    spawn_science_hud(&mut commands, &context);

    //ProductionHud
    spawn_manufacturing_hud(&mut commands, &context);

    //SpawnMissionHud
    spawn_mission_hud(&mut commands, &context);
}

fn off_xcom() {}

fn load_xcom_assets(asset_server: &Res<AssetServer>) -> XcomResources {
    let mut icons = HashMap::new();
    icons.insert(Tech::HeavyBody, asset_server.load("Xcom_hud/Flight.png"));

    XcomResources {
        geo_map: asset_server.load("Xcom_hud/Earth.png"),
        placeholder: asset_server.load("mascot.png"),
        button_normal: asset_server.load("Xcom_hud/Main_button_clicked.png"),
        button_normal_hover: asset_server.load("Xcom_hud/Main_button_unclicked.png"),
        button_normal_big: asset_server.load("Xcom_hud/clock.png"),
        button_green: asset_server.load("Xcom_hud/Icon_button_clicked.png"),
        button_green_hover: asset_server.load("Xcom_hud/Icon_button_unclicked.png"),
        backpanel: asset_server.load("Xcom_hud/Backpanel.png"),
        loadout: asset_server.load("Xcom_hud/Ship_loadment.png"),
        icons,
        font: asset_server.load("fonts/Pixelfont/slkscr.ttf"),
    }
}

fn time_to_date(time: usize) -> String {
    format!("1985\nSep 15\n{}:{}", time / 60, time % 60)
}

fn update(mut context: ResMut<XcomState>, real_time: Res<Time>) {
    context.timer.tick(real_time.delta());
    let scientists: usize = context.resources[&Scientists].amount.clone();
    let engineers: usize = context.resources[&Engineer].amount.clone();
    context.time += 1;
    if let Some(selected_research) = &mut context.selected_research {
        selected_research.progress += scientists;
        if (selected_research.progress > selected_research.cost) {
            //TODO popup/Notification?
        }
    }
    if let Some(selected_production) = &mut context.selected_production {
        println!("lol");
        //        selected_production.progress += scientists;
        //        if (selected_production.progress > selected_production.cost) {
        //TODO popup/Notification?
    }
    //dbg!(time_to_date(context.time));
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
