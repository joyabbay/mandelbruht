#import bevy_sprite::mesh2d_vertex_output::VertexOutput
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> view: vec4<f32>;
@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {


    let centre = view.xy;
    let zoom = view.z;
    let MAX_ITER = view.w;

    //how far from centre
    // uv's count of y axis is reverse
    let p = vec2<f32>(mesh.uv.x - 0.5, mesh.uv.y - 0.5);

    let c = centre + 2.0 * p * zoom;

    var z = vec2<f32>(0.0);
    var i = 0u;
    while (f32(i) < MAX_ITER && dot(z, z) <= 4.0) {   // dot(z,z) = |z|²
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        i = i + 1u;
    }

    let t = f32(i) / MAX_ITER;
    return vec4<f32>(vec3<f32>(t), 1.0);
}
