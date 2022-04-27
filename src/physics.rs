use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paused(pub bool);

#[derive(Deref, DerefMut, Clone)]
pub struct Gravity(pub f32);

#[derive(Component, Clone)]
pub struct Bounciness(pub f32);

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
        #[rustfmt::skip] // because trailing comma
        app.insert_resource(Gravity(self.gravity))
            .add_state(Paused(true))
            .add_system(pause_system)
            .add_system_set(
                SystemSet::on_update(Paused(false))
                    .with_system(gravity_system)
                    .with_system(velocity_system.after(gravity_system))
                    .with_system(collision_system.after(velocity_system))
                    // .with_system(debug_stepper_system.after(collision_system))
            );
    }
}

fn pause_system(mut mode: ResMut<State<Paused>>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::Return) {
        let paused = mode.current().0;
        mode.set(Paused(!paused)).unwrap();
    }
}

fn debug_stepper_system(mut mode: ResMut<State<Paused>>) {
    mode.set(Paused(true)).unwrap();
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
            let distance_squared = positional_difference.length_squared();

            if distance_squared == 0.0 {
                return;
            }

            let acceleration = gravity.0 * dest_mass.0 / distance_squared;

            let distance = distance_squared.sqrt();
            let direction = positional_difference / distance;

            from_velocity.0 += direction * acceleration;
        });
    });
}

pub fn collision_system(
    mut objects: Query<(&Bounciness, &Radius, &Mass, &mut Transform, &mut Velocity)>,
) {
    let mut pairs = objects.iter_combinations_mut::<2>();
    while let Some(pair) = pairs.fetch_next() {
        // unpack stuff
        #[rustfmt::skip]
        let [
            (a_bounciness, a_radius, a_mass, mut a_transform, mut a_velocity),
            (b_bounciness, b_radius, b_mass, mut b_transform, mut b_velocity),
        ] = pair;

        // calculate whether they collided
        let positional_difference = b_transform.translation - a_transform.translation;
        let distance_squared = positional_difference.length_squared();
        let combined_radius = a_radius.0 + b_radius.0;
        let collided = distance_squared < combined_radius.powi(2);

        // if they don't collide, skip this pair
        if !collided {
            continue;
        }

        // find the collision depth
        let distance = distance_squared.sqrt();
        let collision_depth = combined_radius - distance;

        // find how they collide
        let direction_a_to_b = positional_difference / distance;
        let collision_vector = direction_a_to_b * collision_depth;

        // get their masses relative to each other
        let combined_mass = a_mass.0 + b_mass.0;
        let (a_mass_fraction, b_mass_fraction) = if combined_mass != 0.0 {
            (a_mass.0 / combined_mass, b_mass.0 / combined_mass)
        } else {
            // zero mass objects have equal mass
            (0.5, 0.5)
        };

        // shift their positions to no longer collide
        // mass forces the other to make more way; a affects b, b affects a
        a_transform.translation -= collision_vector * b_mass_fraction;
        b_transform.translation += collision_vector * a_mass_fraction;

        // project their velocities to find how quickly they impacted each other
        let a_impact_velocity = a_velocity.project_onto_normalized(direction_a_to_b);
        let b_impact_velocity = -b_velocity.project_onto_normalized(direction_a_to_b);
        let combined_impact_velocity = a_impact_velocity + b_impact_velocity;

        // calculate how much bounce there was
        let bounce_power = combined_impact_velocity * a_bounciness.0 * b_bounciness.0 * 2.0;
        a_velocity.0 -= bounce_power * b_mass_fraction;
        b_velocity.0 += bounce_power * a_mass_fraction;
    }
}

#[derive(Bundle, Clone)]
pub struct SphereBundle {
    pub radius: Radius,
    #[bundle]
    pub mesh: PbrBundle,
}

pub struct SphereBuilder {
    pub radius: f32,
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
            radius: Radius(self.radius),
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
            radius: 1.0,
            location: default(),
            color: Color::ORANGE_RED,
            subdivisions: 0,
        }
    }
}
