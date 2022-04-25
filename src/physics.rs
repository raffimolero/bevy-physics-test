use bevy::prelude::*;

#[derive(Component)]
pub struct SphereCollision;

#[derive(Component, Deref, DerefMut)]
pub struct Mass(pub f32);
impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Radius(pub f32);
impl Default for Radius {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(motion_system);
    }
}
pub fn motion_system(mut moving_objects: Query<(&Velocity, &mut Transform)>) {
    moving_objects.for_each_mut(|(velocity, mut transform)| {
        transform.translation += velocity.0;
    });
}

#[derive(Bundle)]
pub struct SphereBundle {
    mass: Mass,
    radius: Radius,
    velocity: Velocity,

    #[bundle]
    mesh: PbrBundle,
}

pub struct SphereBuilder {
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vec3,
    pub location: Vec3,
    pub color: Color,
    pub subdivisions: usize,
}
impl SphereBuilder {
    pub fn build(
        &self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> SphereBundle {
        let mesh = meshes.add(
            shape::Icosphere {
                radius: self.radius,
                subdivisions: self.subdivisions,
            }
            .into(),
        );
        let material = materials.add(self.color.into());
        SphereBundle {
            mass: Mass(self.mass),
            radius: Radius(self.radius),
            velocity: Velocity(self.velocity),
            mesh: PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(self.location),
                ..default()
            },
        }
    }
}

impl Default for SphereBuilder {
    fn default() -> Self {
        Self {
            mass: 1.0,
            radius: 1.0,
            velocity: default(),
            location: default(),
            color: Color::ORANGE_RED,
            subdivisions: 0,
        }
    }
}
