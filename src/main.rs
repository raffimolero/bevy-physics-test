mod fly;
mod physics;

use fly::*;
use physics::{Bounciness, Mass, PhysicsPlugin, SphereBuilder, Velocity};

use bevy::prelude::*;
use rand::{distributions::Uniform, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyControlsPlugin)
        .add_plugin(PhysicsPlugin {
            gravity: 2_f32.powi(-3),
        })
        .add_startup_system(setup)
        .run();
}

fn float_to_color(value: f32) -> Color {
    Color::Hsla {
        hue: value * 360.0,
        saturation: value,
        lightness: 0.5,
        alpha: 1.0,
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert_bundle(FlyControlsBundle::default());

    // you have to put a really small value
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

    let mut rng = thread_rng();
    let uniform = Uniform::<f32>::new_inclusive(0.0, 1.0);
    let mut random_float = || uniform.sample(&mut rng);

    for y in -2..=2 {
        for x in -2..=2 {
            let mass = random_float();

            let sphere = SphereBuilder {
                color: float_to_color(mass),
                radius: mass + 0.5 + random_float(),
                subdivisions: 1,
                location: Vec3::new(
                    random_float() + x as f32 * 5.0,
                    random_float() + y as f32 * 5.0,
                    random_float() - 50.0,
                ),
                ..default()
            }
            .build(&mut meshes, &mut materials);

            commands
                .spawn_bundle(sphere)
                .insert(Velocity::default())
                .insert(Mass(mass + 0.5))
                .insert(Bounciness(1.0));
        }
    }

    // sphere.mesh.transform.translation = Vec3::new(-2.0, 2.0, -50.0);
    // commands
    //     .spawn_bundle(sphere.clone())
    //     .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
    //     .insert(Mass(0.0))
    //     .insert(Bounciness(1.0));

    // sphere.mesh.transform.translation = Vec3::new(-2.0, -2.0, -50.0);
    // commands
    //     .spawn_bundle(sphere.clone())
    //     .insert(Velocity(Vec3::new(0.0, 0.1, 0.0)))
    //     .insert(Mass(0.0))
    //     .insert(Bounciness(1.0));
}
