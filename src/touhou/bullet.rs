use std::{f32::consts::PI, time::Duration};

use bevy::{
    color::palettes::css::{BLUE, RED},
    ecs::query::QueryFilter,
    time::Stopwatch,
};

use super::*;

#[derive(QueryFilter)]
struct PlayerBullets {
    marker: With<BulletMarker>,
    cond: With<PlayerBullet>,
}
#[derive(QueryFilter)]
struct EnemyBullets {
    marker: With<BulletMarker>,
    cond: Without<PlayerBullet>,
}
type Bullets = With<BulletMarker>;

pub fn bullet_plugin(app: &mut App) {
    app.add_event::<BulletHit>()
        .add_event::<PlayerHit>()
        .add_event::<EnemyHit>()
        .add_systems(
            FixedUpdate,
            (
                check_enemy_bullets,
                check_bullet_bullet,
                move_normal_bullets,
                move_rotating_bullets,
                despawn_bullets,
                fire_weapons,
                tick_bullets,
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
            ..Default::default()
        },
        bullet_type: BulletType::Normal(NormalBullet {
            velocity: Vec2::new(-20.0, 0.0),
        }),
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
                ..Default::default()
            },
            bullet_type: BulletType::Normal(NormalBullet {
                velocity: Vec2::new(-20.0, 0.0),
            }),
            salted: true,
        })
        .insert(AltFire);
}

#[derive(Component)]
pub struct Weapon {
    timer: Timer,
    ammo_cost: u32,
    bullet: BulletBundle,
    bullet_type: BulletType,
    salted: bool,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum BulletType {
    Normal(NormalBullet),
    Rotating(RotatingBullet),
    Homing(HomingBullet),
}

impl Weapon {
    fn spawn_bullet(&mut self, commands: &mut Commands, player_pos: Vec2) {
        self.timer.reset();

        let bullet = BulletBundle {
            transform: Transform {
                translation: player_pos.extend(0.0) + self.bullet.transform.translation,
                ..self.bullet.transform
            },
            ..self.bullet.clone()
        };

        let mut ent = commands.spawn(bullet);

        ent.insert(PlayerBullet { damage: 0 })
            .insert_if(Salted, || self.salted);

        ent.add_bullet(self.bullet_type);
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
    } else {
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
            weapon.spawn_bullet(&mut commands, pos);
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct BulletMarker;

#[derive(Bundle, Default, Clone)]
pub struct BulletBundle {
    pub transform: Transform,
    pub collider: Collider,
    pub sprite: Sprite,
    pub velocity: Velocity,
    pub lifetime: Lifetime,
    pub markers: (BulletMarker, TouhouMarker),
}

#[derive(Component, Default, Clone)]
pub struct Lifetime(Stopwatch);

#[derive(Component, Default)]
pub struct PlayerBullet {
    damage: u32,
}

#[derive(Component, Clone, Copy, Default, Debug)]
pub struct NormalBullet {
    pub velocity: Vec2,
}

#[derive(Component, Clone, Copy, Default, Debug)]
pub struct HomingBullet {
    pub rotation_speed: f32,
    pub seeking_time: f32,
}

#[derive(Debug, Copy, Clone, Component)]
pub struct RotatingBullet {
    pub origin: Vec2,
    // rotation speed in radians/s
    pub rotation_speed: f32,
}

fn circle(t: &Transform, c: &Collider) -> Circle {
    super::Circle::new(c.radius, t.translation.xy())
}

#[derive(Component, Default)]
pub struct Salted;

#[derive(Component, Default)]
pub struct Phasing;

#[derive(Event)]
pub struct PlayerHit(Entity);

#[derive(Debug, Clone, Component, Default)]
pub struct Velocity {
    velocity: Vec2,
}

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
    player_bullets: Query<(&NormalBullet, Option<&Salted>), PlayerBullets>,
    enemy_bullets: Query<&NormalBullet, EnemyBullets>,
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
    player_bullets: Query<(Entity, &Transform, &Collider), (PlayerBullets, Without<Phasing>)>,
    enemy_bullets: Query<(Entity, &Transform, &Collider), EnemyBullets>,
) {
    for (p, p_trans, p_coll) in &player_bullets {
        let player_circle = circle(p_trans, p_coll);

        for (e, e_trans, e_coll) in &enemy_bullets {
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
    bullet_query: Query<(Entity, &Transform, &Collider), EnemyBullets>,
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
    bullet_query: Query<(Entity, &Transform), EnemyBullets>,
) {
    for (entity, transform) in &bullet_query {
        if !Rect::new(-1000.0, -1000.0, 1000.0, 1000.0).contains(transform.translation.xy()) {
            commands.entity(entity).despawn()
        }
    }
}

fn move_normal_bullets(mut bullet_query: Query<(&NormalBullet, &mut Transform)>) {
    for (bullet, mut transform) in &mut bullet_query {
        transform.translation += bullet.velocity.extend(0.0);
    }
}

fn move_rotating_bullets(
    time: Res<Time>,
    mut bullet_query: Query<(&RotatingBullet, Option<&mut NormalBullet>, &mut Transform)>,
    mut gizmos: Gizmos,
) {
    for (bullet, normal, mut trans) in &mut bullet_query {
        let prev_pos = trans.translation.xy();

        let pos_mod = prev_pos - bullet.origin;

        gizmos.cross_2d(Isometry2d::from_translation(bullet.origin), 20.0, RED);

        let angle = bullet.rotation_speed * time.delta_secs();

        let rotated = Vec2::from_angle(angle).rotate(pos_mod);

        let new_pos = rotated + bullet.origin;

        if let Some(mut normal) = normal {
            normal.velocity = Vec2::from_angle(angle).rotate(normal.velocity);
        }

        trans.translation = new_pos.extend(0.0);
    }
}

fn move_homing_bullets(
    time: Res<Time>,
    mut bullet_query: Query<(&HomingBullet, &mut Velocity, &Lifetime, &mut Transform)>,
    player: PlayerQ<&Transform>,
) {
    let playerpos = player.into_inner();

    for (bullet, mut velocity, lifetime, mut trans) in &mut bullet_query {
        if lifetime.0.elapsed_secs() > bullet.seeking_time {
            continue;
        }
        let angle = (playerpos.translation.xy() - trans.translation.xy()).normalize();
        let rotation = bullet.rotation_speed * time.delta_secs();

        velocity.velocity = velocity.velocity.rotate_towards(angle, rotation);
    }
}

fn tick_bullets(time: Res<Time>, mut bullets: Query<&mut Lifetime, Bullets>) {
    for mut watch in &mut bullets {
        watch.0.tick(time.delta());
    }
}

pub trait BulletCommandExt {
    fn add_bullet<T: AsBulletKind>(&mut self, kind: T) -> &mut Self;
}

trait AsBulletKind {
    fn as_bullet_type(self) -> BulletType;
}

impl AsBulletKind for RotatingBullet {
    fn as_bullet_type(self) -> BulletType {
        BulletType::Rotating(self)
    }
}

impl AsBulletKind for NormalBullet {
    fn as_bullet_type(self) -> BulletType {
        BulletType::Normal(self)
    }
}

impl AsBulletKind for HomingBullet {
    fn as_bullet_type(self) -> BulletType {
        BulletType::Homing(self)
    }
}

impl AsBulletKind for BulletType {
    fn as_bullet_type(self) -> BulletType {
        self
    }
}

impl<'a> BulletCommandExt for EntityCommands<'a> {
    fn add_bullet<T: AsBulletKind>(&mut self, kind: T) -> &mut Self {
        let kind = kind.as_bullet_type();

        match kind {
            BulletType::Normal(normal) => self.insert(normal),
            BulletType::Rotating(rotating) => self.insert(rotating),
            BulletType::Homing(homing) => self.insert(homing),
        }
    }
}
