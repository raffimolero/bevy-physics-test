mod fly;
mod physics;

use fly::*;
use physics::{PhysicsPlugin, SphereBuilder};

use bevy::prelude::*;
use rand::{distributions::Uniform, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyControlsPlugin)
        .add_plugin(PhysicsPlugin { gravity: 1.0 })
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert_bundle(FlyControlsBundle::default());

    let mut sphere = SphereBuilder::default().build(&mut meshes, &mut materials);

    let mut rng = thread_rng();
    let uniform = Uniform::<f32>::new(-1.0, 1.0);

    for y in -2..=2 {
        for x in -2..=2 {
            sphere.mesh.transform.translation = Vec3::new(
                uniform.sample(&mut rng) + x as f32 * 5.0,
                uniform.sample(&mut rng) + y as f32 * 5.0,
                uniform.sample(&mut rng) - 50.0,
            );
            commands.spawn_bundle(sphere.clone());
        }
    }
}
