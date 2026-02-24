#version 100
attribute vec3 position;

varying highp vec3 v_dir;

// We pass a combined matrix that includes Projection * Rotation(View)
// We do NOT use the standard Model matrix or full View matrix.
uniform mat4 SkyViewProj;

void main() {
    // 1. Direction:
    // Since we draw the cube at (0,0,0), the vertex position IS the direction.
    v_dir = position;
    
    // 2. Position:
    // Apply our custom Rotation+Projection matrix
    vec4 pos = SkyViewProj * vec4(position, 1.0);
    
    // 3. Infinite Depth Trick:
    // Set z = w so that z/w = 1.0 (The far plane).
    gl_Position = pos.xyww;
}