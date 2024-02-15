/*
2d heatmap that determines the mesh density (defines the border of the heart where dense enough).
but the mesh is not dense enough so we can still animate the tracing of it with a TSP algorithm.
*/

use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle, Material2dPlugin},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<CustomMaterial>::default()) // check and see if a material2dplugin actually exists
        .add_systems(Startup, setup)
        .add_systems(Update, 
            (
                draw_cursor, 
                setup_square
            )
        )
        .run();
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    gizmos.circle_2d(point, 10., Color::WHITE);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}

fn setup_square(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Quad::default());
    let vertex_colors: Vec<[f32; 4]> = vec![
        Color::RED.as_rgba_f32(),
        Color::GREEN.as_rgba_f32(),
        Color::BLUE.as_rgba_f32(),
        Color::WHITE.as_rgba_f32(),
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();
    commands.spawn(MaterialMesh2dBundle {
        mesh:mesh_handle.clone(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(CustomMaterial {}),
        ..default()
    });
}