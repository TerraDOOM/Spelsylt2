#[derive(Resource)]
pub struct Mission {
    id: String,
    name: String,
    enemy: String,
    requirment: Vec<String>,
    consequences: Vec<String>,
    rewards: Vec<String>,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
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

#[derive(Resource)]
pub struct XcomState {
    time: usize,
    research: Vec<Research>,
    selected_research: Research,
    resources: Vec<Resources>,
    assets: XcomSprites,
}

#[derive(Resource)]
pub struct XcomSprites {
    geoMap: Sprite,
}
