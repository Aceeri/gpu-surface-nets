use bevy::{
    core_pipeline::schedule::camera_driver,
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedComputePipelineId, ComputePipelineDescriptor,
    },
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup,
        render_resource::{
            BindGroupLayoutEntries, PipelineCache, ShaderStages, ShaderType, StorageTextureAccess,
            TextureFormat,
            binding_types::{
                storage_buffer, storage_buffer_read_only, texture_storage_3d, uniform_buffer,
            },
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
            // .insert_resource(VoxelSamples::filled(0u32))
            // .insert_resource()
            .add_systems(RenderStartup, init_surface_net_pipeline)
            .add_systems(Render, prepare_voxels)
            .add_systems(RenderGraph, compute_mesh.before(camera_driver));
    }
}

pub const CHUNK_DIM: usize = 16;
pub const CHUNK_LEN: usize = CHUNK_DIM.pow(3);
pub const SLAB_BYTES: usize = CHUNK_LEN * 4;

// idk this is probably wasteful? We could probably have an allocator to size this appropriately to the usage.
pub const MAX_CHUNKS: usize = 16_384;

/// Pool of chunks of varying LODs that we upload to the GPU.
// #[derive(Resource)]
// pub struct ChunkPool {
//     pool: Vec<[u32; CHUNK_LEN]>,
// }

#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, ShaderType)]
#[repr(C)]
pub struct ChunkDescriptor {
    world_position: IVec3,
    lod: u32,
    dirty: u32,
    // 3x3x3 neighborhood, self at slot 13. u32::MAX == absent/air.
    neighbors: [u32; 27],
}
const _: () = assert!(std::mem::size_of::<ChunkDescriptor>() == 128);

pub fn voxel_pool_layout() -> BindGroupLayoutDescriptor {
    BindGroupLayoutDescriptor::new(
        "voxel_pool",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                // voxel pool
                storage_buffer_read_only::<Vec<u32>>(false),
                // chunk descriptors
                storage_buffer_read_only::<Vec<ChunkDescriptor>>(false),
            ),
        ),
    )
}

#[derive(Resource)]
pub struct SurfaceNetPipeline {
    surface_net_layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
}
pub fn init_surface_net_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pipeline_cache: Res<PipelineCache>,
) {
    let surface_net_layout = BindGroupLayoutDescriptor::new(
        "surface_net_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                // target voxel
                uniform_buffer::<u32>(false),
                // jobs
                storage_buffer_read_only::<Vec<u32>>(false),
                // centroids
                storage_buffer::<Vec<u32>>(false),
            ),
        ),
    );

    let shader = asset_server.load(SURFACE_NET_SHADER);
    let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("Mesh generation compute shader".into()),
        layout: vec![voxel_pool_layout(), surface_net_layout.clone()],
        shader: shader.clone(),
        entry_point: Some("compute_centroids".into()),
        ..default()
    });

    commands.insert_resource(SurfaceNetPipeline {
        surface_net_layout,
        pipeline,
    });
}

pub fn prepare_voxels(mut commands: Commands, pipeline: Res<SurfaceNetPipeline>) {}
pub fn compute_mesh() {}
