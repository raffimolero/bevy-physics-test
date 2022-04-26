mod fly;
mod physics;

use bevy::prelude::*;
use fly::*;
use physics::{PhysicsPlugin, SphereBuilder};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyControlsPlugin)
        .add_plugin(PhysicsPlugin { gravity: 10.0 })
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn_bundle(
    //     SphereBuilder {
    //         location: Vec3::new(0.0, 0.0, -15.0),
    //         velocity: Vec3::new(0.0, 0.0, -1.0),
    //         ..default()
    //     }
    //     .build(&mut meshes, &mut materials),
    // );
    // commands.spawn_bundle(
    //     SphereBuilder {
    //         location: Vec3::new(1.0, 0.0, -20.0),
    //         velocity: Vec3::new(0.0, 1.0, 0.0),
    //         ..default()
    //     }
    //     .build(&mut meshes, &mut materials),
    // );

    let mut sphere = SphereBuilder {
        // marked default are the stuff that should be programmatically modified.
        // i do not want to give an explicit default value.
        location: Vec3::new(default(), 0.0, -50.0),
        velocity: Vec3::new(default(), 1.0, 0.0),
        ..default()
    }
    .build(&mut meshes, &mut materials);
    for x in -2..=2 {
        sphere.mesh.transform.translation.x = x as f32 * 5.0;
        sphere.velocity.x = x as f32 * 0.1;
        commands.spawn_bundle(sphere.clone());
    }

    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert_bundle(FlyControlsBundle::default());
}
