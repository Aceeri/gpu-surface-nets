
// 16 * 16 * 16
const CHUNK_DIM: u32 = 16u;
const CHUNK_ARR: u32 = 4096u;
const AIR: u32 = 0u;
const ABSENT: u32 = 0xFFFFFFFFu;

struct ChunkDescriptor {
    // could probably bitpack these if it seems worthwhile but meh
    world_position: vec3<i32>,
    lod: u32,
    dirty: u32,
    // 3x3x3 neighborhood, self lives at slot 13. ABSENT == air.
    neighbors: array<u32, 27>,
}

// pool group
@group(0) @binding(0) var<storage, read> pool: array<u32>;
@group(0) @binding(1) var<storage, read> chunks: array<ChunkDescriptor>;

// surface net group
@group(1) @binding(0) var<uniform> target_voxel: u32;
@group(1) @binding(1) var<storage, read> jobs: array<u32>;
@group(1) @binding(2) var<storage, read_write> centroids: array<u32>;

fn neighbor_slot(offset: vec3<i32>) -> u32 {
    let t = vec3<u32>(offset + vec3<i32>(1)); // -1..1 -> 0..2
    // XZY ordering for 3x3x3 array of neighbors
    return t.x * 9u + t.z * 3u + t.y; // 0..26
}

fn linearize(pos: vec3<u32>) -> u32 {
    // XZY ordering
    return pos.x * 256u + pos.z * 16u + pos.y;
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

@compute @workgroup_size(4, 4, 4)
fn compute_centroids(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // maybe should just be like x == 0..4096, y = job index? idk man
    let job = global_id.z / CHUNK_DIM;
    let desc = chunks[jobs[job]];
    let cell = vec3<u32>(global_id.x, global_id.y, global_id.z % CHUNK_DIM);

    var centroid: u32 = 0u;
    for (var i = 0u; i < 8u; i++) {
        let offset = vec3<u32>(i & 1u, (i >> 1u) & 1u, (i >> 2u) & 1u);
        let voxel = sample_world(desc, cell + offset);
        centroid |= u32(voxel == target_voxel) << i;
    }

    centroids[job * CHUNK_ARR + linearize(cell)] = centroid;
}
