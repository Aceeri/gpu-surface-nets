
#define_import voxel::common;

// 16 * 16 * 16
const CHUNK_DIM: u32 = 16u;
const CHUNK_ARR: u32 = 4096u;
const AIR: u32 = 0u;
const ABSENT: u32 = 0xFFFFFFFFu;

struct ChunkDescriptor {
    // could probably bitpack these if it seems worthwhile but meh
    world_position: vec3<i32>,
    lod: u32,
    _pad: u32,
    // 3x3x3 neighborhood, self lives at slot 13. ABSENT == air.
    neighbors: array<u32, 27>,
}

// pool group
@group(0) @binding(0) var<storage, read> pool: array<u32>;
@group(0) @binding(1) var<storage, read> chunks: array<ChunkDescriptor>;


fn neighbor_slot(offset: vec3<i32>) -> u32 {
    let t = vec3<u32>(offset + vec3<i32>(1)); // -1..1 -> 0..2
    // YXZ ordering for 3x3x3 array of neighbors
    return t.y * 9u + t.x * 3u + t.z; // 0..26
}

fn linearize(pos: vec3<u32>) -> u32 {
    // YXZ ordering
    return pos.y * 256u + pos.x * 16u + pos.z;
}

fn sample(slab: u32, pos: vec3<u32>) -> u32 {
    return pool[slab * CHUNK_ARR + linearize(pos)];
}

fn sample_world(desc: ChunkDescriptor, pos: vec3<u32>) -> u32 {
    // 0..15 -> this chunk, 16 -> the +1 neighbor at local 0.
    let offset = vec3<i32>(pos >> vec3<u32>(4u)); // per-axis 0 or 1
    let neighbor = desc.neighbors[neighbor_slot(offset)];
    if (neighbor == ABSENT) { return AIR; }
    return sample(neighbor, pos & vec3<u32>(15u)); // 16 -> 0
    // Self sits at slot 13, so the local chunk needs no special case.
}
