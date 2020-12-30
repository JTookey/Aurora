#version 450

layout(location = 0) out vec2 f_uv;
layout(location = 1) out float f_aspect_ratio;
layout(location = 2) out uint f_prim_type;
layout(location = 3) out float f_corner_radius;
layout(location = 4) out vec4 f_col;

layout(set = 0, binding = 0) uniform Locals {
    vec2 screen_size;
};

struct Primative {
    vec2 position;          // 8
    vec2 size;              // 8
    vec4 colour;            // 16
    vec4 line_colour;       // 16
    float line_width;       // 4
    float corner_radius;    // 4
    float rotation;         // 4
    uint prim_type;         // 4
};

layout(std430, set = 0, binding = 1) 
buffer Instances {
    Primative prims[];
};

void main() {

    // Pass straight through
    f_prim_type = prims[gl_InstanceIndex].prim_type;
    f_aspect_ratio = prims[gl_InstanceIndex].size.x / prims[gl_InstanceIndex].size.y;
    f_corner_radius = 0.5;
    f_col = prims[gl_InstanceIndex].colour;

    // Set a defaults (just in case)
    f_uv = vec2(0.0);
    vec4 pos = vec4(0.0, 0.0, 0.0, 1.0);

    // Work out the positions & uvs of the all corners
    if (gl_VertexIndex == 0) {
        pos.x = (2.0 * prims[gl_InstanceIndex].position.x / screen_size.x) - 1.0;
        pos.y = (2.0 * prims[gl_InstanceIndex].position.y / screen_size.y) - 1.0;
        f_uv = vec2(-1.0 , 1.0);

    } else if (gl_VertexIndex == 1) {
        pos.x = (2.0 * prims[gl_InstanceIndex].position.x / screen_size.x) - 1.0;
        pos.y = (2.0 * (prims[gl_InstanceIndex].position.y + prims[gl_InstanceIndex].size.y) / screen_size.y) - 1.0;
        f_uv = vec2(-1.0 , -1.0);

    } else if (gl_VertexIndex == 2) {
        pos.x = (2.0 * (prims[gl_InstanceIndex].position.x + prims[gl_InstanceIndex].size.x)/ screen_size.x) - 1.0;
        pos.y = (2.0 * prims[gl_InstanceIndex].position.y / screen_size.y) - 1.0;
        f_uv = vec2(1.0 , 1.0);

    } else if (gl_VertexIndex == 3) {
        pos.x = (2.0 * (prims[gl_InstanceIndex].position.x + prims[gl_InstanceIndex].size.x) / screen_size.x) - 1.0;
        pos.y = (2.0 * (prims[gl_InstanceIndex].position.y + prims[gl_InstanceIndex].size.y) / screen_size.y) - 1.0;
        f_uv = vec2(1.0 , -1.0);

    } 

    // Pass the position through
    gl_Position = pos;
}
