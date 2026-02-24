#version 100
attribute vec3 position;
attribute vec4 color0;

varying highp vec3 v_dir;
varying lowp vec4 v_color;

uniform mat4 SkyViewProj;

void main() {
    v_dir = position;

    v_color = color0 / 255.0;
    
    vec4 pos = SkyViewProj * vec4(position, 1.0);
    
    // Infinite Depth Trick:
    gl_Position = pos.xyww;
}