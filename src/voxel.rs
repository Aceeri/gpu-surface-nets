use bevy::{
    material::descriptor::BindGroupLayoutDescriptor,
    platform::collections::HashMap,
    prelude::*,
    render::render_resource::{
        BindGroupLayoutEntries, ShaderStages, ShaderType, binding_types::storage_buffer_read_only,
    },
};

use crate::simulation::ClaimJob;

pub const CHUNK_DIM: usize = 16;
pub const CHUNK_LEN: usize = CHUNK_DIM.pow(3);
pub const SLAB_BYTES: usize = CHUNK_LEN * 4;
pub const ABSENT: u32 = u32::MAX;

// idk this is probably wasteful? We could probably have an allocator to size this appropriately to the usage.
pub const MAX_CHUNKS: usize = 16_384;

/// Pool of chunks of varying LODs that we upload to the GPU.
#[derive(Resource)]
pub struct ChunkPool {
    // GPU buffers
    voxels: Vec<u32>,
    descriptors: Vec<ChunkDescriptor>,
    free_slots: Vec<u32>,

    // simulation
    claim_jobs: Vec<ClaimJob>,
    claims: Vec<u32>, // double buffer for claims

    // TODO:
    // requested chunks by the GPU, should be added either by dilating chunks or dirty areas of chunks.
    need: Vec<IVec3>,
    // chunks to remove as idle
    // nice to have for now, necessary later to clean up VRAM
    remove: Vec<u32>,

    // CPU-side management
    chunk_map: HashMap<IVec3, u32>,
}

impl ChunkPool {
    pub fn new() -> Self {
        Self {
            voxels: Vec::new(),
            descriptors: Vec::new(),
            free_slots: Vec::new(),

            claim_jobs: Vec::new(),
            claims: Vec::new(),

            need: Vec::new(),
            remove: Vec::new(),

            chunk_map: HashMap::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk_position: IVec3, chunk: &[u32; CHUNK_LEN], lod: u32) {
        if self.chunk_map.contains_key(&chunk_position) {
            panic!(
                "Trying to add chunk that already exists in buffers, GPU is authoritative the moment it is pushed here."
            );
        }

        let mut descriptor = ChunkDescriptor {
            world_position: chunk_position,
            lod: lod,
            _pad: 0u32,
            neighbors: [ABSENT; 27],
        };

        let slot = if let Some(free_slot) = self.free_slots.pop() {
            let start = free_slot as usize * CHUNK_LEN;
            self.voxels[start..start + CHUNK_LEN].copy_from_slice(chunk);

            // simulation buffers
            self.claims[start..start + CHUNK_LEN].fill(0u32); // zero out
            free_slot
        } else {
            let slot = self.voxels.len() / CHUNK_LEN;
            self.voxels.extend(chunk);
            self.descriptors.push(descriptor.clone());

            // simulation buffers
            self.claims.extend([0u32; CHUNK_LEN]);
            slot as u32
        };

        // TODO: do we need to add claim jobs for its neighbor sections that border this one?
        self.claim_jobs.extend(ClaimJob::all_sections(slot));
        self.chunk_map.insert(chunk_position, slot);

        for (index, offset) in ChunkDescriptor::neighbor_iter() {
            let world_neighbor = chunk_position + offset;
            if let Some(neighbor_slot) = self.chunk_map.get(&world_neighbor) {
                descriptor.neighbors[index] = *neighbor_slot;
                let inverse_index = ChunkDescriptor::linearize_offset(-offset);
                self.descriptors[*neighbor_slot as usize].neighbors[inverse_index] = slot;
            }
        }

        self.descriptors[slot as usize] = descriptor;
    }

    pub fn remove_chunk(&mut self, chunk_position: IVec3) {
        let Some(slot) = self.chunk_map.remove(&chunk_position) else {
            panic!("Tried to remove chunk that does not currently exist in the chunk pool");
        };

        self.free_slots.push(slot);

        // free neighbor references by setting to u32::MAX/ABSENT
        let neighbors = self.descriptors[slot as usize].neighbors.clone();
        for (index, neighbor_slot) in neighbors.iter().enumerate() {
            if *neighbor_slot == ABSENT {
                continue;
            }

            self.descriptors[*neighbor_slot as usize].neighbors
                [ChunkDescriptor::inverse_index(index)] = ABSENT;
        }

        // Should we garble the ChunkDescriptor or make it obviously dead/freed?
    }
}

#[derive(Copy, Clone, ShaderType)]
pub struct ChunkDescriptor {
    world_position: IVec3,
    lod: u32,
    _pad: u32, // could use this as a bitset? denote idle or some other properties?
    // 3x3x3 neighborhood, self at slot 13. u32::MAX == absent/air. XYZ ordering
    neighbors: [u32; 27],
}
const _: () = assert!(std::mem::size_of::<ChunkDescriptor>() == 128);

impl ChunkDescriptor {
    pub fn neighbor_iter() -> impl Iterator<Item = (usize, IVec3)> {
        (-1..=1)
            .flat_map(move |y| {
                (-1..=1).flat_map(move |x| (-1..=1).map(move |z| IVec3::new(x, y, z)))
            })
            .enumerate()
    }

    #[inline]
    fn linearize_offset(offset: IVec3) -> usize {
        ((offset.z + 1) + (offset.x + 1) * 3 + (offset.y + 1) * 9) as usize
    }

    pub fn neighbor(&self, offset: IVec3) -> u32 {
        self.neighbors[Self::linearize_offset(offset)]
    }

    #[inline]
    pub fn inverse_index(index: usize) -> usize {
        26 - index
    }
}

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
