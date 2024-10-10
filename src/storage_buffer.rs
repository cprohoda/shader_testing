use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        storage::ShaderStorageBuffer,
    },
};

const SHADER_ASSET_PATH: &str = "shaders/storage_buffer.wgsl";

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let color_data: Vec<[f32; 4]> = vec![
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [1.0, 1.0, 0.0, 1.0],
        [0.0, 1.0, 1.0, 1.0],
    ];

    let colors = buffers.add(ShaderStorageBuffer::from(color_data));

    let custom_material = CustomMaterial { colors };

    let material_handle = materials.add(custom_material);
    commands.insert_resource(CustomMaterialHandle(material_handle.clone()));

    for i in -6..=6 {
        for j in -3..=3 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(0.3)))),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(i as f32, j as f32, 0.0),
            ));
        }
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn update(
    time: Res<Time>,
    material_handle: Res<CustomMaterialHandle>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let material = materials.get_mut(&material_handle.0).unwrap();
    let buffer = buffers.get_mut(&material.colors).unwrap();
    buffer.set_data(
        (0..5)
            .map(|i| {
                let t = time.elapsed_seconds() * 5.0;
                [
                    ops::sin(t + i as f32) / 2.0 + 5.0,
                    ops::sin(t + i as f32 + 2.0) / 2.0 + 5.0,
                    ops::sin(t + i as f32 + 4.0) / 2.0 + 5.0,
                    1.0
                ]
            })
            .collect::<Vec<[f32; 4]>>()
            .as_slice(),
    );
}

#[derive(Resource)]
struct CustomMaterialHandle(Handle<CustomMaterial>);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[storage(0, read_only)]
    colors: Handle<ShaderStorageBuffer>,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
