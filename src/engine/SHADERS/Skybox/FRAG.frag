#version 100
precision mediump float;
varying vec2 v_uv;

void main() {
    // v_uv is just 0..1 across the screen
    gl_FragColor = vec4(v_uv.x, v_uv.y, 0.5, 1.0);
}