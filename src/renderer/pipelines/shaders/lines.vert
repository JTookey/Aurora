#version 450

layout(location = 0) out vec4 f_col;

layout(set = 0, binding = 0) uniform Locals {
    vec2 screen_size;
};

struct Line {
    vec2 position_1;        // 8
    vec2 position_2;        // 8
    vec4 line_colour;       // 16
    float line_width;       // 4
};

layout(std430, set = 0, binding = 1) 
buffer Instances {
    Line lines[];
};

void main() {

    // Pass straight through
    f_col = lines[gl_InstanceIndex].line_colour;

    // Create position default (just in case)
    vec4 pos = vec4(0.0, 0.0, 0.0, 1.0);

    // calc postions in Vulcan canvas space -1 to 1
    vec2 pos1 = (2.0 * lines[gl_InstanceIndex].position_1 / screen_size) - 1.0;
    vec2 pos2 = (2.0 * lines[gl_InstanceIndex].position_2 / screen_size) - 1.0;

    // Calc single pixel vec
    vec2 pixel = vec2(2.0)/screen_size;

    // Line unit vector and perpendicular unit vector
    vec2 line_vec = normalize(pos2 - pos1);
    
    // Work out the positions & uvs of the all corners
    //
    //    TL          TR
    //    0-----------0
    //    BL          BR
    //
    if (gl_VertexIndex == 0) { // BL
        // CW Perpendicular * Pixel Size * Line Width / 2.0
        vec2 offset = vec2(line_vec.y, -line_vec.x) * pixel * lines[gl_InstanceIndex].line_width / 2.0;
        // Point 1
        pos.x = pos1.x + offset.x;
        pos.y = pos1.y + offset.y;

    } else if (gl_VertexIndex == 1) { // TL
        // CCW Perpendicular * Pixel Size * Line Width / 2.0
        vec2 offset = vec2(-line_vec.y, line_vec.x) * pixel * lines[gl_InstanceIndex].line_width / 2.0;
        // Point 1
        pos.x = pos1.x + offset.x;
        pos.y = pos1.y + offset.y;

    } else if (gl_VertexIndex == 2) { // BR
        // CW Perpendicular * Pixel Size * Line Width / 2.0
        vec2 offset = vec2(line_vec.y, -line_vec.x) * pixel * lines[gl_InstanceIndex].line_width / 2.0;
        // Point 2
        pos.x = pos2.x + offset.x;
        pos.y = pos2.y + offset.y;

    } else if (gl_VertexIndex == 3) { // TR
        // CCW Perpendicular * Pixel Size * Line Width / 2.0
        vec2 offset = vec2(-line_vec.y, line_vec.x) * pixel * lines[gl_InstanceIndex].line_width / 2.0;
        // Point 2
        pos.x = pos2.x + offset.x;
        pos.y = pos2.y + offset.y;
    } 

    // Pass the position through
    gl_Position = pos;
}
