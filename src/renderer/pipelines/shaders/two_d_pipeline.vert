#version 450

layout(location = 0) out vec2 f_uv;
layout(location = 1) out float f_aspect_ratio;
layout(location = 2) out uint f_shape;
layout(location = 3) out float f_corner_radius;
layout(location = 4) out vec4 f_col;
layout(location = 5) out vec2 f_tex_coord;
layout(location = 6) out float f_tex_opacity;

layout(set = 0, binding = 0) uniform Locals {
    vec2 screen_size;
};

layout(location = 4) in vec2 position;
layout(location = 5) in vec2 size;
layout(location = 6) in vec4 colour;
layout(location = 7) in vec4 texture_coords;
layout(location = 8) in float texture_opacity;
layout(location = 9) in float line_width;
layout(location = 10) in float corner_radius;
layout(location = 11) in float rotation;
layout(location = 12) in uint shape;

void main() {

    // Pass straight through
    f_shape = shape;
    f_aspect_ratio = size.x / size.y;
    f_corner_radius = corner_radius;
    f_col = colour;
    f_tex_opacity = texture_opacity;

    // Set a defaults (just in case)
    f_uv = vec2(0.0);
    vec4 pos = vec4(0.0, 0.0, 0.0, 1.0);

    // Work out the positions & uvs of the all corners
    if (gl_VertexIndex == 0) {
        pos.x = (2.0 * position.x / screen_size.x) - 1.0;
        pos.y = 1.0 - (2.0 * position.y / screen_size.y);
        f_uv = vec2(-1.0 , 1.0);
        f_tex_coord = texture_coords.xy;

    } else if (gl_VertexIndex == 1) {
        pos.x = (2.0 * position.x / screen_size.x) - 1.0;
        pos.y = 1.0 - (2.0 * (position.y + size.y) / screen_size.y);
        f_uv = vec2(-1.0 , -1.0);
        f_tex_coord = texture_coords.xw;

    } else if (gl_VertexIndex == 2) {
        pos.x = (2.0 * (position.x + size.x)/ screen_size.x) - 1.0;
        pos.y = 1.0 - (2.0 * position.y / screen_size.y);
        f_uv = vec2(1.0 , 1.0);
        f_tex_coord = texture_coords.zy;

    } else if (gl_VertexIndex == 3) {
        pos.x = (2.0 * (position.x + size.x) / screen_size.x) - 1.0;
        pos.y = 1.0 - (2.0 * (position.y + size.y) / screen_size.y);
        f_uv = vec2(1.0 , -1.0);
        f_tex_coord = texture_coords.zw;
    } 

    // Pass the position through
    gl_Position = pos;
}
