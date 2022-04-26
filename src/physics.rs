use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Paused(bool);

#[derive(Deref, DerefMut, Clone)]
pub struct Gravity(f32);

#[derive(Component, Clone)]
pub struct SphereCollider;

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Mass(pub f32);
impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Radius(pub f32);
impl Default for Radius {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Default, Deref, DerefMut, Clone)]
pub struct Velocity(pub Vec3);

pub struct PhysicsPlugin {
    pub gravity: f32,
}
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(self.gravity))
            .add_state(Paused(true))
            .add_system(pause_system)
            .add_system_set(
                SystemSet::on_update(Paused(false))
                    .with_system(velocity_system)
                    .with_system(gravity_system),
            );
    }
}

fn pause_system(mut mode: ResMut<State<Paused>>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::Return) {
        let paused = mode.current().0;
        mode.set(Paused(!paused)).unwrap();
    }
}

pub fn velocity_system(mut objects: Query<(&Velocity, &mut Transform)>) {
    objects.for_each_mut(|(velocity, mut transform)| {
        transform.translation += velocity.0;
    });
}

pub fn gravity_system(
    gravity: Res<Gravity>,
    attractors: Query<(&Transform, &Mass)>,
    mut attractees: Query<(&Transform, &mut Velocity)>,
) {
    attractees.for_each_mut(|(from_transform, mut from_velocity)| {
        attractors.for_each(|(dest_transform, dest_mass)| {
            let positional_difference = dest_transform.translation - from_transform.translation;
            let distance_squared = positional_difference.length_squared().max(1.0);

            let acceleration = gravity.0 * dest_mass.0 / distance_squared;

            let distance = distance_squared.sqrt();
            let direction = positional_difference / distance;

            from_velocity.0 += direction * acceleration;
        });
    });
}

#[derive(Bundle, Clone)]
pub struct SphereBundle {
    pub mass: Mass,
    pub radius: Radius,
    pub velocity: Velocity,
    #[bundle]
    pub mesh: PbrBundle,
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
