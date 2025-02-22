use bevy::prelude::*;

#[derive(Resource)]
pub struct Mission {
    id: String,
    name: String,
    enemy: String,
    requirment: Vec<String>,
    consequences: Vec<String>,
    rewards: Vec<String>,
    time_left: usize,
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
    MagicDust,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Tech {
    HeavyBody,
    HoverMagic,
}
