// The Blinn-Phong reflection model.

// Vertex shader

struct Camera {
	view_pos: vec4<f32>,
	view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: Camera;

struct InstanceInput {
	@location(5) model_matrix_0: vec4<f32>,
	@location(6) model_matrix_1: vec4<f32>,
	@location(7) model_matrix_2: vec4<f32>,
	@location(8) model_matrix_3: vec4<f32>,
	@location(9) normal_matrix_0: vec3<f32>,
	@location(10) normal_matrix_1: vec3<f32>,
	@location(11) normal_matrix_2: vec3<f32>,
}

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
	@location(2) normal: vec3<f32>,
};

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) tex_coords: vec2<f32>,
	@location(1) world_normal: vec3<f32>,
	@location(2) world_position: vec3<f32>,
};

@vertex
fn vs_main(
	model: VertexInput,
	instance: InstanceInput,
) -> VertexOutput {
	let model_matrix = mat4x4<f32>(
		instance.model_matrix_0,
		instance.model_matrix_1,
		instance.model_matrix_2,
		instance.model_matrix_3,
	);
	let normal_matrix = mat3x3<f32>(
		instance.normal_matrix_0,
		instance.normal_matrix_1,
		instance.normal_matrix_2,
	);
	var out: VertexOutput;
	out.tex_coords = model.tex_coords;
	out.world_normal = normal_matrix * model.normal;
	var world_position: vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0);
	out.world_position = world_position.xyz;
	out.clip_position = camera.view_proj * world_position;
	return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

struct Light {
	position: vec3<f32>,
	colour: vec3<f32>,
}
@group(2) @binding(0)
var<uniform> light: Light;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let object_colour: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);

	let light_direction = normalize(light.position - in.world_position);
	let view_direction = normalize(camera.view_pos.xyz - in.world_position);
	let half_direction = normalize(view_direction + light_direction);
	let reflect_direction = reflect(-light_direction, in.world_normal);

	let ambient_strength = 0.1;
	let ambient_colour = light.colour * ambient_strength;

	let diffuse_strength = max(dot(in.world_normal, light_direction), 0.0);
	let diffuse_colour = light.colour * diffuse_strength;

	let specular_strength = pow(max(dot(in.world_normal, half_direction), 0.0), 32.0);
	let specular_colour = specular_strength * light.colour;

	let result = (ambient_colour + diffuse_colour + specular_colour) * object_colour.xyz;

	return vec4<f32>(result, object_colour.a);
}