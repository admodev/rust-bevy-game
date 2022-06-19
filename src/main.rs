use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 80.0 / 60.0;

fn main() {
    let height: f32 = 600.0;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Bevy Tutorial".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera: OrthographicCameraBundle = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = -1.0;
    camera.orthographic_projection.left = -1.0;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
