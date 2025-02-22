use bevy::render::camera::ScalingMode;

use crate::prelude::*;

mod bullet;
mod enemy;

#[derive(Component, Clone, Default)]
struct TouhouMarker;
#[derive(Component, Default)]
struct PlayerMarker;
#[derive(Component, Default)]
struct TouhouCamera;

type PlayerQ<'a, T> = Single<'a, T, With<PlayerMarker>>;

#[derive(Component, Default, Copy, Clone, Debug)]
struct Collider {
    radius: f32,
}

impl Collider {
    fn to_circle(&self, pos: Vec2) -> Circle {
        let Self { radius } = *self;
        Circle { pos, radius }
    }
}

#[derive(Default, Copy, Clone, Debug)]
struct Circle {
    pos: Vec2,
    radius: f32,
}

impl Circle {
    fn new(radius: f32, pos: Vec2) -> Self {
        Self { pos, radius }
    }

    fn within(&self, rect: Rect) -> bool {
        let Self { pos, radius } = *self;

        let bounding_rect = Rect::from_center_size(pos, Vec2::splat(radius));

        rect.contains(bounding_rect.min) && rect.contains(bounding_rect.max)
    }

    fn hits(&self, other: Circle) -> bool {
        (self.pos - other.pos).length() - (self.radius + other.radius) < 0.0
    }
}

#[derive(Resource, Default)]
struct GameplayRect {
    rect: Rect,
}

pub fn touhou_plugin(app: &mut App) {
    app.add_plugins((bullet::bullet_plugin, enemy::enemy_plugin))
        .add_systems(
            OnEnter(GameState::Touhou),
            (spawn_player, create_gameplay_rect, make_game_camera),
        )
        .add_systems(Update, do_movement.run_if(in_state(GameState::Touhou)))
        .add_systems(PostUpdate, draw_gizmos.run_if(in_state(GameState::Touhou)));
}

fn make_game_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            viewport_origin: Vec2::splat(0.5),
            scaling_mode: ScalingMode::Fixed {
                width: 1920.0,
                height: 1080.0,
            },
            scale: 1.0,
            area: Rect::new(0.0, 0.0, 800.0, 600.0),
        },
        TouhouCamera,
    ));
}

#[derive(Bundle, Default)]
pub struct Player {
    sprite: Sprite,
    collider: Collider,
    transform: Transform,
    markers: (PlayerMarker, TouhouMarker),
}

pub fn create_gameplay_rect(mut commands: Commands) {
    const SIZE: Vec2 = Vec2::new(1920.0, 1080.0);

    commands.insert_resource(GameplayRect {
        rect: Rect {
            min: -SIZE / 2.0,
            max: SIZE / 2.0,
        },
    });
}

pub fn spawn_player(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Player {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            image: asset_server.load("mascot.png"),
            ..Default::default()
        },
        transform: Transform::from_xyz(800.0 / 2.0, 600.0 / 2.0, 0.0),
        collider: Collider { radius: 20.0 },
        ..Default::default()
    });
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    area: Res<GameplayRect>,
    colliders: Query<(&Transform, &Collider)>,
) {
    use bevy::color::palettes::css::RED;

    gizmos.rect_2d(
        Isometry2d::from_translation(area.rect.center()),
        Vec2 {
            x: area.rect.width(),
            y: area.rect.height(),
        },
        RED,
    );

    for (trans, coll) in &colliders {
        gizmos.circle_2d(
            Isometry2d::from_translation(trans.translation.xy()),
            coll.radius,
            RED,
        );
    }
}

fn do_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    area: Res<GameplayRect>,
    mut player_info: Single<(&mut Transform, &Collider), With<PlayerMarker>>,
) {
    let (mut trans, mut collider) = player_info.into_inner();
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) as i32 as f32;
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) as i32 as f32;
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) as i32 as f32;
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) as i32 as f32;

    let dy = up + -down;
    let dx = right + -left;

    let wishdir = Vec3::new(dx, dy, 0.0).normalize_or_zero() * 3.0;

    let new_pos = (trans.translation + wishdir).xy();

    let rect = area.rect;

    let rv = Vec2::splat(collider.radius);

    let new_pos_clamped = new_pos.clamp(rect.min + rv, rect.max - rv);

    trans.translation = new_pos_clamped.extend(0.0);
}
