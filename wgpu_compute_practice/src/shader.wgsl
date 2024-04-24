struct DataBuf {
    data: array<f32>,
}

@group(0)
@binding(0)
var<storage, read_write> v_indicies: DataBuf;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // takes v_indicies at global_id of thread and then adds 42 and store it to same spot
    // how can I take from one buf and store in another buffer?
    v_indicies.data[global_id.x] = v_indicies.data[global_id.x] + 42.0;
}