use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::xcom;

#[derive(Component, Default)]
struct TouhouMarker;
#[derive(Component, Default)]
struct PlayerMarker;

pub fn touhou_plugin(app: &mut App) {
    app.add_plugins((RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0)))
        .add_systems(Startup, setup);
}

pub fn setup(mut commands: Commands) {
    xcom::setup;
}

#[derive(Bundle, Default)]
pub struct Player {
    controller: KinematicCharacterController,
    sprite: Sprite,
    rb: RigidBody,
    trans: Transform,
    collider: Collider,
    coll_types: ActiveCollisionTypes,
    markers: (PlayerMarker, TouhouMarker)
}

pub fn spawn_player(mut commands: Commands, mut rapier_config: Query<&mut RapierConfiguration>, asset_server: ResMut<AssetServer>) {
    let mut rapier_config = rapier_config.single_mut();
    rapier_config.gravity = Vec2::ZERO;

    commands.spawn(Player {
        sprite: Sprite {
            custom_size: Some(Vec2::new(10.0, 10.0)),
            image: asset_server.load("player.png"),
            ..Default::default()
        }
        trans: Transform::from_xyz(0, 0, 0),
        rb: RigidBody::KinematicPositionBased,
        coll_types: ActiveCollisionTypes::KINEMATIC_STATIC,
    })
}
