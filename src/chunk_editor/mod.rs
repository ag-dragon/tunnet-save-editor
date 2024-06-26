mod voxels;
mod camera;

use camera::CameraPlugin;
use voxels::VoxelType;

use crate::CurrentSave;

use tunnet_save::chunks::ChunkCoords;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        mesh::{
            PrimitiveTopology,
            VertexAttributeValues,
            Indices,
        },
    },
};
use block_mesh::{
    ndshape::{ConstShape, ConstShape3u32},
    RIGHT_HANDED_Y_UP_CONFIG,
    visible_block_faces,
    UnitQuadBuffer,
};

const CHUNK_WIDTH: u32 = 32;
const PADDED_CHUNK_WIDTH: u32 = 34;
type ChunkShape = ConstShape3u32<CHUNK_WIDTH, CHUNK_WIDTH, CHUNK_WIDTH>;
type PaddedChunkShape = ConstShape3u32<PADDED_CHUNK_WIDTH, PADDED_CHUNK_WIDTH, PADDED_CHUNK_WIDTH>;

#[derive(Resource, Default)]
pub struct CurrentChunk(pub ChunkCoords);

pub struct ChunkEditorPlugin;

impl Plugin for ChunkEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentChunk>()
            .add_plugins(CameraPlugin)
            .add_systems(Startup, chunk_editor_setup)
            .add_systems(Update, chunk_editor_update);
    }
}

#[derive(Component)]
pub struct BlockMesh;

pub fn chunk_editor_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_genblockmesh: EventWriter<GenBlockMeshEvent>,
) {
    // light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 400.0,
    });

    // camera
    /*
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(32.0, 64.0, 64.0),
            ..default()
        },
        PanOrbitCamera {
            focus: Vec3::new(16.5, 16.5, 16.5),
            ..default()
        }
    ));
    */


    let custom_texture_handle: Handle<Image> = asset_server.load("textures/array_texture.png");
    let custom_mesh_handle: Handle<Mesh> = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    commands.spawn((
        PbrBundle {
            mesh: custom_mesh_handle,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(custom_texture_handle),
                ..default()
            }),
            ..default()
        },
        BlockMesh,
    ));

    ev_genblockmesh.send(GenBlockMeshEvent);
}

#[derive(Event)]
pub struct GenBlockMeshEvent;

pub fn chunk_editor_update(
    mut ev_genblockmesh: EventReader<GenBlockMeshEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&Handle<Mesh>, With<BlockMesh>>,
    save_file: Res<CurrentSave>,
    current_chunk: Res<CurrentChunk>,
) {
    for _ in ev_genblockmesh.read() {
        let mut voxels = [VoxelType::Dirt; ChunkShape::SIZE as usize];
        match save_file.0.chunk_data.chunks.get(&current_chunk.0) {
            Some(rle_chunk) => {
                let voxel_data = decode_rle(rle_chunk);
                for i in 0..ChunkShape::SIZE {
                    voxels[i as usize] = VoxelType::try_from(voxel_data[i as usize]).unwrap();
                }
            },
            None => {},
        }

        let handle = query.get_single_mut().expect("Error getting block mesh");
        if let Some(mesh) = meshes.get_mut(handle.id()) {
            *mesh = generate_chunk_mesh(voxels);
        }
    }
}

pub fn decode_rle(encoded_data: &Vec<[i32; 2]>) -> Vec<i32> {
    let mut output = Vec::new();
    for data in encoded_data {
        output.extend(vec![data[1]; data[0] as usize]);
    }
    output
}

pub fn encode_rle(data: &Vec<i32>) -> Vec<[i32; 2]> {
    let mut output = Vec::new();
    let mut current_run: [i32; 2] = [0, 0];
    for voxel in data {
        if current_run[0] == 0 {
            current_run[0] = 1;
            current_run[1] = *voxel;
        } else {
            if current_run[1] == *voxel {
                current_run[0] += 1;
            } else {
                output.push(current_run);
                current_run = [1, *voxel];
            }
        }
    }
    output.push(current_run);

    output
}

fn pad_voxels(voxels: [VoxelType; ChunkShape::SIZE as usize]) -> [VoxelType; PaddedChunkShape::SIZE as usize] {
    let mut padded_voxels = [VoxelType::Air; PaddedChunkShape::SIZE as usize];
    for i in 0..ChunkShape::SIZE {
        let [x, y, z] = ChunkShape::delinearize(i);
        padded_voxels[PaddedChunkShape::linearize([x+1, y+1, z+1]) as usize] = voxels[i as usize];
    }
    padded_voxels
}

fn generate_chunk_mesh(voxels: [VoxelType; ChunkShape::SIZE as usize]) -> Mesh {
    let padded_voxels = pad_voxels(voxels);

    let mut buffer: UnitQuadBuffer = UnitQuadBuffer::new();
    visible_block_faces(
        &padded_voxels,
        &PaddedChunkShape {},
        [0; 3],
        [33; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );

    mesh_from_quads(buffer, padded_voxels)
}

// Function heavily inspired by mesh_from_quads function from bevy_voxel_world crate
fn mesh_from_quads(quads: UnitQuadBuffer, voxels: [VoxelType; PaddedChunkShape::SIZE as usize]) -> Mesh {
    let num_indices = quads.num_quads() * 6;
    let num_vertices = quads.num_quads() * 4;

    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_coords = Vec::with_capacity(num_vertices);

    for (group, face)  in quads.groups.into_iter()
            .zip(RIGHT_HANDED_Y_UP_CONFIG.faces.into_iter()) {
        for quad in group.into_iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(&quad.into(), 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());

            let voxel= voxels[PaddedChunkShape::linearize(quad.minimum) as usize];
            let voxel_texture_coords = voxel.atlas_coords();
            tex_coords.extend_from_slice(&face.tex_coords(
                RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                true,
                &quad.into(),
            ).map(|coords|
                [
                    (coords[0] / 4.0) + voxel_texture_coords[0],
                    (coords[1] / 4.0) + voxel_texture_coords[1],
                ]
            ));
        }
    }

    let mut render_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions.clone()),
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float32x2(tex_coords),
    );

    render_mesh.insert_indices(Indices::U32(indices.clone()));

    render_mesh
}
