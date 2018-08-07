#version 330 core

#include <noise.glsl>

in vec3 frag_pos;
in vec3 frag_norm;
in vec4 frag_col;

layout (std140)
uniform model_consts {
	mat4 model_mat;
};

layout (std140)
uniform world_consts {
	mat4 view_mat;
	mat4 proj_mat;
	vec4 sky_color;
	vec4 play_origin;
	vec4 view_distance;
	vec4 time;
};

out vec4 target;

void main() {
	target = frag_col;

	// Sunlight
	float light_level = clamp(cos(time.y / 600.0), 0.1, 1);
	float diffuse_factor = 0.9;
	float ambient_factor = 0.1;
	vec3 sun_direction = normalize(vec3(-sin(time.x / 600), 0.0, -cos(time.y / 600)));
	vec3 sun_color = vec3(1.0, 1.0, 1.0) * light_level;
	float sun_specular = 0.5;
	float sun_factor = 5;
	float sun_shine = 0;

	// Geometry
	vec3 world_norm = normalize((model_mat * vec4(frag_norm, 0)).xyz);
	vec3 world_pos = (model_mat * vec4(frag_pos, 1)).xyz;
	vec3 cam_pos = (view_mat * vec4(world_pos, 1)).xyz;
	float play_dist = length(play_origin.xyz - world_pos);

	float mist_start = view_distance.x * 0.8;// + snoise(vec4(world_pos, time) * 0.02) * 50.0;
	float mist_end = view_distance.x;// + snoise(vec4(world_pos, -time) * 0.02) * 50.0;

	// Ambiant light
	vec3 ambient = frag_col.xyz * ambient_factor * sun_color;

	// Diffuse light
	vec3 diffuse = frag_col.xyz * diffuse_factor * sun_color * max(0, dot(world_norm, -normalize(sun_direction)));

	// Specular light
	vec3 reflect_vec = (view_mat * vec4(reflect(sun_direction, world_norm), 0)).xyz;
	float specular_val = clamp(dot(-normalize(cam_pos), reflect_vec) + sun_shine, 0, 1);
	vec3 specular = sun_color * pow(specular_val, sun_factor) * sun_specular;

	// Mist
	float mist_delta = 1 / (mist_end - mist_start);
	float mist_value = clamp(play_dist * mist_delta - mist_delta * mist_start, 0.0, 1.0);

	target = mix(vec4(ambient + diffuse + specular, frag_col.w), vec4(sky_color.xyz, 1), mist_value);
}
