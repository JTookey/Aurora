struct VertexOutput {
    [[builtin(position)]]   pos: vec4<f32>;
    [[location(0)]] col: vec4<f32>;
};

struct Locals {
    screen_size: vec2<f32>;
};
[[group(0), binding(0)]]
var<uniform> locals: Locals;

[[stage(vertex)]]
fn vs_main (
    [[builtin(vertex_index)]]   index: u32,
    [[location(4)]] position_1: vec2<f32>,
    [[location(5)]] position_2: vec2<f32>,
    [[location(6)]] line_colour: vec4<f32>,
    [[location(7)]] line_width: f32,
) -> VertexOutput {

    var out: VertexOutput;

    // Pass straight through
    out.col = line_colour;

    // Create position default (just in case)
    out.pos = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    // calc postions in Vulcan canvas space 1 to -1
    let pos1 = vec2<f32>((2.0 * position_1.x / locals.screen_size.x) - 1.0, 1.0 - (2.0 * position_1.y / locals.screen_size.y));

    let pos2 = vec2<f32>((2.0 * position_2.x / locals.screen_size.x) - 1.0, 1.0 - (2.0 * position_2.y / locals.screen_size.y));

    // Calc single pixel vec
    let pixel = vec2<f32>(2.0)/locals.screen_size;

    // Line unit vector and perpendicular unit vector
    let line_vec = normalize(pos2 - pos1);
    
    // Work out the positions & uvs of the all corners
    //
    //    TL          TR
    //    0-----------0
    //    BL          BR
    //
    if (index == u32(0)) { // BL
        // CW Perpendicular * Pixel Size * Line Width / 2.0
        let offset = vec2<f32>(-line_vec.y, line_vec.x) * pixel * line_width / 2.0;
        // Point 1
        out.pos.x = pos1.x + offset.x;
        out.pos.y = pos1.y + offset.y;

    } else if (index == u32(1)) { // TL
        // CCW Perpendicular * Pixel Size * Line Width / 2.0
        let offset = vec2<f32>(line_vec.y, -line_vec.x) * pixel * line_width / 2.0;
        // Point 1
        out.pos.x = pos1.x + offset.x;
        out.pos.y = pos1.y + offset.y;

    } else if (index == u32(2)) { // BR
        // CW Perpendicular * Pixel Size * Line Width / 2.0
        let offset = vec2<f32>(-line_vec.y, line_vec.x) * pixel * line_width / 2.0;
        // Point 2
        out.pos.x = pos2.x + offset.x;
        out.pos.y = pos2.y + offset.y;

    } else if (index == u32(3)) { // TR
        // CCW Perpendicular * Pixel Size * Line Width / 2.0
        let offset = vec2<f32>(line_vec.y, -line_vec.x) * pixel * line_width / 2.0;
        // Point 2
        out.pos.x = pos2.x + offset.x;
        out.pos.y = pos2.y + offset.y;
    } 

    return out;
}



[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.col;
}