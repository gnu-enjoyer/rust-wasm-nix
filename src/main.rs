use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

mod player;
use player::{core::spawn_player};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync, // reduce input lag
                prevent_default_event_handling: false,  // don't eat 'normal' events e.g. F5
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_player))
        .run();
}

fn setup(mut commands: Commands) {
    // camera setup
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}
