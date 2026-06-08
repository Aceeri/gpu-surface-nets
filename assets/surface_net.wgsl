
// TODO: How can I bind all 27 chunks related to this one?
// Probably should have 1 big allocated portion with index, lod level, etc.
// Higher LODs need to sample the voxels from teh high fidelity ones if
// they are in the simulation
@group(0) @binding(0) var voxels: texture_storage_3d<r32uint, read>;
@group(0) @binding(1) var<storage, read_write> centroids: array<u32>;

@compute @workgroup_size(1)
fn compute_centroids(@builtin(global_invocation_id) global_id: vec3<u32>) {

}
