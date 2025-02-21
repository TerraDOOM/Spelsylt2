use bevy::prelude::*;

#[derive(Resource)]
pub struct Mission {
    id: String,
    name: String,
    enemy: String,
    requirment: Vec<String>,
    consequences: Vec<String>,
    rewards: Vec<String>,
}

pub struct Research {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: usize,
    pub prerequisites: Vec<String>,
    pub progress: usize,
}

#[derive(Resource)]
pub struct Resources {
    name: String,
    description: String,
    amount: usize,
}
