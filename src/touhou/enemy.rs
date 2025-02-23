use std::{f32::consts::TAU, time::Duration};

use bevy::ecs::schedule::common_conditions;
use bullet::{
    BulletBundle, BulletCommandExt, BulletType, HomingBullet, NormalBullet, RotatingBullet,
    StutterBullet,
};

use super::*;

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Touhou), spawn_enemy)
        .add_systems(
            FixedUpdate,
            circular_aimed_emitter.in_set(TouhouSets::Gameplay),
        );
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
    health: Health,
    markers: (TouhouMarker, HasEmitters),
}

#[derive(Component, Default)]
struct Health(usize);

#[derive(Component, Default)]
struct HasEmitters(usize);

#[derive(Component, Default)]
pub struct CircularAimedEmitter {
    offset: f32,
    count: usize,
}

#[derive(Component)]
struct Emitter {
    timer: Timer,
}

#[derive(Bundle)]
pub struct EmitterBundle {
    emitter: Emitter,
    bullet_spawner: BulletSpawner,
    transform: Transform,
}

#[derive(Component, Clone, Default)]
struct BulletSpawner {
    bullet: BulletBundle,
    normal: Option<NormalBullet>,
    rotation: Option<RotatingBullet>,
    stutter: Option<StutterBullet>,
    homing: Option<HomingBullet>,
}

fn circular_aimed_emitter(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &Transform,
        &mut Emitter,
        &BulletSpawner,
        &mut CircularAimedEmitter,
    )>,
) {
    for (trans, mut emitter, spawner, circ) in &mut query {
        emitter.timer.tick(time.delta());

        let mut bullet = spawner.bullet.clone();

        bullet.transform.translation += trans.translation;

        if emitter.timer.finished() {
            emitter.timer.reset();

            let ang = TAU / circ.count as f32;
            for i in 0..circ.count {
                let mut bullet = bullet.clone();
                let dir = Vec2::from_angle(ang * i as f32);
                bullet.transform.translation += (dir * circ.offset).extend(0.0);
                let mut commands = commands.spawn(bullet);

                if let Some(normal) = spawner.normal {
                    let velocity = dir.rotate(normal.velocity);
                    commands.add_bullet(NormalBullet { velocity });
                }
                if let Some(rotation) = spawner.rotation {
                    let origin = rotation.origin + trans.translation.xy();
                    commands.add_bullet(RotatingBullet { origin, ..rotation });
                }
            }
        }
    }
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(EnemyBundle {
            sprite: Sprite {
                image: asset_server.load("mascot.png"),
                custom_size: Some(Vec2::splat(100.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-200.0, 0.0, 0.0),
            collider: Collider { radius: 50.0 },
            health: Health(500),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(EmitterBundle {
                    transform: Transform::IDENTITY,
                    emitter: Emitter {
                        timer: Timer::new(Duration::from_secs_f32(3.0), TimerMode::Repeating),
                    },
                    bullet_spawner: BulletSpawner {
                        bullet: BulletBundle {
                            collider: Collider { radius: 50.0 },
                            sprite: Sprite {
                                image: asset_server.load("bullets/bullet1.png"),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                })
                .insert(CircularAimedEmitter {
                    offset: 150.0,
                    count: 36,
                });
        });
}
