use std::{f32::consts::TAU, time::Duration};

use bullet::{
    BulletBundle, BulletCommandExt, HomingBullet, NormalBullet, RotatingBullet, StutterBullet,
    WaveBullet,
};

use super::*;

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Touhou), spawn_enemy)
        .add_systems(
            FixedUpdate,
            circular_rotating_emitter.in_set(TouhouSets::Gameplay),
        );
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
    health: Health,
    markers: (EnemyMarker, TouhouMarker, HasEmitters),
}

#[derive(Component, Default)]
pub struct EnemyMarker;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Health(u32);

#[derive(Component, Default)]
struct HasEmitters;

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
    wave: Option<WaveBullet>,
}

fn circular_rotating_emitter(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &Transform,
        &mut Emitter,
        &BulletSpawner,
        &mut CircularAimedEmitter,
    )>,
    player: Single<&Transform, With<PlayerMarker>>,
) {
    let playerpos = player.into_inner();
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
                let player_dir = playerpos.translation.xy() - bullet.transform.translation.xy();

                let mut commands = commands.spawn(bullet);

                if let Some(normal) = spawner.normal {
                    let velocity = dir.rotate(normal.velocity);
                    commands.add_bullet(NormalBullet { velocity });
                }
                if let Some(mut rotating) = spawner.rotation {
                    rotating.origin += trans.translation.xy();
                    commands.add_bullet(rotating);
                }
            }
        }
    }
}

pub fn spawn_enemy(mut commands: Commands, assets: Res<TouhouAssets>) {
    commands
        .spawn(EnemyBundle {
            sprite: Sprite {
                image: assets.redgirl.clone(),
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
                    transform: Transform::from_xyz(-200.0, 0.0, 0.0),
                    emitter: Emitter {
                        timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
                    },
                    bullet_spawner: BulletSpawner {
                        bullet: BulletBundle {
                            collider: Collider { radius: 5.0 },
                            sprite: Sprite {
                                image: assets.bullet1.clone(),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        normal: Some(NormalBullet {
                            velocity: Vec2::new(2.0, 0.0),
                        }),
                        rotation: Some(RotatingBullet {
                            origin: Vec2::ZERO,
                            rotation_speed: TAU / 16.0,
                        }),
                        homing: Some(HomingBullet {
                            rotation_speed: TAU / 8.0,
                            seeking_time: 5.0,
                        }),
                        stutter: Some(StutterBullet {
                            wait_time: 2.0,
                            initial_velocity: Vec2::new(5.0, 0.0),
                            has_started: false,
                        }),
                        wave: Some(WaveBullet {
                            sine_mod: 1.0,
                            true_velocity: Vec2::new(5.0, 0.0),
                        }),
                        ..Default::default()
                    },
                })
                .insert(CircularAimedEmitter {
                    offset: 150.0,
                    count: 20,
                });
            parent
                .spawn(EmitterBundle {
                    transform: Transform::from_xyz(-200.0, 0.0, 0.0),
                    emitter: Emitter {
                        timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
                    },
                    bullet_spawner: BulletSpawner {
                        bullet: BulletBundle {
                            collider: Collider { radius: 5.0 },
                            sprite: Sprite {
                                image: assets.bullet1.clone(),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        normal: Some(NormalBullet {
                            velocity: Vec2::new(2.0, 0.0),
                        }),
                        rotation: Some(RotatingBullet {
                            origin: Vec2::ZERO,
                            rotation_speed: TAU / -16.0,
                        }),
                        homing: Some(HomingBullet {
                            rotation_speed: TAU / 8.0,
                            seeking_time: 5.0,
                        }),
                        stutter: Some(StutterBullet {
                            wait_time: 2.0,
                            initial_velocity: Vec2::new(5.0, 0.0),
                            has_started: false,
                        }),
                        wave: Some(WaveBullet {
                            sine_mod: 1.0,
                            true_velocity: Vec2::new(5.0, 0.0),
                        }),
                        ..Default::default()
                    },
                })
                .insert(CircularAimedEmitter {
                    offset: 150.0,
                    count: 20,
                });
        });
}
