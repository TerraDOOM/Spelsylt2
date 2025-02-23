use std::{f32::consts::TAU, time::Duration};

use bevy::time::Stopwatch;
use bullet::{
    BulletBundle, BulletCommandExt, HomingBullet, NormalBullet, RotatingBullet, StutterBullet,
    WaveBullet,
};

use super::*;

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Touhou), spawn_enemy)
        .insert_resource(EncounterTime {
            time: Stopwatch::new(),
        })
        .add_systems(
            FixedUpdate,
            (
                circular_rotating_emitter,
                circular_homing_emitter,
                circular_wave_emitter,
                advance_encounter_time,
                process_spellcards,
            )
                .in_set(TouhouSets::Gameplay),
        );
}

#[derive(Resource)]
struct EncounterTime {
    time: Stopwatch,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    sprite: Sprite,
    animation: AnimatedSprite,
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

#[derive(Component, Default)]
pub struct CircularHomingEmitter {
    offset: f32,
    count: usize,
    idx: usize,
}

#[derive(Component, Default)]
pub struct CircularWaveEmitter {
    offset: f32,
    count: usize,
    rotation: f32,
    rotation_speed: f32,
}

#[derive(Component, Default)]
pub struct AnimatedSprite {
    transition_time: Timer,
    max_index: usize,
    min_index: usize,
    index: usize,
}

pub fn animate_sprites(time: Res<Time>, mut sprites: Query<(&mut Sprite, &mut AnimatedSprite)>) {
    for (mut sprite, mut animation) in &mut sprites {
        let Some(mut atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };

        animation.transition_time.tick(time.delta());
        if animation.transition_time.just_finished() {
            atlas.index = (atlas.index + 1) % (animation.max_index - animation.min_index)
                + animation.min_index
        }
    }
}

#[derive(Component)]
struct Emitter {
    timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
struct Active(bool);

#[derive(Component)]
struct Spellcard {
    emitters: Vec<Entity>,
    start_time: f32,
    end_time: f32,
}

fn advance_encounter_time(
    time: Res<Time>,
    mut enc_time: ResMut<EncounterTime>,
    spell_cards: Query<&Spellcard>,
) {
    enc_time.time.tick(time.delta());
    let current_time = enc_time.time.elapsed_secs();
    if spell_cards.iter().all(|card| card.end_time < current_time) {
        enc_time.time.reset();
    }
}

fn process_spellcards(
    enc_time: Res<EncounterTime>,
    cards: Query<&Spellcard>,
    mut emitters: Query<&mut Active, With<Emitter>>,
) {
    for mut active in &mut emitters {
        **active = false;
    }
    let current_time = enc_time.time.elapsed_secs();

    for card in &cards {
        if card.start_time < current_time && current_time < card.end_time {
            for ent in &card.emitters {
                let Ok(mut active) = emitters.get_mut(*ent) else {
                    continue;
                };
                **active = true;
            }
        }
    }
}

#[derive(Bundle)]
pub struct EmitterBundle {
    emitter: Emitter,
    bullet_spawner: BulletSpawner,
    transform: Transform,
    active: Active,
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

impl BulletSpawner {
    fn new(bullet: BulletBundle) -> Self {
        Self {
            bullet,
            ..Default::default()
        }
    }

    fn normal(self, velocity: Vec2) -> Self {
        Self {
            normal: Some(NormalBullet { velocity }),
            ..self
        }
    }

    fn rotation(self, origin: Vec2, rotation_speed: f32) -> Self {
        Self {
            rotation: Some(RotatingBullet {
                origin,
                rotation_speed,
            }),
            ..self
        }
    }

    fn homing(self, seeking_time: f32, rotation_speed: f32) -> Self {
        Self {
            homing: Some(HomingBullet {
                seeking_time,
                rotation_speed,
            }),
            ..self
        }
    }

    fn stutter(self, wait_time: f32, initial_velocity: Vec2, has_started: bool) -> Self {
        Self {
            stutter: Some(StutterBullet {
                wait_time,
                initial_velocity,
                has_started,
            }),
            ..self
        }
    }

    fn wave(self, sine_mod: f32, true_velocity: Vec2) -> Self {
        Self {
            wave: Some(WaveBullet {
                sine_mod,
                true_velocity,
            }),
            ..self
        }
    }
}

fn circular_rotating_emitter(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &Transform,
        &mut Emitter,
        &BulletSpawner,
        &mut CircularAimedEmitter,
        &Active,
    )>,
    player: Single<&Transform, With<PlayerMarker>>,
) {
    let playerpos = player.into_inner();
    for (trans, mut emitter, spawner, circ, active) in &mut query {
        if !**active {
            continue;
        }

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

fn circular_wave_emitter(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &Transform,
        &mut Emitter,
        &BulletSpawner,
        &mut CircularWaveEmitter,
        &Active,
    )>,
    player: Single<&Transform, With<PlayerMarker>>,
) {
    let playerpos = player.into_inner();
    for (trans, mut emitter, spawner, mut circ, active) in &mut query {
        if !**active {
            continue;
        }

        emitter.timer.tick(time.delta());

        let mut bullet = spawner.bullet.clone();

        bullet.transform.translation += trans.translation;

        if emitter.timer.finished() {
            emitter.timer.reset();

            let ang = TAU / circ.count as f32;
            for i in 0..circ.count {
                let mut bullet = bullet.clone();
                let dir = Vec2::from_angle(ang * i as f32 + circ.rotation);
                bullet.transform.translation += (dir * circ.offset).extend(0.0);
                let player_dir = playerpos.translation.xy() - bullet.transform.translation.xy();

                let mut commands = commands.spawn(bullet);

                if let Some(normal) = spawner.normal {
                    let velocity = dir.rotate(normal.velocity);
                    commands.add_bullet(NormalBullet { velocity });
                }
                if let Some(mut rotating) = spawner.wave {
                    let velocity = dir.rotate(rotating.true_velocity);
                    commands.add_bullet(WaveBullet { sine_mod: rotating.sine_mod, true_velocity: velocity });
                }
            }

            circ.rotation += circ.rotation_speed;
        }
    }
}

fn circular_homing_emitter(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &Transform,
        &mut Emitter,
        &BulletSpawner,
        &mut CircularHomingEmitter,
        &Active,
    )>,
    player: Single<&Transform, With<PlayerMarker>>,
) {
    let playerpos = player.into_inner();
    for (trans, mut emitter, spawner, mut circ, active) in &mut query {
        if !**active {
            continue;
        }

        emitter.timer.tick(time.delta());

        let mut bullet = spawner.bullet.clone();

        bullet.transform.translation += trans.translation;

        if emitter.timer.finished() {
            emitter.timer.reset();

            let ang = TAU / circ.count as f32;
                let mut bullet = bullet.clone();
                let dir = Vec2::from_angle(ang * circ.idx as f32);
                bullet.transform.translation += (dir * circ.offset).extend(0.0);
                let player_dir = playerpos.translation.xy() - bullet.transform.translation.xy();

                let mut commands = commands.spawn(bullet);

                if let Some(normal) = spawner.normal {
                    let velocity = dir.rotate(normal.velocity);
                    commands.add_bullet(NormalBullet { velocity });
                }
                if let Some(mut rotating) = spawner.homing {
                    commands.add_bullet(rotating);
                }

            circ.idx += 1;
            if circ.idx == 4 { circ.idx = 0; }
        }
    }
}

pub fn spawn_enemy(mut commands: Commands, 
                assets: Res<TouhouAssets>
                ) {
    
    let (mut em1, mut em2, mut em3) = (vec!(), vec!(), vec!());
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
            em1.push(parent
                .spawn(EmitterBundle {
                    transform: Transform::from_xyz(-200.0, 0.0, 0.0),
                    emitter: Emitter {
                        timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
                    },
                    bullet_spawner: BulletSpawner::new(BulletBundle {
                        collider: Collider { radius: 5.0 },
                        sprite: Sprite {
                            image: assets.bullet1.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .normal(Vec2::new(2.0, 0.0))
                    .rotation(Vec2::ZERO, TAU / 16.0),
                    active: Active(false),
                })
                .insert(CircularAimedEmitter {
                    offset: 150.0,
                    count: 20,
                }).id());
            em1.push(parent
                .spawn(EmitterBundle {
                    transform: Transform::from_xyz(-200.0, 0.0, 0.0),
                    emitter: Emitter {
                        timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
                    },
                    bullet_spawner: BulletSpawner::new(BulletBundle {
                        collider: Collider { radius: 5.0 },
                        sprite: Sprite {
                            image: assets.bullet1.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .normal(Vec2::new(2.0, 0.0))
                    .rotation(Vec2::ZERO, TAU / -16.0),
                    active: Active(false),
                })
                .insert(CircularAimedEmitter {
                    offset: 150.0,
                    count: 20,
                }).id());
            em1.push(parent
                .spawn(EmitterBundle {
                    transform: Transform::from_xyz(-200.0, 0.0, 0.0),
                    emitter: Emitter {
                        timer: Timer::new(Duration::from_secs_f32(0.25), TimerMode::Repeating),
                    },
                    bullet_spawner: BulletSpawner::new(BulletBundle {
                        collider: Collider { radius: 10.0 },
                        sprite: Sprite {
                            image: assets.bullet1.clone(),
                            custom_size: Some(Vec2::from((20.0, 20.0))),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .normal(Vec2::new(2.0, 0.0))
                    .homing(4.0, TAU / 8.0),
                    active: Active(false),
                })
                .insert(CircularHomingEmitter {
                    offset: 150.0,
                    count: 4,
                    idx: 0,
                }).id());
            em2.push(parent
                .spawn(EmitterBundle {
                    transform: Transform::from_xyz(-200.0, 0.0, 0.0),
                    emitter: Emitter {
                        timer: Timer::new(Duration::from_secs_f32(1.5), TimerMode::Repeating),
                    },
                    bullet_spawner: BulletSpawner::new(BulletBundle {
                        collider: Collider { radius: 5.0 },
                        sprite: Sprite {
                            image: assets.bullet1.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .normal(Vec2::new(2.0, 0.0))
                    .rotation(Vec2::ZERO, TAU / 16.0),
                    active: Active(false),
                })
                .insert(CircularAimedEmitter {
                    offset: 0.0,
                    count: 48,
                }).id());
            em2.push(parent
                .spawn(EmitterBundle {
                    transform: Transform::from_xyz(-200.0, 0.0, 0.0),
                    emitter: Emitter {
                        timer: Timer::new(Duration::from_secs_f32(1.5), TimerMode::Repeating),
                    },
                    bullet_spawner: BulletSpawner::new(BulletBundle {
                        collider: Collider { radius: 5.0 },
                        sprite: Sprite {
                            image: assets.bullet1.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .normal(Vec2::new(2.0, 0.0))
                    .rotation(Vec2::ZERO, TAU / -16.0),
                    active: Active(false),
                })
                .insert(CircularAimedEmitter {
                    offset: 0.0,
                    count: 48,
                }).id());
            em3.push(parent
                .spawn(EmitterBundle {
                    transform: Transform::from_xyz(-200.0, 0.0, 0.0),
                    emitter: Emitter {
                        timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
                    },
                    bullet_spawner: BulletSpawner::new(BulletBundle {
                        collider: Collider { radius: 5.0 },
                        sprite: Sprite {
                            image: assets.bullet1.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .normal(Vec2::new(2.0, 0.0))
                    .wave(1.0, Vec2::new(1.0, 0.0)),
                    active: Active(false),
                })
                .insert(CircularWaveEmitter {
                    offset: 150.0,
                    count: 6,
                    rotation: 0.0,
                    rotation_speed: 0.1,
                }).id());
        });
        commands.spawn(Spellcard {
            emitters: em3,
            start_time: 0.0,
            end_time: 25.0,
        });
        commands.spawn(Spellcard {
            emitters: em2,
            start_time: 25.0,
            end_time: 45.0,
        });
        commands.spawn(Spellcard {
            emitters: em1,
            start_time: 45.0,
            end_time: 60.0,
        });
}
