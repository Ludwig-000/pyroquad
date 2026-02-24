#version 100
precision highp float;

varying vec3 v_dir;
uniform sampler2D Texture;

void main() {
    vec3 dir = normalize(v_dir);
    
    // Standard Equirectangular mapping
    // atan(z, x) is standard for forward-Z, but depending on texture source, 
    // you might need atan(x, z). This is the most common standard:
    vec2 uv = vec2(atan(dir.z, dir.x), asin(clamp(dir.y, -1.0, 1.0)));
    
    uv *= vec2(0.1591549, 0.3183098); // Inverse 2PI and PI
    uv += vec2(0.5, 0.5);
    
    
    gl_FragColor = texture2D(Texture, uv);
}