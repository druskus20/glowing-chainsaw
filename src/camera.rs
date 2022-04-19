use crate::game_state::GameState;
use bevy::{prelude::*, render::camera::Camera2d};

use smooth_bevy_cameras::{LookTransform, LookTransformBundle, Smoother};

// https://github.com/Leafwing-Studios/leafwing-input-manager/blob/446ac84cfcd2c76ae5607cca1c871681af09a0d9/src/lib.rs#L98
#[derive(Default)]
pub struct CameraPlugin {
    desired_state: Option<GameState>,
}

impl CameraPlugin {
    pub fn new() -> Self {
        Self {
            desired_state: None,
        }
    }

    pub fn run_in_state(state: GameState) -> Self {
        Self {
            desired_state: Some(state),
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        if let Some(desired_state) = self.desired_state {
            app.add_system_set(SystemSet::on_enter(desired_state).with_system(setup))
                .add_system_set(SystemSet::on_exit(desired_state).with_system(destroy));
        } else {
            app.add_startup_system(setup);
        }
    }
}

fn setup(mut commands: Commands) {
    let scale = 10.0;
    commands.spawn_bundle(CameraBundle {
        camera: Camera {
            speed: 25.0 * scale,
            bounds_x: (-5.0 * scale, 5.0 * scale),
            bounds_y: (10.0 * scale, 30.0 * scale),
            bounds_z: (-5.0 * scale, 5.0 * scale),
        },
        //look_transform: LookTransformBundle {
        //    transform: LookTransform {
        //        eye: Vec3::new(-5.0 * scale, 15.0 * scale, 5.0 * scale),
        //        target: Vec3::new(0.0 * scale, 0.0 * scale, 0.0 * scale),
        //    },
        //    smoother: Smoother::new(0.9),
        //},
        orthographic_camera: OrthographicCameraBundle::new_2d(),
    });
}

fn destroy(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    commands.entity(query.single()).despawn_recursive();
}

#[derive(Component)]
pub struct Camera {
    speed: f32,
    bounds_x: (f32, f32),
    bounds_y: (f32, f32),
    bounds_z: (f32, f32),
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            speed: 25.0,
            bounds_x: (-f32::INFINITY, f32::INFINITY),
            bounds_y: (-f32::INFINITY, f32::INFINITY),
            bounds_z: (-f32::INFINITY, f32::INFINITY),
        }
    }
}

#[derive(Bundle)]
pub struct CameraBundle {
    //#[bundle]
    //look_transform: LookTransformBundle,
    #[bundle]
    orthographic_camera: OrthographicCameraBundle<Camera2d>,
    camera: Camera,
}
