use std::{thread, time::Duration};

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
// use bevy_polyline::*;

#[derive(Debug, Component)]
struct Momentum(Vec3);

#[derive(Component)]
struct Cursor;

#[derive(Component)]
struct Speed(f32);

// struct DragFactor(f32);
// struct MaxSpeed(f32);
struct Radius {
    value: f32,
    square: f32,
}
impl Radius {
    fn new(value: f32) -> Self {
        Self {
            value,
            square: value * value,
        }
    }
}
struct Gravity(f32);

// intended to debug the physics.
// struct Pause(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(PolylinePlugin)
        // .insert_resource(DragFactor(0.99))
        // .insert_resource(MaxSpeed(1e6))
        // .insert_resource(Pause(false))
        .insert_resource(Radius::new(5.0))
        // .insert_resource(Radius::new(0.5))
        .insert_resource(Gravity(1e4))
        .add_startup_system(spawn.label("spawn"))
        .add_startup_system(connect.after("spawn"))
        .add_system(motion.label("motion"))
        .add_system(attract)
        .add_system(follow_mouse.label("mouse").after("motion"))
        .add_system(options.after("mouse"))
        .add_system(camera_motion)
        .run();
}

fn spawn_mesh(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    pos: Vec3,
    momentum: Vec3,
) -> Entity {
    commands
        .spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform {
                translation: pos,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Momentum(momentum))
        .id()
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    radius: Res<Radius>,
) {
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Speed(500.0));

    // light
    // let rotation = Quat::default();
    // let rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
    // let rotation = Quat::from_axis_angle(
    //     vec3(-1.0, 1.0, 0.0).normalize(),
    //     std::f32::consts::FRAC_PI_4,
    // );
    let rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2 - 0.000001);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 25_000.0,
            ..Default::default()
        },
        transform: Transform::default().with_rotation(rotation),
        ..Default::default()
    });

    let mesh = meshes.add(
        shape::UVSphere {
            radius: radius.value,
            sectors: 12,
            stacks: 6,
        }
        .into(),
    );

    let material = materials.add(Color::RED.into());
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.clone(),
        material,
        ..Default::default()
    });

    let material = materials.add(Color::BLUE.into());
    // spawn free objects
    for x in (-100..=100).step_by(20) {
        for z in (-40..40).step_by(20) {
            spawn_mesh(
                &mut commands,
                mesh.clone(),
                material.clone(),
                vec3(x as f32, 0.0, z as f32),
                vec3(0.0, (x / 10) as f32, 0.0),
            );
        }
    }

    let material = materials.add(Color::GREEN.into());
    // spawn cursor object
    let mouse_ball = spawn_mesh(
        &mut commands,
        mesh.clone(),
        material.clone(),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, 0.0),
    );
    commands.entity(mouse_ball).insert(Cursor);
}
// fn connect(mut cmds: Commands, objects: Query<Entity, Without<Cursor>>) {
//     for [a, b] in objects.iter_combinations() {
fn connect(mut commands: Commands, objects: Query<(Entity, Option<&Cursor>)>) {

    // cmds.spawn_bundle(ColorMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    //     transform: Transform::default()
    //         .with_scale(Vec3::splat(128.))
    //         .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
    //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //     ..Default::default()
    // });
    // for [(a, a_cursor), (b, b_cursor)] in objects.iter_combinations() {
    // workaround for a bug in bevy that doesn't let me use iter_combinations on a Without<> query
    // if let (None, None) = (a_cursor, b_cursor) {
    // cmds.spawn_bundle(/* TODO: */).insert(Spring(a, b));
    // }
    // }
}

fn motion(time: Res<Time>, mut objects: Query<(&Momentum, &mut Transform)>) {
    let delta = time.delta_seconds();
    objects.for_each_mut(|(momentum, mut transform)| {
        transform.translation += momentum.0 * delta;
    })
}

fn attract(
    time: Res<Time>,
    radius: Res<Radius>,
    gravity: Res<Gravity>,
    mut objects: Query<(&mut Transform, &mut Momentum)>,
) {
    // if pause.0 {
    //     return;
    // }
    let delta = time.delta_seconds();
    let mut pairs = objects.iter_combinations_mut();
    while let Some([(mut a_transform, mut a_momentum), (mut b_transform, mut b_momentum)]) =
        pairs.fetch_next()
    {
        let diff = a_transform.translation - b_transform.translation;
        if let Some(direction) = diff.try_normalize() {
            let len_sq = (diff / 2.0).length_squared();
            if len_sq > radius.square {
                let attraction = gravity.0 / len_sq;
                let effect = direction * attraction * delta;
                a_momentum.0 -= effect;
                b_momentum.0 += effect;
            } else {
                // b o u n c e
                let len = len_sq.sqrt();
                let overlap = radius.value - len;
                let shift = direction * overlap;
                a_transform.translation += shift;
                b_transform.translation -= shift;
                let bounce =
                    a_momentum.0.project_onto(direction) - b_momentum.0.project_onto(direction);
                a_momentum.0 -= bounce;
                b_momentum.0 += bounce;
            }
        }
    }
}

fn follow_mouse(
    time: Res<Time>,
    windows: Res<Windows>,
    mut camera: Query<(&Camera, &GlobalTransform)>,
    mut cursor: Query<(&Transform, &mut Momentum), With<Cursor>>,
) {
    let delta = time.delta_seconds();
    if delta == 0.0 {
        return;
    }
    let (camera, camera_transform) = camera.single_mut();
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        let window_size = vec2(window.width(), window.height());
        let ndc = (pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera.projection_matrix.inverse();
        let mut pos = ndc_to_world.project_point3(
            (ndc * camera_transform.translation.length() /* * 0.75 */ * 10.0).extend(1.0),
        );
        pos = camera_transform.rotation * pos;
        // pos += camera_transform.translation * 0.25;

        let (transform, mut momentum) = cursor.single_mut();
        momentum.0 = (pos - transform.translation) / delta;
    }
}

fn options(
    mut camera: Query<&mut Speed, With<Camera>>,
    mut gravity: ResMut<Gravity>,
    keys: Res<Input<KeyCode>>,
) {
    use KeyCode::*;
    const GRAV_SENS: f32 = 1.25;
    const SPEED_SENS: f32 = 1.25;

    let mut speed = camera
        .get_single_mut()
        .expect("There isn't exactly one camera.");
    if keys.just_pressed(Period) {
        speed.0 *= SPEED_SENS;
    }
    if keys.just_pressed(Comma) {
        speed.0 /= SPEED_SENS;
    }

    if keys.just_pressed(Equals) {
        gravity.0 *= GRAV_SENS;
        println!("Set gravity to {}.", gravity.0);
    }
    if keys.just_pressed(Minus) {
        gravity.0 /= GRAV_SENS;
        println!("Set gravity to {}.", gravity.0);
    }
}

fn camera_motion(
    time: Res<Time>,
    mut camera: Query<(&Speed, &mut Transform), With<Camera>>,
    keys: Res<Input<KeyCode>>,
) {
    let delta = time.delta_seconds();
    use KeyCode::*;

    let (speed, mut transform) = camera
        .get_single_mut()
        .expect("There isn't exactly one camera.");
    let x_move = (keys.pressed(D) as i32 - keys.pressed(A) as i32) as f32;
    let y_move = (keys.pressed(Space) as i32 - keys.pressed(LShift) as i32) as f32;
    let z_move = (keys.pressed(S) as i32 - keys.pressed(W) as i32) as f32;
    if x_move == 0.0 && y_move == 0.0 && z_move == 0.0 {
        return;
    }
    let motion = transform
        .rotation
        .mul_vec3(vec3(x_move, y_move, z_move).normalize());
    transform.translation += motion * delta * speed.0;
    transform.look_at(Vec3::ZERO, Vec3::Y);
}
