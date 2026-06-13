
#import voxel::common;

@group(1) @binding(0) var<storage, read> jobs: array<u32>;
@group(1) @binding(1) var<storage, read_write> claims: array<u32>;

@compute @workgroup_size(8, 8, 4)
fn compute_claims(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // 2d job linearization into the global id
    let cell_y = gid.y & 15u;   // low 4 bits  -> cell.y (0..15)
    let job_hi = gid.y >> 4u;   // high bits   -> job row
    let cell_z = gid.z & 15u;   // low 4 bits  -> cell.z (0..15)
    let job_lo = gid.z >> 4u;   // high bits   -> job col
    let job = job_hi * Z_JOBS + job_lo;
}
