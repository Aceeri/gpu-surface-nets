use bevy::{
    core_pipeline::schedule::camera_driver,
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedComputePipelineId, ComputePipelineDescriptor,
    },
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup,
        render_resource::{
            BindGroupLayoutEntries, PipelineCache, ShaderStages, StorageTextureAccess,
            TextureFormat,
            binding_types::{storage_buffer, texture_storage_3d, uniform_buffer},
        },
    },
};

const SURFACE_NET_SHADER: &'static str = "surface_net.wgsl";

#[derive(Default)]
pub struct GpuSurfaceNets;
impl Plugin for GpuSurfaceNets {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            panic!("Missing RenderApp");
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

#[derive(Debug, Resource)]
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

#[derive(Resource)]
pub struct SurfaceNetPipeline {
    layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
}
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
                // target voxel
                uniform_buffer::<u32>(false),
                // voxels
                texture_storage_3d(TextureFormat::R32Uint, StorageTextureAccess::ReadOnly),
                // centroids
                storage_buffer::<Vec<u32>>(false),
            ),
        ),
    );
    let shader = asset_server.load(SURFACE_NET_SHADER);
    let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("Mesh generation compute shader".into()),
        layout: vec![layout.clone()],
        shader: shader.clone(),
        entry_point: Some("compute_centroids".into()),
        ..default()
    });
    commands.insert_resource(SurfaceNetPipeline { layout, pipeline });
}

pub fn prepare_voxel_buffer() {}
pub fn compute_mesh() {}
