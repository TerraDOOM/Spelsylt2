use crate::prelude::*;
use rand::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use std::{f32::consts::PI, time::Duration};
use ResourceType::*;

mod uispawner;

use uispawner::*;

pub fn xcom_plugin(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_systems(OnEnter(GameState::Xcom), on_xcom)
        .add_systems(
            Update,
            (button_system, update_scroll_position).run_if(in_state(GameState::Xcom)),
        )
        .add_systems(
            Update,
            (update).run_if(in_state(GameState::Xcom).and(in_state(Focus::Map))),
        )
        .add_systems(
            Update,
            (unequip_loadout).run_if(in_state(GameState::Xcom).and(in_state(Focus::Mission))),
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

#[derive(Component)]
pub struct ShipComponent(Slot);

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
    pub circle: Handle<Image>,
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
    pub loadout: HashMap<Slot, Option<ResourceType>>,
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

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Slot {
    Front,
    Core1,
    Engine,
    LeftWing1,
    RightWing1,
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

#[derive(Component)]
struct LoadoutIcon;

#[derive(Component)]
pub struct MissionMarker;

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
                if link.0 != ButtonPath::MissionMenu {
                    sprite.image = context.assets.button_normal_hover.clone();
                }
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
                if link.0 != ButtonPath::MissionMenu {
                    sprite.image = context.assets.button_normal_hover.clone();
                }
            }
            Interaction::None => {
                if link.0 != ButtonPath::MissionMenu {
                    sprite.image = context.assets.button_normal.clone();
                }
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
        time: 371520,
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
        loadout: HashMap::from([
            (Slot::Front, Some(Pilot)),
            (Slot::Engine, Some(Engine_T1)),
            (Slot::Core1, None),
            (Slot::LeftWing1, Some(Gun_machinegun)),
            (Slot::RightWing1, Some(Gun_Rocket)),
        ]),
        timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating),
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

fn spawn_mission(commands: &mut Commands, context: &ResMut<XcomState>) -> Mission {
    commands.spawn((
        Button,
        ButtonLink(ButtonPath::MissionMenu),
        MissionMarker,
        Node {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        ImageNode::new(context.assets.circle.clone()),
        ZIndex(1),
    ));
    Mission {
        id: "TODO".to_string(),
        name: "Not implemented yet".to_string(),
        enemy: "Cirno idk".to_string(),
        requirment: vec![],
        consequences: vec![],
        rewards: vec![],
        time_left: 20000,
    }
}

fn move_enemies(mut marker_query: Query<(&mut Node), (With<MissionMarker>)>, passed_time: f32) {
    for (mut node) in &mut marker_query {
        node.left = Val::Px(passed_time.sin() * 400.0 + 400.0);
        node.top = Val::Px(passed_time.cos() * 200.0 + 400.0);
    }

    /*    for mission in active_missions {
        mission.time_left -= passed_time;
    }*/
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
        icons: HashMap::from([
            (Tech::HeavyBody, asset_server.load("Xcom_hud/Flight.png")),
            (
                Tech::MagicBullet,
                asset_server.load("Xcom_hud/Magic_bullet.png"),
            ),
            (
                Tech::MachineGun,
                asset_server.load("Xcom_hud/Machingun.png"),
            ),
            (Tech::Rocket, asset_server.load("Xcom_hud/rocket.png")),
        ]),
        circle: asset_server.load("Enemies/Redcirle.png"),
        font: asset_server.load("fonts/Pixelfont/slkscr.ttf"),
    }
}

fn time_to_date(time: usize) -> String {
    let mut month = "Jan".to_owned();
    let mut day_reduction = 0;
    match (time / (24 * 60)) % 365 {
        32..=59 => {
            month = "Feb".to_owned();
            day_reduction = 31;
        }
        60..=90 => {
            month = "Mar".to_owned();
            day_reduction = 59;
        }
        91..=120 => {
            month = "Apr".to_owned();
            day_reduction = 90;
        }
        121..=151 => {
            month = "May".to_owned();
            day_reduction = 120;
        }
        152..=181 => {
            month = "Jun".to_owned();
            day_reduction = 151;
        }
        182..=212 => {
            month = "Jul".to_owned();
            day_reduction = 181;
        }
        213..=243 => {
            month = "Aug".to_owned();
            day_reduction = 212;
        }
        244..=273 => {
            month = "Sep".to_owned();
            day_reduction = 243;
        }
        274..=304 => {
            month = "Oct".to_owned();
            day_reduction = 273;
        }
        305..=334 => {
            month = "Nov".to_owned();
            day_reduction = 304;
        }
        335..=365 => {
            month = "Dec".to_owned();
            day_reduction = 334;
        }
        _ => {}
    }
    format!(
        "{}\n{} {}\n{:02}:{:02}",
        1985 + (time / (24 * 60 * 365)),
        month,
        (time / (24 * 60)) - day_reduction,
        (time / 60) % 24,
        time % 60
    )
}

fn update(
    mut context: ResMut<XcomState>,
    real_time: Res<Time>,
    clock_query: Single<(&mut Children), With<Clock>>,
    mut text_query: Query<&mut Text>,
    mut marker_query: Query<(&mut Node), (With<MissionMarker>)>,
) {
    context.timer.tick(real_time.delta());

    if context.timer.just_finished() {
        context.timer.reset();

        let scientists: usize = context.resources[&Scientists].amount.clone();
        let engineers: usize = context.resources[&Engineer].amount.clone();
        context.time += 5;

        let mut rng = rand::thread_rng();
        //        if 0 == rng.gen_range(0..5){
        //            spawn_mission(commands: &mut Commands, context: &ResMut<XcomState>);
        // TODO        }

        move_enemies(marker_query, (context.time as f32) / 80.0);

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
        //    if clock_query.is_some() {
        let mut text = text_query.get_mut(clock_query[0]).unwrap();
        **text = time_to_date(context.time);
    }
}

fn on_time_tick(context: ResMut<XcomState>, delta_time: usize) {
    //Chance for invasion/mission TODO
    //Tick research and production

    //Change hudv
}
