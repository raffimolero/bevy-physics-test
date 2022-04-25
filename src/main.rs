mod fly;
mod physics;

use bevy::prelude::*;
use fly::*;
use physics::{PhysicsPlugin, SphereBuilder};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyControlsPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = SphereBuilder {
        location: Vec3::new(0.0, 0.0, -5.0),
        velocity: Vec3::new(0.0, 0.0, -1.0),
        ..default()
    }
    .build(&mut meshes, &mut materials);
    commands.spawn_bundle(sphere);
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert_bundle(FlyControlsBundle::default());
}
