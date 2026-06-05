
use bevy::prelude::*;


#[derive(Default)]
pub struct GpuSurfaceNets;

impl Plugin for GpuSurfaceNets {
    fn build(&self, app: &mut App) {
        // alright we need buffers to read from? how do we dispatch and stuff
        app.add_systems(Startup, (setup_voxel_buffer);
        app.add_systems(PostUpdate, (push_voxel_buffer, dispatch_surface_nets).chain());
    }
}

pub const WIDTH: usize = 18;
pub const SAMPLES: usize = WIDTH*WIDTH*WIDTH;

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

pub fn setup_voxel_buffer(render_device: Res<RenderDevice>) {
    // TODO
}

pub fn push_voxel_buffer(render_device: Res<RenderDevice>) {
    // TODO
}

pub fn dispatch_surface_nets(render_device: Res<RenderDevice>) {
    // TODO
}
