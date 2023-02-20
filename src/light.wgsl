// This is a debug shader to view light objects.

struct Camera {
	view_position: vec3<f32>,
	view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Light {
	position: vec3<f32>,
	colour: vec3<f32>,
}
@group(1) @binding(0)
var<uniform> light: Light;

struct VertexInput {
	@location(0) position: vec3<f32>,
};

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) colour: vec3<f32>,
};

@vertex
fn vs_main(
	model: VertexInput,
) -> VertexOutput {
	let scale = 0.25;
	var out: VertexOutput;
	out.clip_position = camera.view_proj * vec4<f32>(model.position * scale + light.position, 1.0);
	out.colour = light.colour;
	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return vec4<f32>(in.colour, 1.0);
}
