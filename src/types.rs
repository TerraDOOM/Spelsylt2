use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Mission {
    pub id: String,
    pub name: String,
    pub enemy: Enemies,
    pub requirment: Vec<String>,
    pub consequences: Vec<String>,
    pub status: MissionStatus,
    pub rewards: Vec<String>,
    pub time_left: isize,
    pub overworld_x: f32,
    pub overworld_y: f32,
    pub phase: f32,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Research {
    pub id: Tech,
    pub name: String,
    pub description: String,
    pub cost: usize,
    pub prerequisites: Vec<String>,
    pub progress: usize,
}

#[derive(Resource)]
pub struct Resources {
    pub name: ResourceType,
    pub description: String,
    pub amount: usize,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MissionStatus {
    Pending,
    Lost,
    Won,
    Abandonend,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Enemies {
    RedGirl,
    Lizard,
    Tentacle,
    MoonGirl,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ResourceType {
    Scientists,
    Engineer,
    MagicDust,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Tech {
    HeavyBody,
    HoverMagic,
    MagicBullet,
    MachineGun,
    Rocket,
    FinalMission,
}

#[derive(Resource)]
pub struct MissionParams {
    pub loadout: Vec<String>,
    pub enemy: String,
    //    pub mission: Mission,
}
