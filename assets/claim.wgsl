
#import voxel::common;

@group(1) @binding(0) var<storage, read> jobs: array<u32>;
@group(1) @binding(1) var<storage, read_write> claims: array<u32>;

@compute @workgroup_size(8, 8, 4)
fn compute_claims(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // 2d job linearization into the global id
    let cell_y = global_id.y & 15u;   // low 4 bits -> cell.y (0..15)
    let job_hi = global_id.y >> 4u;   // high bits  -> job row
    let cell_z = global_id.z & 15u;   // low 4 bits -> cell.z (0..15)
    let job_lo = global_id.z >> 4u;   // high bits  -> job column
    let job = job_hi * 64 + job_lo;

    return;
}
