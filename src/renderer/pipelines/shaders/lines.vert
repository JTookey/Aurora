#version 450

layout(location = 0) out vec4 f_col;

layout(set = 0, binding = 0) uniform Locals {
    vec2 screen_size;
};

layout(location = 4) in vec2 position_1;
layout(location = 5) in vec2 position_2;
layout(location = 6) in vec4 line_colour;
layout(location = 7) in float line_width;

void main() {

    // Pass straight through
    f_col = line_colour;

    // Create position default (just in case)
    vec4 pos = vec4(0.0, 0.0, 0.0, 1.0);

    // calc postions in Vulcan canvas space 1 to -1
    vec2 pos1 = vec2((2.0 * position_1.x / screen_size.x) - 1.0, 1.0 - (2.0 * position_1.y / screen_size.y));

    vec2 pos2 = vec2((2.0 * position_2.x / screen_size.x) - 1.0, 1.0 - (2.0 * position_2.y / screen_size.y));

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
        vec2 offset = vec2(-line_vec.y, line_vec.x) * pixel * line_width / 2.0;
        // Point 1
        pos.x = pos1.x + offset.x;
        pos.y = pos1.y + offset.y;

    } else if (gl_VertexIndex == 1) { // TL
        // CCW Perpendicular * Pixel Size * Line Width / 2.0
        vec2 offset = vec2(line_vec.y, -line_vec.x) * pixel * line_width / 2.0;
        // Point 1
        pos.x = pos1.x + offset.x;
        pos.y = pos1.y + offset.y;

    } else if (gl_VertexIndex == 2) { // BR
        // CW Perpendicular * Pixel Size * Line Width / 2.0
        vec2 offset = vec2(-line_vec.y, line_vec.x) * pixel * line_width / 2.0;
        // Point 2
        pos.x = pos2.x + offset.x;
        pos.y = pos2.y + offset.y;

    } else if (gl_VertexIndex == 3) { // TR
        // CCW Perpendicular * Pixel Size * Line Width / 2.0
        vec2 offset = vec2(line_vec.y, -line_vec.x) * pixel * line_width / 2.0;
        // Point 2
        pos.x = pos2.x + offset.x;
        pos.y = pos2.y + offset.y;
    } 

    // Pass the position through
    gl_Position = pos;
}
