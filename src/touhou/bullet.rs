use std::{f32::consts::PI, time::Duration};

use super::*;

pub fn bullet_plugin(app: &mut App) {
    app.add_event::<BulletHit>()
        .add_event::<PlayerHit>()
        .add_event::<EnemyHit>()
        .add_systems(
            FixedUpdate,
            (
                check_enemy_bullets,
                check_bullet_bullet,
                move_bullets,
                despawn_bullets,
                bullet_spawner,
                fire_weapons,
            )
                .run_if(in_state(GameState::Touhou)),
        )
        .add_systems(
            FixedPreUpdate,
            set_alt_fire.run_if(in_state(GameState::Touhou)),
        )
        .add_systems(
            FixedPostUpdate,
            (player_hits, bullet_bullet_hit).run_if(in_state(GameState::Touhou)),
        )
        .add_systems(Startup, (add_dead, make_cannon, make_cannon2));
}

fn add_dead(asset_server: Res<AssetServer>) {
    let _: Handle<Image> = asset_server.load("dead.png");
}

fn make_cannon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Weapon {
        timer: Timer::new(Duration::from_secs_f32(0.05), TimerMode::Repeating),
        ammo_cost: 0,
        bullet: BulletBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            collider: Collider { radius: 6.0 },
            sprite: Sprite {
                image: asset_server.load("bullets/bullet1.png"),
                ..Default::default()
            },
            bullet: Bullet {
                velocity: Vec2::new(-20.0, 0.0),
            },
            ..Default::default()
        },
        salted: true,
    });
}

fn make_cannon2(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Weapon {
            timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating),
            ammo_cost: 0,
            bullet: BulletBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(PI / 2.0)),
                collider: Collider { radius: 100.0 },
                sprite: Sprite {
                    image: asset_server.load("bullets/bullet1.png"),
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..Default::default()
                },
                bullet: Bullet {
                    velocity: Vec2::new(-20.0, 0.0),
                },
                ..Default::default()
            },
            salted: true,
        })
        .insert(AltFire);
}

fn bullet_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut time_since_last: Local<f32>,
    player: PlayerQ<&Transform>,
    asset_server: Res<AssetServer>,
) {
    let start_pos = Vec2::new(0.0, 1000.0);
    let player_pos = player.into_inner().translation.xy();

    if *time_since_last > 1.0 {
        commands.spawn(BulletBundle {
            transform: Transform::from_xyz(0.0, 1000.0, 0.0),
            collider: Collider { radius: 10.0 },
            sprite: Sprite {
                image: asset_server.load("mascot.png"),
                custom_size: Some(Vec2::splat(30.0)),
                ..Default::default()
            },
            bullet: Bullet {
                velocity: (player_pos - start_pos).normalize() * 5.0,
            },
            ..Default::default()
        });
        *time_since_last = 0.0;
    } else {
        *time_since_last += time.delta_secs();
    }
}

#[derive(Component)]
pub struct Weapon {
    timer: Timer,
    ammo_cost: u32,
    bullet: BulletBundle,
    salted: bool,
}

impl Weapon {
    fn spawn_bullet(&mut self, player_pos: Vec2) -> BulletBundle {
        self.timer.reset();
        let new_bullet = BulletBundle {
            transform: Transform {
                translation: player_pos.extend(0.0) + self.bullet.transform.translation,
                ..self.bullet.transform
            },
            ..self.bullet.clone()
        };

        new_bullet
    }
}

fn set_alt_fire(
    mut commands: Commands,
    player: PlayerQ<Entity>,
    mut weapons: Query<&mut Weapon>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut player = commands.entity(*player);

    if input.just_pressed(KeyCode::ShiftLeft) || input.just_released(KeyCode::ShiftLeft) {
        for mut weapon in &mut weapons {
            weapon.timer.reset();
        }
    }
    if input.pressed(KeyCode::ShiftLeft) {
        player.insert(AltFire);
    } else if input.released(KeyCode::ShiftLeft) {
        player.remove::<AltFire>();
    }
}

fn fire_weapons(
    time: Res<Time>,
    mut commands: Commands,
    mut weapons: Query<(&mut Weapon, Option<&AltFire>)>,
    player: PlayerQ<(&Transform, Option<&AltFire>)>,
) {
    let (trans, alt) = player.into_inner();

    let pos = trans.translation.xy();
    let alt_fire = alt.is_some();

    for (mut weapon, is_alt) in &mut weapons {
        // we are in the wrong weapon group
        if is_alt.is_some() != alt_fire {
            continue;
        }

        weapon.timer.tick(time.delta());

        if weapon.timer.just_finished() {
            let bullet = weapon.spawn_bullet(pos);
            commands
                .spawn(bullet)
                .insert(PlayerBullet { damage: 0 })
                .insert_if(Salted, || weapon.salted);
        }
    }
}

#[derive(Bundle, Default, Clone)]
pub struct BulletBundle {
    transform: Transform,
    collider: Collider,
    sprite: Sprite,
    bullet: Bullet,
    markers: TouhouMarker,
}

#[derive(Component, Default)]
pub struct PlayerBullet {
    damage: u32,
}

#[derive(Component, Clone, Default)]
pub struct Bullet {
    velocity: Vec2,
}

fn circle(t: &Transform, c: &Collider) -> Circle {
    Circle {
        pos: t.translation.xy(),
        radius: c.radius,
    }
}

#[derive(Component, Default)]
pub struct Salted;

#[derive(Component, Default)]
pub struct Phasing;

#[derive(Event)]
pub struct PlayerHit(Entity);

#[derive(Component)]
pub struct AltFire;

#[derive(Event)]
struct BulletHit {
    player: Entity,
    enemy: Entity,
}

#[derive(Event)]
struct EnemyHit {
    enemy: Entity,
}

fn player_hits(
    mut commands: Commands,
    mut hits: EventReader<PlayerHit>,
    player: PlayerQ<&mut Sprite>,
    asset_server: Res<AssetServer>,
) {
    let mut sprite = player.into_inner();

    for PlayerHit(ent) in hits.read() {
        sprite.image = asset_server.load("dead.png");
        commands.entity(*ent).despawn();
    }
}

fn bullet_bullet_hit(
    mut commands: Commands,
    mut hits: EventReader<BulletHit>,
    player_bullets: Query<(&Bullet, Option<&Salted>), (With<PlayerBullet>)>,
    enemy_bullets: Query<(&Bullet), Without<PlayerBullet>>,
) {
    for BulletHit { player, enemy } in hits.read() {
        let Ok((p, salted)) = player_bullets.get(*player) else {
            continue;
        };
        let Ok(e) = enemy_bullets.get(*enemy) else {
            continue;
        };

        commands.entity(*player).despawn();
        if salted.is_some() {
            commands.entity(*enemy).despawn();
        }
    }
}

fn check_bullet_bullet(
    mut hits: EventWriter<BulletHit>,
    player_bullets: Query<
        (Entity, &Transform, &Collider, &Bullet),
        (With<PlayerBullet>, Without<Phasing>),
    >,
    enemy_bullets: Query<(Entity, &Transform, &Collider, &Bullet), Without<PlayerBullet>>,
) {
    for (p, p_trans, p_coll, _) in &player_bullets {
        let player_circle = circle(p_trans, p_coll);

        for (e, e_trans, e_coll, _) in &enemy_bullets {
            let enemy_circle = circle(e_trans, e_coll);

            if player_circle.hits(enemy_circle) {
                hits.send(BulletHit {
                    player: p,
                    enemy: e,
                });
            }
        }
    }
}

fn check_enemy_bullets(
    player: PlayerQ<(&Transform, &Collider)>,
    bullet_query: Query<(Entity, &Transform, &Collider), (With<Bullet>, Without<PlayerBullet>)>,
    mut hit_writer: EventWriter<PlayerHit>,
) {
    let player_circle = {
        let (trans, coll) = player.into_inner();
        circle(trans, coll)
    };

    for (ent, trans, coll) in &bullet_query {
        let bullet_circle = circle(trans, coll);

        if bullet_circle.hits(player_circle) {
            hit_writer.send(PlayerHit(ent));
        }
    }
}

fn despawn_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<PlayerMarker>)>,
) {
    for (entity, transform) in &bullet_query {
        if !Rect::new(-4000.0, -4000.0, 4000.0, 4000.0).contains(transform.translation.xy()) {
            commands.entity(entity).despawn()
        }
    }
}

fn move_bullets(mut bullet_query: Query<(&mut Bullet, &mut Transform)>) {
    for (mut bullet, mut transform) in &mut bullet_query {
        transform.translation += bullet.velocity.extend(0.0);
    }
}
