use bevy::{
    core_pipeline::schedule::camera_driver,
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedComputePipelineId, ComputePipelineDescriptor,
    },
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup,
        render_resource::{
            BindGroup, BindGroupLayoutEntries, PipelineCache, ShaderStages, ShaderType,
            StorageTextureAccess, TextureFormat,
            binding_types::{
                storage_buffer, storage_buffer_read_only, texture_storage_3d, uniform_buffer,
            },
        },
        renderer::RenderDevice,
    },
    shader::load_shader_library,
};

use crate::voxel::voxel_pool_layout;

const CLAIM_SHADER: &'static str = "claim.wgsl";
const APPLY_SHADER: &'static str = "apply.wgsl";

#[derive(Default)]
pub struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "../assets/voxel.wgsl");

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            panic!("Missing RenderApp");
        };

        render_app
            .add_systems(RenderStartup, init_simulation_pipeline)
            .add_systems(Render, prepare_bind_groups)
            .add_systems(RenderGraph, dispatch);
    }
}

// Packed u32, sections are 8x8x4/256 voxels so we have 16 sections from a 16x16x16/4096 voxel group.
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, ShaderType)]
#[repr(C)]
pub struct ClaimJob {
    bits: u32,
}

impl ClaimJob {
    pub fn new(chunk: u32, section: u32) -> Self {
        assert!(chunk < 2u32.pow(28));
        assert!(section < 16);
        Self {
            bits: chunk | (section << 28),
        }
    }

    pub fn all_sections(chunk: u32) -> impl Iterator<Item = Self> {
        (0..16u32).map(move |section| ClaimJob::new(chunk, section))
    }

    #[inline]
    pub fn chunk(&self) -> u32 {
        self.bits & 0x0F_FF_FF_FF
    }

    #[inline]
    pub fn section(&self) -> u32 {
        self.bits >> 28
    }
}

#[derive(Resource)]
pub struct SimulationPipeline {
    claim_layout: BindGroupLayoutDescriptor,
    claim_pipeline: CachedComputePipelineId,
    // apply_layout: BindGroupLayoutDescriptor,
    // apply_pipeline: CachedComputePipelineId,
}

pub struct SimulationBindGroups {
    claim_group: BindGroup,
}

pub fn init_simulation_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pipeline_cache: Res<PipelineCache>,
) {
    let claim_layout = BindGroupLayoutDescriptor::new(
        "claim_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                // claim jobs
                storage_buffer_read_only::<Vec<u32>>(false),
                // claim buffer
                storage_buffer::<Vec<u32>>(false),
            ),
        ),
    );

    let apply_layout = BindGroupLayoutDescriptor::new(
        "apply_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                // jobs
                storage_buffer_read_only::<Vec<u32>>(false),
                // claim buffer
                storage_buffer::<Vec<u32>>(false),
            ),
        ),
    );

    let claim_shader = asset_server.load(CLAIM_SHADER);
    let claim_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("claim".into()),
        layout: vec![voxel_pool_layout(), claim_layout.clone()],
        shader: claim_shader.clone(),
        entry_point: Some("compute_claims".into()),
        ..default()
    });

    // let apply_shader = asset_server.load(APPLY_SHADER);
    // let apply_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
    //     label: Some("apply".into()),
    //     layout: vec![voxel_pool_layout(), apply_layout.clone()],
    //     shader: apply_shader.clone(),
    //     entry_point: Some("".into()),
    //     ..default()
    // });

    commands.insert_resource(SimulationPipeline {
        claim_layout,
        claim_pipeline,
        // apply_layout,
        // apply_pipeline,
    });
}

pub fn prepare_bind_groups(
    mut commands: Commands,
    pipeline: Res<SimulationPipeline>,
    render_device: Res<RenderDevice>,
) {
    // render_device.create_bind_group();
}
pub fn dispatch() {}

#[cfg(test)]
mod test {
    use super::ClaimJob;

    #[test]
    fn claim_job_sanity() {
        for chunk in 0..2000 {
            for section in 0..16 {
                let job = ClaimJob::new(chunk, section);
                assert!(job.section() == section);
                assert!(job.chunk() == chunk);
            }
        }
    }
}
