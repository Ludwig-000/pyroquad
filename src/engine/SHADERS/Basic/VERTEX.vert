#version 100
precision highp float;

// The browser compiles this as GLSL ES 3.00 (see shaderLoader): map the 1.00 keywords.
#if __VERSION__ >= 300
#define attribute in
#define varying out
#endif
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
attribute vec4 normal;

uniform mat4 Model;
uniform mat4 Projection;

varying lowp vec4 v_color;
varying highp vec3 v_world_pos;
varying mediump vec3 v_normal;
varying vec2 v_texcoord;

void main() {
    vec4 world_pos = Model * vec4(position, 1.0);
    v_world_pos = world_pos.xyz;
    v_color = color0 / 255.0;
    
    v_texcoord = texcoord; 

    if (length(normal.xyz) > 1e-6) {
        v_normal = normalize((Model * vec4(normal.xyz, 0.0)).xyz);
    } else {
        v_normal = vec3(0.0);
    }

    gl_Position = Projection * world_pos;
}