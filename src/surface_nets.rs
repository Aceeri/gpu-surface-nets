use bevy::{
    material::descriptor::BindGroupLayoutDescriptor,
    prelude::*,
    render::render_resource::{
        BindGroupLayoutEntries, PipelineCache, ShaderStages, StorageTextureAccess, TextureFormat,
        binding_types::{storage_buffer, texture_storage_3d},
    },
};

#[derive(Default)]
pub struct GpuSurfaceNets;

impl Plugin for GpuSurfaceNets {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            panic!("Missing RenderApp");
            return;
        };

        // alright we need buffers to read from? how do we dispatch and stuff
        render_app
            .insert_resource(VoxelSamples::filled(0u32))
            .add_systems(RenderStartup, init_surface_net_pipeline)
            .add_systems(Render, prepare_voxel_buffer)
            .add_systems(RenderGraph, compute_mesh.before(camera_driver));
    }
}

pub const WIDTH: usize = 18;
pub const SAMPLES: usize = WIDTH * WIDTH * WIDTH;

#[derive(Debug, Component)]
pub struct VoxelSamples {
    samples: [u32; SAMPLES],
}

impl VoxelSamples {
    pub fn empty() -> Self {
        Self::filled(0u32) // air
    }

    pub fn filled(voxel: u32) -> Self {
        Self {
            samples: [voxel; SAMPLES],
        }
    }
}

pub struct SurfaceNetPipeline {}
pub fn init_surface_net_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pipeline_cache: Res<PipelineCache>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "centroid_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                // voxels
                texture_storage_3d(TextureFormat::R32Uint, StorageTextureAccess::ReadWrite),
                // centroids
                storage_buffer::<Vec<u32>>(false),
            ),
        ),
    );
    let shader = asset_server.load(CENTROID_GENERATE_SHADER);
}

pub fn prepare_voxel_buffer() {}
pub fn compute_mesh() {}
