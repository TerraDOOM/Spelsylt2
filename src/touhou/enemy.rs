use std::{f32::consts::TAU, time::Duration};

use bevy::ecs::schedule::common_conditions;
use bullet::{BulletBundle, BulletCommandExt, BulletType, NormalBullet, RotatingBullet};

use super::*;

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Touhou), spawn_enemy)
        .add_systems(
            FixedUpdate,
            process_enemy_emitters::<CircularAimedEmitter>.in_set(TouhouSets::Gameplay),
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
    timer: Timer,
    offset: f32,
    normal: Option<NormalBullet>,
    rotation: Option<RotatingBullet>,
    bullet: BulletBundle,
    count: usize,
}

#[derive(Component)]
struct Emitter {}

#[derive(Bundle)]
pub struct EmitterBundle {
    emitter: Emitter,
    timer: Timer,
    bullet_spawner: BulletSpawner,
    transform: Transform,
}

#[derive(Component)]
struct BulletSpawner {
    bullet_bundle: BulletBundle,

}

trait BulletEmitter: Component {
    fn emit(&mut self, time: Time, commands: &mut Commands, pos: Vec3);
}

fn circular_aimed_emitter(&mut self, time: Time, commands: &mut Commands, pos: Vec3) {
    self.timer.tick(time.delta());

    let mut bullet = self.bullet.clone();
    bullet.transform.translation += pos;

    if self.timer.finished() {
        self.timer.reset();
        let ang = TAU / self.count as f32;
        for i in 0..self.count {
            let mut bullet = bullet.clone();
            let dir = Vec2::from_angle(ang * i as f32);
            bullet.transform.translation += (dir * self.offset).extend(0.0);
            let mut commands = commands.spawn(bullet);

            if let Some(normal) = self.normal {
                let velocity = dir.rotate(normal.velocity);
                commands.add_bullet(NormalBullet { velocity });
            }
            if let Some(rotation) = self.rotation {
                let origin = rotation.origin + pos.xy();
                commands.add_bullet(RotatingBullet { origin, ..rotation });
            }
        }
    }
}

fn process_enemy_emitters<E: BulletEmitter>(
    time: Res<Time>,
    mut commands: Commands,
    ents: Query<(&Transform, &Children), With<HasEmitters>>,
    mut emitters: Query<(&Transform, &mut E)>,
) {
    for (ent_trans, children) in &ents {
        for child in children {
            let Ok((trans, mut emitter)) = emitters.get_mut(*child) else {
                continue;
            };

            let emitter_pos = ent_trans.translation + trans.translation;

            emitter.emit(*time, &mut commands, emitter_pos);
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
            parent.spawn(EmitterBundle {
                transform: Transform::IDENTITY,
                emitter: CircularAimedEmitter {
                    timer: Timer::new(Duration::from_secs_f32(3.0), TimerMode::Repeating),
                    normal: Some(NormalBullet {
                        velocity: Vec2::new(5.0, 0.0),
                    }),
                    rotation: Some(RotatingBullet {
                        origin: Vec2::ZERO,
                        rotation_speed: TAU / 8.0,
                    }),
                    offset: 150.0,
                    bullet: BulletBundle {
                        collider: Collider { radius: 50.0 },
                        sprite: Sprite {
                            image: asset_server.load("bullets/bullet1.png"),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    count: 36,
                },
            });
        });
}
