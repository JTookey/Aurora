struct VertexOutput {
    [[builtin(position)]]   pos: vec4<f32>;
    [[location(0)]]         uv: vec2<f32>;
    [[location(1)]]         aspect_ratio: f32;
    [[location(2)]]         shape: u32;
    [[location(3)]]         corner_radius: f32;
    [[location(4)]]         col: vec4<f32>;
    [[location(5)]]         tex_coord: vec2<f32>;
    [[location(6)]]         tex_opacity: f32;
};

struct Locals {
    screen_size: vec2<f32>;
};
[[group(0), binding(0)]]
var<uniform> locals: Locals;

fn rotate_point(cx: f32, cy: f32, angle: f32, p: vec2<f32>) -> vec2<f32>
{

  var new_p: vec2<f32> = p;

  let s: f32 = sin(angle);
  let c: f32 = cos(angle);

  // translate point back to origin:
  new_p.x = p.x - cx;
  new_p.y = p.y - cy;

  // rotate point
  let xnew: f32 = new_p.x * c - new_p.y * s;
  let ynew: f32 = new_p.x * s + new_p.y * c;

  // translate point back:
  new_p.x = xnew + cx;
  new_p.y = ynew + cy;

  return p;
}

[[stage(vertex)]]
fn vs_main (
    [[builtin(vertex_index)]]   index: u32,
    [[location(4)]]             position: vec2<f32>,
    [[location(5)]]             size: vec2<f32>,
    [[location(6)]]             colour: vec4<f32>,
    [[location(7)]]             texture_coords: vec4<f32>,
    [[location(8)]]             texture_opacity: f32,
    [[location(9)]]             line_width: f32,
    [[location(10)]]            corner_radius: f32,
    [[location(11)]]            rotation: f32,
    [[location(12)]]            shape: u32,
) -> VertexOutput {
    var out: VertexOutput;

    // Pass straight through
    out.shape = shape;
    out.aspect_ratio = size.x / size.y;
    out.corner_radius = corner_radius;
    out.col = colour;
    out.tex_opacity = texture_opacity;

    // Set a defaults (just in case)
    out.uv  = vec2<f32>(0.0);
    out.pos = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    // find size
    let midpoint = vec2<f32>( position.x + size.x / 2.0 , position.y + size.y / 2.0 );

    // Work out the positions & uvs of the all corners
    if (index == u32(0)) {
        var point = vec2<f32>( position.x, position.y );
        point = rotate_point(midpoint.x, midpoint.y, rotation, point);

        out.pos.x = (2.0 * point.x / locals.screen_size.x) - 1.0;
        out.pos.y = 1.0 - (2.0 * point.y / locals.screen_size.y);

        out.uv = vec2<f32>(-1.0 , 1.0);
        out.tex_coord = texture_coords.xy;

    } else if (index == u32(1)) {
        var point = vec2<f32>( position.x, position.y + size.y );
        point = rotate_point(midpoint.x, midpoint.y, rotation, point);

        out.pos.x = (2.0 * point.x / locals.screen_size.x) - 1.0;
        out.pos.y = 1.0 - (2.0 * point.y / locals.screen_size.y);

        out.uv = vec2<f32>(-1.0 , -1.0);
        out.tex_coord = texture_coords.xw;

    } else if (index == u32(2)) {
        var point = vec2<f32>( position.x + size.x, position.y );
        point = rotate_point(midpoint.x, midpoint.y, rotation, point);

        out.pos.x = (2.0 * point.x / locals.screen_size.x) - 1.0;
        out.pos.y = 1.0 - (2.0 * point.y / locals.screen_size.y);

        out.uv = vec2<f32>(1.0 , 1.0);
        out.tex_coord = texture_coords.zy;

    } else if (index == u32(3)) {
        var point = vec2<f32>( position.x + size.x, position.y + size.y);
        point = rotate_point(midpoint.x, midpoint.y, rotation, point);

        out.pos.x = (2.0 * point.x / locals.screen_size.x) - 1.0;
        out.pos.y = 1.0 - (2.0 * point.y / locals.screen_size.y);

        out.uv = vec2<f32>(1.0 , -1.0);
        out.tex_coord = texture_coords.zw;
    }

    return out;
}


[[group(0), binding(1)]]
var t_Color: texture_2d<f32>;

[[group(0), binding(2)]]
var s_Color: sampler;


fn udRoundRect(p: vec2<f32>, r: f32) -> f32 {
  	return length(max(abs(p)-vec2<f32>(1.0 - r),vec2<f32>(0.0)))-r;
}
fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
	return length(p) - r;
}
fn sdTri(p: vec2<f32>, h: f32) -> f32 {
	let q: vec2<f32> = abs(p);
    return max(q.x*0.866025+p.y*0.5,-p.y)-h*0.5;
}
fn sdHex(p: vec2<f32>, h: vec2<f32>) -> f32 {
    let q: vec2<f32> = abs(p);
    return max(q.x*0.866025+q.y*0.5,q.y)-h.x;
}
fn asFilled(d: f32) -> f32 {
	return (1. - smoothStep(0.,.01,d));
} 
fn asLine(d: f32, thickness: f32) -> f32 {
	return (1. - smoothStep(0.,thickness,abs(d)));
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    
    // Set a default value for the distance function
    var d: f32 = 1.0;
    
    // Calc for different prim types
    if (in.shape == u32(1)) {
        d = udRoundRect(in.uv, in.corner_radius);
        d = asFilled(d);
    } else if (in.shape == u32(2)) {
        d = sdCircle(in.uv, 1.0);
        d = asFilled(d);
    } else if (in.shape == u32(3)) {
        d = sdTri(in.uv, 1.0);
        d = asFilled(d);
    } else if (in.shape == u32(4)) {
        d = sdHex(in.uv, vec2<f32>(1.0));
        d = asFilled(d);
    }

    var tex = textureSample(t_Color, s_Color, in.tex_coord);

    // Transparency
    if (d<0.01) {
        return tex;
    }

    // Texture
    return mix(tex * d, in.col * d, in.col.a );
}