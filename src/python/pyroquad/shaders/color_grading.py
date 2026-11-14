"""
COLOR_GRADING: a general purpose color-grading pass over everything drawn so far.

Use it as `Material.COLOR_GRADING`. It is compiled the first time that is read.

Draw any rectangle with it once you have drawn the scene. The rectangle itself is
invisible: the material stretches it over the whole screen (or the whole render target,
if one is set) and replaces every pixel with its graded version. Only what was drawn
before the rectangle is graded, so a UI drawn after it keeps its colors:

    draw_all_3d_objects()
    draw_texture(...)                          # the scene

    grade = Material.COLOR_GRADING
    grade.set_uniform("saturation", UniformType.Float1([0.3]))
    grade.start_using()
    draw_rectangle()    # grades the whole screen
    use_default_material()

    draw_text(...)                             # the UI, not graded

Where and how large the rectangle is does not matter, nor does the camera. Its color is
ignored too. Use `draw_rectangle`: other shapes don't span the 0..1 texture coordinates
the material builds the full-screen quad from. Each pass copies the screen once.

All uniforms start at a neutral value, so the material changes nothing until one is set.
The steps run in this order, each on the result of the one before:

    uniform       type   neutral          what it does
    exposure      float  0.0              brightness in photographic stops: +1 doubles, -1 halves
    temperature   float  0.0              white balance, -1 cool (blue) .. +1 warm (orange)
    tint          float  0.0              white balance, -1 green .. +1 magenta
    contrast      float  1.0              pushes colors away from (>1) or toward (<1) mid grey
    brightness    float  0.0              added to every channel
    lift          vec3   (0, 0, 0)        raises the shadows per channel, leaves white alone
    gamma         vec3   (1, 1, 1)        bends the midtones per channel: >1 brighter, <1 darker
    gain          vec3   (1, 1, 1)        scales the highlights per channel, leaves black alone
    hue_shift     float  0.0              rotates every hue, in degrees
    saturation    float  1.0              0 is greyscale, >1 is more colorful
    intensity     float  1.0              blends from the original (0) to the graded result (1)

The result is clamped to 0..1. Alpha is left as it was on screen.
"""

from .._pyroquad import PipelineParams, UniformDesc, UniformType  # type: ignore


# Ignores the camera (`Projection`, `Model`) and the rectangle's position: each corner is
# placed on a corner of the screen by its texture coordinate, so any `draw_rectangle` covers
# the whole screen. `screen_uv` is then where that pixel sits in the screen copy.
VERTEX = """#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying mediump vec2 screen_uv;

void main() {
    screen_uv = texcoord;
    gl_Position = vec4(texcoord * 2.0 - 1.0, 0.0, 1.0);
}
"""


# `_ScreenTexture` is a name the engine knows: a material that uses it gets a copy of
# everything drawn so far (screen or render target), taken right before it draws.
FRAGMENT = """#version 100
precision mediump float;

varying vec2 screen_uv;

uniform sampler2D _ScreenTexture;

uniform float exposure;
uniform float temperature;
uniform float tint;
uniform float contrast;
uniform float brightness;
uniform vec3 lift;
uniform vec3 gamma;
uniform vec3 gain;
uniform float hue_shift;
uniform float saturation;
uniform float intensity;

// how bright each channel looks to the eye (Rec. 709)
const vec3 LUMA = vec3(0.2126, 0.7152, 0.0722);

// rotates a color around the grey axis, which turns every hue by the same angle
// and keeps greys grey.
vec3 rotate_hue(vec3 c, float degrees) {
    const vec3 k = vec3(0.57735027);  // (1, 1, 1) normalized
    float a = radians(degrees);
    float cos_a = cos(a);
    return c * cos_a + cross(k, c) * sin(a) + k * dot(k, c) * (1.0 - cos_a);
}

void main() {
    vec4 original = texture2D(_ScreenTexture, screen_uv);
    vec3 c = original.rgb;

    // exposure, in stops
    c *= exp2(exposure);

    // white balance: warm moves red up and blue down, magenta moves green down
    c *= vec3(1.0 + 0.2 * temperature, 1.0 - 0.2 * tint, 1.0 - 0.2 * temperature);

    // contrast around mid grey, then brightness
    c = (c - 0.5) * contrast + 0.5;
    c += brightness;
    c = max(c, 0.0);

    // lift / gamma / gain
    c = gain * (c + lift * (1.0 - c));
    c = pow(max(c, 0.0), 1.0 / max(gamma, vec3(0.0001)));

    // hue, then saturation
    c = rotate_hue(c, hue_shift);
    c = mix(vec3(dot(c, LUMA)), c, saturation);

    c = clamp(c, 0.0, 1.0);
    gl_FragColor = vec4(mix(original.rgb, c, intensity), original.a);
}
"""


UNIFORMS = [
    UniformDesc.Float1("exposure"),
    UniformDesc.Float1("temperature"),
    UniformDesc.Float1("tint"),
    UniformDesc.Float1("contrast"),
    UniformDesc.Float1("brightness"),
    UniformDesc.Float3("lift"),
    UniformDesc.Float3("gamma"),
    UniformDesc.Float3("gain"),
    UniformDesc.Float1("hue_shift"),
    UniformDesc.Float1("saturation"),
    UniformDesc.Float1("intensity"),
]

# the neutral value of every uniform: with these the pass leaves the screen as it is.
DEFAULTS = {
    "exposure":    UniformType.Float1([0.0]),
    "temperature": UniformType.Float1([0.0]),
    "tint":        UniformType.Float1([0.0]),
    "contrast":    UniformType.Float1([1.0]),
    "brightness":  UniformType.Float1([0.0]),
    "lift":        UniformType.Float3([0.0, 0.0, 0.0]),
    "gamma":       UniformType.Float3([1.0, 1.0, 1.0]),
    "gain":        UniformType.Float3([1.0, 1.0, 1.0]),
    "hue_shift":   UniformType.Float1([0.0]),
    "saturation":  UniformType.Float1([1.0]),
    "intensity":   UniformType.Float1([1.0]),
}

TEXTURES = []

# no blending and no depth test: every pixel is overwritten with its graded version,
# whatever was drawn before (3D included).
PIPELINE = PipelineParams()
