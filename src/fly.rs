use bevy::{input::mouse::MouseMotion, math::vec3, prelude::*};

pub struct FlyControlsPlugin;
impl Plugin for FlyControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_lookaround).add_system(keyboard_motion);
    }
}

#[derive(Component, Default)]
pub struct FlyControls;

#[derive(Component, Clone, Copy)]
pub struct Speed(Vec3);
impl Default for Speed {
    fn default() -> Self {
        Self(Vec3::splat(2_f32.powi(8)))
    }
}

#[derive(Component, Clone, Copy)]
pub struct LookSens(pub f32, pub f32);
impl Default for LookSens {
    fn default() -> Self {
        Self(2_f32.powi(-10), 2_f32.powi(-10))
    }
}

/// `Facing.0` is the rotation along the y axis, which forms a *horizontal circle.*
///
/// `Facing.1` is the rotation along the x axis, which should form a *vertical semicircle facing forward.*
#[derive(Component, Clone, Copy, Default)]
pub struct Facing(pub f32, pub f32);
impl From<Facing> for Quat {
    fn from(face: Facing) -> Self {
        Quat::from_euler(EulerRot::YXZ, face.0, face.1, 0.0)
    }
}

#[derive(Bundle, Default)]
pub struct FlyControlsBundle {
    tag: FlyControls,
    speed: Speed,
    facing: Facing,
    look_sens: LookSens,
}

/// keyboard controls for moving the object.
fn keyboard_motion(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut controllables: Query<(&Speed, &Facing, &mut Transform), With<FlyControls>>,
) {
    use KeyCode::*;
    let delta = time.delta_seconds();

    // 2 keys control positive and negative motion along one axis.
    let key_motion = |positive: KeyCode, negative: KeyCode| -> f32 {
        (keys.pressed(positive) as i8 - keys.pressed(negative) as i8) as f32
    };

    // combine all 3 axis controls:
    let movement = vec3(
        key_motion(D, A),          // Right, Left
        key_motion(Space, LShift), // Up, Down
        key_motion(S, W),          // Backward, Forward
    ) * delta;
    if movement == Vec3::ZERO {
        return;
    }

    // move things.
    controllables.for_each_mut(|(speed, facing, mut transform)| {
        // rotate the motion based on where the object is facing horizontally.
        let compass_direction = Quat::from_rotation_y(facing.0);
        transform.translation += compass_direction * (movement * speed.0);
    });
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

/// mouse controls for looking around.
fn mouse_lookaround(
    mut windows: ResMut<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut mouse_events: EventReader<MouseMotion>,
    mut controllables: Query<(&LookSens, &mut Facing, &mut Transform), With<FlyControls>>,
) {
    let window = windows.get_primary_mut().unwrap();

    let cursor_locked = window.cursor_locked();
    if buttons.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(!cursor_locked);
        window.set_cursor_visibility(cursor_locked);
    }

    // don't look around when the mouse isn't captured.
    if !cursor_locked {
        return;
    }

    // compile all the movements into one variable
    let mut total_delta = Vec2::ZERO;
    for motion in mouse_events.iter() {
        total_delta += motion.delta;
    }

    // make things look around.
    controllables.for_each_mut(|(look_sens, mut facing, mut transform)| {
        // add the motions to the rotations.
        facing.0 -= total_delta.x * look_sens.0;
        facing.1 -= total_delta.y * look_sens.0;

        // prevent infinite rotation on either axis.
        use std::f32::consts::{FRAC_PI_2 as QUARTER_TURN, TAU};
        facing.0 %= TAU;
        facing.1 = facing.1.clamp(-QUARTER_TURN, QUARTER_TURN);

        // actual camera rotation
        transform.rotation = Quat::from(*facing);
    })
}
