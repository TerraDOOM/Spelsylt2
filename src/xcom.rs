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
            (unequip_loadout, equip_loadout)
                .run_if(in_state(GameState::Xcom).and(in_state(Focus::Mission))),
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

#[derive(Component)]
pub struct CurrentResearch;

#[derive(Component, Clone)]
pub struct Equipment(pub Tech);

pub fn on_science(
    mut science_query: Query<&mut Node, With<ScienceScreen>>,
    mut current_research_text: Query<&mut Text, With<CurrentResearch>>,
    context: ResMut<XcomState>,
) {
    for mut science_node in &mut science_query {
        science_node.display = Display::Flex;
    }

    for mut text in &mut current_research_text {
        if let Some(selected_research) = &context.selected_research {
            **text = format![
                "Currently researching: \n{}",
                selected_research.name.clone()
            ];
        } else {
            **text = "Not researching anything".to_string();
        }
    }
}

pub fn off_science(mut science_query: Query<&mut Node, With<ScienceScreen>>) {
    for mut science_node in &mut science_query {
        science_node.display = Display::None;
    }
}

pub fn on_prod(mut prod_query: Query<&mut Node, With<ProdScreen>>) {
    for mut prod_node in &mut prod_query {
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
    pub finished_research: Vec<Research>,
    pub possible_research: Vec<Research>,
    pub selected_research: Option<Research>,
    pub selected_production: Option<ResourceType>,
    pub inventory: HashMap<ResourceType, Resources>,
    pub assets: XcomResources,
    pub active_missions: Vec<Mission>,
    pub loadout: HashMap<Slot, Option<Tech>>,
    pub timer: Timer,
    pub speed: usize,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ButtonPath {
    MainMenu,
    ScienceMenu,
    ProductionMenu,
    MissionMenu,
    StartMission,
    StartResearch,
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

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ScienceSelect(pub Tech);

#[derive(Component)]
pub struct MissionMarker(Mission);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut ImageNode,
            &ButtonLink,
            Option<&ScienceSelect>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut context: ResMut<XcomState>,
    mut mission_params: ResMut<MissionParams>,
    mut next_state: ResMut<NextState<Focus>>,
    mut next_scene: ResMut<NextState<GameState>>,
) {
    for (interaction, mut sprite, link, potential_tech) in &mut interaction_query {
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
                        next_state.set(Focus::Production);
                    }
                    ButtonPath::MissionMenu => {
                        next_state.set(Focus::Mission);
                    }

                    ButtonPath::StartMission => {
                        println!("Starting a Mission!");
                        *mission_params = MissionParams {
                            loadout: vec![],
                            enemy: "First enemy".to_string(),
                        };
                        next_scene.set(GameState::Touhou);
                    }

                    ButtonPath::StartResearch => {
                        println!("Changing research TODO");
                        if let Some(tech) = potential_tech {
                            if let Some(i) = context
                                .possible_research
                                .iter()
                                .position(|n| n.id == tech.0)
                            {
                                context.selected_research =
                                    Some(context.possible_research[i].clone());
                            }
                        }
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
    commands.insert_resource(MissionParams {
        loadout: vec![],
        enemy: "".to_string(),
    });

    commands.insert_resource(XcomState {
        time: 371520,
        finished_research: vec![Research {
            id: Tech::HeavyBody,
            name: "Normal jetplane".to_string(),
            description: "A normal fucking plane, kinda shit ngl".to_string(),
            cost: 50,
            prerequisites: vec![],
            progress: 50,
        }],
        possible_research: vec![Research {
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
            (Slot::Front, Some(Tech::Rocket)),
            (Slot::Engine, Some(Tech::MachineGun)),
            (Slot::Core1, None),
            (Slot::LeftWing1, Some(Tech::MagicBullet)),
            (Slot::RightWing1, None),
        ]),
        timer: Timer::new(Duration::from_secs_f32(0.8), TimerMode::Repeating),
        speed: 5,
        inventory: vec![
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

fn spawn_mission(
    commands: &mut Commands,
    context: &ResMut<XcomState>,
    x: f32,
    y: f32,
    phase: f32,
) -> Mission {
    let mission = Mission {
        id: "TODO".to_string(),
        name: "Not implemented yet".to_string(),
        enemy: "Cirno idk".to_string(),
        requirment: vec![],
        consequences: vec![],
        rewards: vec![],
        time_left: 20000 * 40,
        overworld_x: x,
        overworld_y: y,
        phase,
    };
    commands.spawn((
        Button,
        ButtonLink(ButtonPath::MissionMenu),
        MissionMarker(mission.clone()),
        Node {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(5.0)),
            left: Val::Px(x),
            bottom: Val::Px(y),
            ..default()
        },
        ImageNode::new(context.assets.circle.clone()),
        ZIndex(1),
    ));
    mission
}

fn move_enemies(
    mut marker_query: Query<(&mut Node, &mut MissionMarker), (With<MissionMarker>)>,
    passed_time: f32,
) {
    for (mut node, mut mission_marker) in &mut marker_query {
        let phase = mission_marker.0.phase;
        mission_marker.0.overworld_x += (passed_time + phase).sin() * 5.;
        mission_marker.0.overworld_y += (passed_time + phase).cos() * 5.;
        mission_marker.0.time_left -= passed_time as usize;
        dbg!(mission_marker.0.time_left);

        node.left = Val::Px(mission_marker.0.overworld_x);
        node.top = Val::Px(mission_marker.0.overworld_y);
    }
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

    spawn_mission(&mut commands, &context, 100., 100., 0.);

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
    mut commands: Commands,
    mut context: ResMut<XcomState>,
    real_time: Res<Time>,
    clock_query: Single<(&mut Children), With<Clock>>,
    mut text_query: Query<&mut Text>,
    mut marker_query: Query<(&mut Node, &mut MissionMarker), (With<MissionMarker>)>,
) {
    context.timer.tick(real_time.delta());

    if context.timer.just_finished() {
        context.timer.reset();

        let scientists: usize = context.inventory[&Scientists].amount.clone();
        let engineers: usize = context.inventory[&Engineer].amount.clone();
        context.time += 30;

        let mut rng = rand::thread_rng();
        if 0 == rng.gen_range(0..20) {
            spawn_mission(
                &mut commands,
                &context,
                rng.gen_range(120..800) as f32, //The x spawn range
                rng.gen_range(120..500) as f32, //The y spawn range
                rng.gen_range(0..360) as f32,   //The complete phase randomisation
            );
        }

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
