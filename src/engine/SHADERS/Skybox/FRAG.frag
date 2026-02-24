#version 100
precision highp float;

varying vec3 v_dir;
uniform sampler2D Texture;

varying lowp vec4 v_color;
varying vec2 v_texcoord; // Note: probably not needed.

void main() {
    vec3 dir = normalize(v_dir);
    
    // Standard Equirectangular mapping
    vec2 uv = vec2(atan(dir.z, dir.x), asin(clamp(dir.y, -1.0, 1.0)));
    
    uv *= vec2(0.1591549, 0.3183098); // Inverse 2PI and PI
    uv += vec2(0.5, 0.5);

    vec4 texColor = texture2D(Texture, uv);
    
    vec3 final_rgb = texColor.rgb * v_color.rgb;

    gl_FragColor = vec4(final_rgb, texColor.a * v_color.a);
}