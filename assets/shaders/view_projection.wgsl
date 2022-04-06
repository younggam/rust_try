// Vertex shader

struct ViewProjection {
    matrix: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> view_proj: ViewProjection;

struct VertexInput {
    [[location(0)]] position: vec4<f32>;
    [[location(1)]] color: vec4<f32>;
};

struct InstanceInput {
    [[location(2)]] transform_matrix_0: vec4<f32>;
    [[location(3)]] transform_matrix_1: vec4<f32>;
    [[location(4)]] transform_matrix_2: vec4<f32>;
    [[location(5)]] transform_matrix_3: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let transform_matrix = mat4x4<f32>(
        instance.transform_matrix_0,
        instance.transform_matrix_1,
        instance.transform_matrix_2,
        instance.transform_matrix_3,
    );

    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = view_proj.matrix * transform_matrix * model.position;
    return out;
}


// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}
