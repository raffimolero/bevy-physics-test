mod fly;

use bevy::prelude::*;
use fly::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyControlsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(
        shape::Icosphere {
            radius: 10.0,
            subdivisions: 0,
        }
        .into(),
    );
    let material = materials.add(Color::RED.into());

    commands.spawn_bundle(PbrBundle {
        mesh,
        material,
        ..Default::default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert_bundle(FlyControlsBundle::default());
}
