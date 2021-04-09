#version 450

layout(location = 0) in vec2 uv;
layout(location = 1) in float aspect_ratio;

layout(location = 2) in flat uint f_shape;
layout(location = 3) in float f_corner_radius;
layout(location = 4) in vec4 f_col;
layout(location = 5) in vec2 f_tex_coord;
layout(location = 6) in float f_tex_opacity;

layout(set = 0, binding = 1) uniform texture2D t_Color;
layout(set = 0, binding = 2) uniform sampler s_Color;

layout(location = 0) out vec4 o_Target;

float udRoundRect(vec2 p, float r) {
  	return length(max(abs(p)-vec2(1.0 - r),0.0))-r;
}
float sdCircle(vec2 p, float r) {
	return length(p) - r;
}
float sdTri(vec2 p, float h) {
	vec2 q = abs(p);
    return max(q.x*0.866025+p.y*0.5,-p.y)-h*0.5;
}
float sdHex(vec2 p, vec2 h){
    vec2 q = abs(p);
    return max(q.x*0.866025+q.y*0.5,q.y)-h.x;
}
float asFilled(float d) {
	return (1. - smoothstep(0.,.01,d));
} 
float asLine(float d, float thickness) {
	return (1. - smoothstep(0.,thickness,abs(d)));
}

void main() {
    
    // Set a default value for the distance function
    float d = 1.0;
    
    // Calc for different prim types
    if (f_shape == 1) {
        d = udRoundRect(uv, f_corner_radius);
        d = asFilled(d);
    } else if (f_shape == 2) {
        d = sdCircle(uv, 1.0);
        d = asFilled(d);
    } else if (f_shape == 3) {
        d = sdTri(uv, 1.0);
        d = asFilled(d);
    } else if (f_shape == 4) {
        d = sdHex(uv, vec2(1.0));
        d = asFilled(d);
    }

    // Transparency
    if (d<0.01) {
        discard;
    }

    // Texture
    vec4 tex = texture(sampler2D(t_Color, s_Color), f_tex_coord);
    o_Target = mix(tex * d, f_col * d, f_col.a );
}