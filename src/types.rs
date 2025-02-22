use bevy::prelude::*;

#[derive(Resource)]
pub struct Mission {
    pub id: String,
    pub name: String,
    pub enemy: String,
    pub requirment: Vec<String>,
    pub consequences: Vec<String>,
    pub rewards: Vec<String>,
    pub time_left: usize,
}

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
pub enum ResourceType {
    Scientists,
    Engineer,
    MagicDust,
    Engine_T1,
    Gun_machinegun,
    Gun_Rocket,
    Pilot,
    Plane_T1,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Tech {
    HeavyBody,
    HoverMagic,
    MagicBullet,
    MachineGun,
    Rocket,
}
