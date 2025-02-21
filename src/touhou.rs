use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

struct TouhouMarker;

pub fn touhou_plugin(app: &mut App) {
    app.add_plugins((RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0)))
        .add_systems(Startup, setup);
}

pub fn setup(mut commands: Commands) {}

#[derive(Bundle)]
pub struct Player {
    controller: KinematicCharacterController,
    sprite: Sprite,
    rb: RigidBody,
    trans: Transform,
    collider: Collider,
    coll_types: ActiveCollisionTypes,
}
