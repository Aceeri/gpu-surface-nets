
// TODO: How can I bind all 27 chunks related to this one?
// Probably should have 1 big allocated portion with index, lod level, etc.
// Higher LODs need to sample the voxels from teh high fidelity ones if
// they are in the simulation
@group(0) @binding(0) var<uniform> target_voxel: u32;
@group(0) @binding(1) var voxels: texture_storage_3d<r32uint, read>;
@group(0) @binding(2) var<storage, read_write> centroids: array<u32>;

@compute @workgroup_size(1)
fn compute_centroids(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var centroid: u32 = 0u;
    var bit_index: u32 = 0u;
    for (var x = 0u; x <= 1u; x++) {
        for (var y = 0u; y <= 1u; y++) {
            for (var z = 0u; z <= 1u; z++) {
                let coord = global_id + vec3<u32>(x, y, z);
                let sample = textureLoad(voxels, coord).r;
                centroid |= u32(sample == target_voxel) << bit_index;
                bit_index += 1u;
            }
        }
    }

    let index = global_id.y * 16u * 16u + global_id.x * 16u + global_id.z;
    centroids[index] = centroid;
}
