#version 100
precision highp float;

#ifdef GL_OES_standard_derivatives
#extension GL_OES_standard_derivatives : enable
#endif

varying lowp vec4 v_color;
varying highp vec3 v_world_pos;
varying mediump vec3 v_normal;
varying vec2 v_texcoord;

uniform vec3 LightDir;
uniform sampler2D Texture;

void main() {
    vec3 N = v_normal;

    if (length(N) < 1e-5) {
        #ifdef GL_OES_standard_derivatives
            vec3 dx = dFdx(v_world_pos);
            vec3 dy = dFdy(v_world_pos);
            N = normalize(cross(dx, dy));
        #else
            N = vec3(0.0, 1.0, 0.0);
        #endif
    }

    vec3 L = normalize(LightDir);
    float ndotl = dot(N, L) * 0.5 + 0.5;
    float brightness = 0.3 + 0.5 * ndotl;

    vec4 texColor = texture2D(Texture, v_texcoord);

    vec3 final_rgb = v_color.rgb * texColor.rgb * brightness;
    
    gl_FragColor = vec4(final_rgb, v_color.a * texColor.a);
}