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
    id: String,
    name: String,
    description: String,
    cost: usize,
    prerequisites: Vec<String>,
}

#[derive(Resource)]
pub struct Resources {
    name: String,
    description: String,
    amount: usize,
}
