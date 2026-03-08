use pyo3::prelude::*;
 
use pyo3_stub_gen::derive::* ;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {

    /// red channel. ranges from 0.0 to 1.0
    #[pyo3(get, set)]
    pub r: f32,

    /// green channel. ranges from 0.0 to 1.0
    #[pyo3(get, set)]
    pub g: f32,

    /// blue channel. ranges from 0.0 to 1.0
    #[pyo3(get, set)]
    pub b: f32,

    /// alpha channel. ranges from 0.0 to 1.0
    #[pyo3(get, set)]
    pub a: f32,

}

#[gen_stub_pymethods]
#[pymethods]
impl Color {

    /// creates a new color.
    ///
    /// inputs range from:
    /// ```
    /// >>>Color(r=0.0, g=0.0, b=0.0, a=1.0) -> Color.BLACK()
    /// ...#to
    /// >>Color(r=1.0, g=1.0, b=1.0, a=1.0) -> Color.WHITE()
    /// ```
    /// r represents the red channel.
    /// 
    /// g represents the green channel.
    /// 
    /// b represents the blue channel.
    /// 
    /// a represents the alpha channel aka. transparency.
    #[pyo3(signature = (r= 1.0, g= 1.0, b= 1.0, a= 1.0))]
    #[new]
    pub fn new(r: f32, g: f32,b: f32,a: f32) -> Self {
       Self { r,g,b,a }
    }

    /// CLOUDY_BLUE | #ACC2D9 | rgb(0.675, 0.761, 0.851)
    #[classattr]
    pub fn CLOUDY_BLUE() -> Color {
        Color { r: 0.6745098039215687, g: 0.7607843137254902, b: 0.8509803921568627, a: 1.0 }
    }

    /// DARK_PASTEL_GREEN | #56AE57 | rgb(0.337, 0.682, 0.341)
    #[classattr]
    pub fn DARK_PASTEL_GREEN() -> Color {
        Color { r: 0.33725490196078434, g: 0.6823529411764706, b: 0.3411764705882353, a: 1.0 }
    }

    /// DUST | #B2996E | rgb(0.698, 0.600, 0.431)
    #[classattr]
    pub fn DUST() -> Color {
        Color { r: 0.6980392156862745, g: 0.6, b: 0.43137254901960786, a: 1.0 }
    }

    /// ELECTRIC_LIME | #A8FF04 | rgb(0.659, 1.000, 0.016)
    #[classattr]
    pub fn ELECTRIC_LIME() -> Color {
        Color { r: 0.6588235294117647, g: 1.0, b: 0.01568627450980392, a: 1.0 }
    }

    /// FRESH_GREEN | #69D84F | rgb(0.412, 0.847, 0.310)
    #[classattr]
    pub fn FRESH_GREEN() -> Color {
        Color { r: 0.4117647058823529, g: 0.8470588235294118, b: 0.30980392156862746, a: 1.0 }
    }

    /// LIGHT_EGGPLANT | #894585 | rgb(0.537, 0.271, 0.522)
    #[classattr]
    pub fn LIGHT_EGGPLANT() -> Color {
        Color { r: 0.5372549019607843, g: 0.27058823529411763, b: 0.5215686274509804, a: 1.0 }
    }

    /// NASTY_GREEN | #70B23F | rgb(0.439, 0.698, 0.247)
    #[classattr]
    pub fn NASTY_GREEN() -> Color {
        Color { r: 0.4392156862745098, g: 0.6980392156862745, b: 0.24705882352941178, a: 1.0 }
    }

    /// REALLY_LIGHT_BLUE | #D4FFFF | rgb(0.831, 1.000, 1.000)
    #[classattr]
    pub fn REALLY_LIGHT_BLUE() -> Color {
        Color { r: 0.8313725490196079, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// TEA | #65AB7C | rgb(0.396, 0.671, 0.486)
    #[classattr]
    pub fn TEA() -> Color {
        Color { r: 0.396078431372549, g: 0.6705882352941176, b: 0.48627450980392156, a: 1.0 }
    }

    /// WARM_PURPLE | #952E8F | rgb(0.584, 0.180, 0.561)
    #[classattr]
    pub fn WARM_PURPLE() -> Color {
        Color { r: 0.5843137254901961, g: 0.1803921568627451, b: 0.5607843137254902, a: 1.0 }
    }

    /// YELLOWISH_TAN | #FCFC81 | rgb(0.988, 0.988, 0.506)
    #[classattr]
    pub fn YELLOWISH_TAN() -> Color {
        Color { r: 0.9882352941176471, g: 0.9882352941176471, b: 0.5058823529411764, a: 1.0 }
    }

    /// CEMENT | #A5A391 | rgb(0.647, 0.639, 0.569)
    #[classattr]
    pub fn CEMENT() -> Color {
        Color { r: 0.6470588235294118, g: 0.6392156862745098, b: 0.5686274509803921, a: 1.0 }
    }

    /// DARK_GRASS_GREEN | #388004 | rgb(0.220, 0.502, 0.016)
    #[classattr]
    pub fn DARK_GRASS_GREEN() -> Color {
        Color { r: 0.2196078431372549, g: 0.5019607843137255, b: 0.01568627450980392, a: 1.0 }
    }

    /// DUSTY_TEAL | #4C9085 | rgb(0.298, 0.565, 0.522)
    #[classattr]
    pub fn DUSTY_TEAL() -> Color {
        Color { r: 0.2980392156862745, g: 0.5647058823529412, b: 0.5215686274509804, a: 1.0 }
    }

    /// GREY_TEAL | #5E9B8A | rgb(0.369, 0.608, 0.541)
    #[classattr]
    pub fn GREY_TEAL() -> Color {
        Color { r: 0.3686274509803922, g: 0.6078431372549019, b: 0.5411764705882353, a: 1.0 }
    }

    /// MACARONI_AND_CHEESE | #EFB435 | rgb(0.937, 0.706, 0.208)
    #[classattr]
    pub fn MACARONI_AND_CHEESE() -> Color {
        Color { r: 0.9372549019607843, g: 0.7058823529411765, b: 0.20784313725490197, a: 1.0 }
    }

    /// PINKISH_TAN | #D99B82 | rgb(0.851, 0.608, 0.510)
    #[classattr]
    pub fn PINKISH_TAN() -> Color {
        Color { r: 0.8509803921568627, g: 0.6078431372549019, b: 0.5098039215686274, a: 1.0 }
    }

    /// SPRUCE | #0A5F38 | rgb(0.039, 0.373, 0.220)
    #[classattr]
    pub fn SPRUCE() -> Color {
        Color { r: 0.0392156862745098, g: 0.37254901960784315, b: 0.2196078431372549, a: 1.0 }
    }

    /// STRONG_BLUE | #0C06F7 | rgb(0.047, 0.024, 0.969)
    #[classattr]
    pub fn STRONG_BLUE() -> Color {
        Color { r: 0.047058823529411764, g: 0.023529411764705882, b: 0.9686274509803922, a: 1.0 }
    }

    /// TOXIC_GREEN | #61DE2A | rgb(0.380, 0.871, 0.165)
    #[classattr]
    pub fn TOXIC_GREEN() -> Color {
        Color { r: 0.3803921568627451, g: 0.8705882352941177, b: 0.16470588235294117, a: 1.0 }
    }

    /// WINDOWS_BLUE | #3778BF | rgb(0.216, 0.471, 0.749)
    #[classattr]
    pub fn WINDOWS_BLUE() -> Color {
        Color { r: 0.21568627450980393, g: 0.47058823529411764, b: 0.7490196078431373, a: 1.0 }
    }

    /// BLUE_BLUE | #2242C7 | rgb(0.133, 0.259, 0.780)
    #[classattr]
    pub fn BLUE_BLUE() -> Color {
        Color { r: 0.13333333333333333, g: 0.25882352941176473, b: 0.7803921568627451, a: 1.0 }
    }

    /// BLUE_WITH_A_HINT_OF_PURPLE | #533CC6 | rgb(0.325, 0.235, 0.776)
    #[classattr]
    pub fn BLUE_WITH_A_HINT_OF_PURPLE() -> Color {
        Color { r: 0.3254901960784314, g: 0.23529411764705882, b: 0.7764705882352941, a: 1.0 }
    }

    /// BOOGER | #9BB53C | rgb(0.608, 0.710, 0.235)
    #[classattr]
    pub fn BOOGER() -> Color {
        Color { r: 0.6078431372549019, g: 0.7098039215686275, b: 0.23529411764705882, a: 1.0 }
    }

    /// BRIGHT_SEA_GREEN | #05FFA6 | rgb(0.020, 1.000, 0.651)
    #[classattr]
    pub fn BRIGHT_SEA_GREEN() -> Color {
        Color { r: 0.0196078431372549, g: 1.0, b: 0.6509803921568628, a: 1.0 }
    }

    /// DARK_GREEN_BLUE | #1F6357 | rgb(0.122, 0.388, 0.341)
    #[classattr]
    pub fn DARK_GREEN_BLUE() -> Color {
        Color { r: 0.12156862745098039, g: 0.38823529411764707, b: 0.3411764705882353, a: 1.0 }
    }

    /// DEEP_TURQUOISE | #017374 | rgb(0.004, 0.451, 0.455)
    #[classattr]
    pub fn DEEP_TURQUOISE() -> Color {
        Color { r: 0.00392156862745098, g: 0.45098039215686275, b: 0.4549019607843137, a: 1.0 }
    }

    /// GREEN_TEAL | #0CB577 | rgb(0.047, 0.710, 0.467)
    #[classattr]
    pub fn GREEN_TEAL() -> Color {
        Color { r: 0.047058823529411764, g: 0.7098039215686275, b: 0.4666666666666667, a: 1.0 }
    }

    /// STRONG_PINK | #FF0789 | rgb(1.000, 0.027, 0.537)
    #[classattr]
    pub fn STRONG_PINK() -> Color {
        Color { r: 1.0, g: 0.027450980392156862, b: 0.5372549019607843, a: 1.0 }
    }

    /// BLAND | #AFA88B | rgb(0.686, 0.659, 0.545)
    #[classattr]
    pub fn BLAND() -> Color {
        Color { r: 0.6862745098039216, g: 0.6588235294117647, b: 0.5450980392156862, a: 1.0 }
    }

    /// DEEP_AQUA | #08787F | rgb(0.031, 0.471, 0.498)
    #[classattr]
    pub fn DEEP_AQUA() -> Color {
        Color { r: 0.03137254901960784, g: 0.47058823529411764, b: 0.4980392156862745, a: 1.0 }
    }

    /// LAVENDER_PINK | #DD85D7 | rgb(0.867, 0.522, 0.843)
    #[classattr]
    pub fn LAVENDER_PINK() -> Color {
        Color { r: 0.8666666666666667, g: 0.5215686274509804, b: 0.8431372549019608, a: 1.0 }
    }

    /// LIGHT_MOSS_GREEN | #A6C875 | rgb(0.651, 0.784, 0.459)
    #[classattr]
    pub fn LIGHT_MOSS_GREEN() -> Color {
        Color { r: 0.6509803921568628, g: 0.7843137254901961, b: 0.4588235294117647, a: 1.0 }
    }

    /// LIGHT_SEAFOAM_GREEN | #A7FFB5 | rgb(0.655, 1.000, 0.710)
    #[classattr]
    pub fn LIGHT_SEAFOAM_GREEN() -> Color {
        Color { r: 0.6549019607843137, g: 1.0, b: 0.7098039215686275, a: 1.0 }
    }

    /// OLIVE_YELLOW | #C2B709 | rgb(0.761, 0.718, 0.035)
    #[classattr]
    pub fn OLIVE_YELLOW() -> Color {
        Color { r: 0.7607843137254902, g: 0.7176470588235294, b: 0.03529411764705882, a: 1.0 }
    }

    /// PIG_PINK | #E78EA5 | rgb(0.906, 0.557, 0.647)
    #[classattr]
    pub fn PIG_PINK() -> Color {
        Color { r: 0.9058823529411765, g: 0.5568627450980392, b: 0.6470588235294118, a: 1.0 }
    }

    /// DEEP_LILAC | #966EBD | rgb(0.588, 0.431, 0.741)
    #[classattr]
    pub fn DEEP_LILAC() -> Color {
        Color { r: 0.5882352941176471, g: 0.43137254901960786, b: 0.7411764705882353, a: 1.0 }
    }

    /// DESERT | #CCAD60 | rgb(0.800, 0.678, 0.376)
    #[classattr]
    pub fn DESERT() -> Color {
        Color { r: 0.8, g: 0.6784313725490196, b: 0.3764705882352941, a: 1.0 }
    }

    /// DUSTY_LAVENDER | #AC86A8 | rgb(0.675, 0.525, 0.659)
    #[classattr]
    pub fn DUSTY_LAVENDER() -> Color {
        Color { r: 0.6745098039215687, g: 0.5254901960784314, b: 0.6588235294117647, a: 1.0 }
    }

    /// PURPLEY_GREY | #947E94 | rgb(0.580, 0.494, 0.580)
    #[classattr]
    pub fn PURPLEY_GREY() -> Color {
        Color { r: 0.5803921568627451, g: 0.49411764705882355, b: 0.5803921568627451, a: 1.0 }
    }

    /// PURPLY | #983FB2 | rgb(0.596, 0.247, 0.698)
    #[classattr]
    pub fn PURPLY() -> Color {
        Color { r: 0.596078431372549, g: 0.24705882352941178, b: 0.6980392156862745, a: 1.0 }
    }

    /// CANDY_PINK | #FF63E9 | rgb(1.000, 0.388, 0.914)
    #[classattr]
    pub fn CANDY_PINK() -> Color {
        Color { r: 1.0, g: 0.38823529411764707, b: 0.9137254901960784, a: 1.0 }
    }

    /// LIGHT_PASTEL_GREEN | #B2FBA5 | rgb(0.698, 0.984, 0.647)
    #[classattr]
    pub fn LIGHT_PASTEL_GREEN() -> Color {
        Color { r: 0.6980392156862745, g: 0.984313725490196, b: 0.6470588235294118, a: 1.0 }
    }

    /// BORING_GREEN | #63B365 | rgb(0.388, 0.702, 0.396)
    #[classattr]
    pub fn BORING_GREEN() -> Color {
        Color { r: 0.38823529411764707, g: 0.7019607843137254, b: 0.396078431372549, a: 1.0 }
    }

    /// KIWI_GREEN | #8EE53F | rgb(0.557, 0.898, 0.247)
    #[classattr]
    pub fn KIWI_GREEN() -> Color {
        Color { r: 0.5568627450980392, g: 0.8980392156862745, b: 0.24705882352941178, a: 1.0 }
    }

    /// LIGHT_GREY_GREEN | #B7E1A1 | rgb(0.718, 0.882, 0.631)
    #[classattr]
    pub fn LIGHT_GREY_GREEN() -> Color {
        Color { r: 0.7176470588235294, g: 0.8823529411764706, b: 0.6313725490196078, a: 1.0 }
    }

    /// ORANGE_PINK | #FF6F52 | rgb(1.000, 0.435, 0.322)
    #[classattr]
    pub fn ORANGE_PINK() -> Color {
        Color { r: 1.0, g: 0.43529411764705883, b: 0.3215686274509804, a: 1.0 }
    }

    /// TEA_GREEN | #BDF8A3 | rgb(0.741, 0.973, 0.639)
    #[classattr]
    pub fn TEA_GREEN() -> Color {
        Color { r: 0.7411764705882353, g: 0.9725490196078431, b: 0.6392156862745098, a: 1.0 }
    }

    /// VERY_LIGHT_BROWN | #D3B683 | rgb(0.827, 0.714, 0.514)
    #[classattr]
    pub fn VERY_LIGHT_BROWN() -> Color {
        Color { r: 0.8274509803921568, g: 0.7137254901960784, b: 0.5137254901960784, a: 1.0 }
    }

    /// EGG_SHELL | #FFFCC4 | rgb(1.000, 0.988, 0.769)
    #[classattr]
    pub fn EGG_SHELL() -> Color {
        Color { r: 1.0, g: 0.9882352941176471, b: 0.7686274509803922, a: 1.0 }
    }

    /// EGGPLANT_PURPLE | #430541 | rgb(0.263, 0.020, 0.255)
    #[classattr]
    pub fn EGGPLANT_PURPLE() -> Color {
        Color { r: 0.2627450980392157, g: 0.0196078431372549, b: 0.2549019607843137, a: 1.0 }
    }

    /// POWDER_PINK | #FFB2D0 | rgb(1.000, 0.698, 0.816)
    #[classattr]
    pub fn POWDER_PINK() -> Color {
        Color { r: 1.0, g: 0.6980392156862745, b: 0.8156862745098039, a: 1.0 }
    }

    /// REDDISH_GREY | #997570 | rgb(0.600, 0.459, 0.439)
    #[classattr]
    pub fn REDDISH_GREY() -> Color {
        Color { r: 0.6, g: 0.4588235294117647, b: 0.4392156862745098, a: 1.0 }
    }

    /// BABY_SHIT_BROWN | #AD900D | rgb(0.678, 0.565, 0.051)
    #[classattr]
    pub fn BABY_SHIT_BROWN() -> Color {
        Color { r: 0.6784313725490196, g: 0.5647058823529412, b: 0.050980392156862744, a: 1.0 }
    }

    /// LILIAC | #C48EFD | rgb(0.769, 0.557, 0.992)
    #[classattr]
    pub fn LILIAC() -> Color {
        Color { r: 0.7686274509803922, g: 0.5568627450980392, b: 0.9921568627450981, a: 1.0 }
    }

    /// STORMY_BLUE | #507B9C | rgb(0.314, 0.482, 0.612)
    #[classattr]
    pub fn STORMY_BLUE() -> Color {
        Color { r: 0.3137254901960784, g: 0.4823529411764706, b: 0.611764705882353, a: 1.0 }
    }

    /// UGLY_BROWN | #7D7103 | rgb(0.490, 0.443, 0.012)
    #[classattr]
    pub fn UGLY_BROWN() -> Color {
        Color { r: 0.49019607843137253, g: 0.44313725490196076, b: 0.011764705882352941, a: 1.0 }
    }

    /// CUSTARD | #FFFD78 | rgb(1.000, 0.992, 0.471)
    #[classattr]
    pub fn CUSTARD() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.47058823529411764, a: 1.0 }
    }

    /// DARKISH_PINK | #DA467D | rgb(0.855, 0.275, 0.490)
    #[classattr]
    pub fn DARKISH_PINK() -> Color {
        Color { r: 0.8549019607843137, g: 0.27450980392156865, b: 0.49019607843137253, a: 1.0 }
    }

    /// DEEP_BROWN | #410200 | rgb(0.255, 0.008, 0.000)
    #[classattr]
    pub fn DEEP_BROWN() -> Color {
        Color { r: 0.2549019607843137, g: 0.00784313725490196, b: 0.0, a: 1.0 }
    }

    /// GREENISH_BEIGE | #C9D179 | rgb(0.788, 0.820, 0.475)
    #[classattr]
    pub fn GREENISH_BEIGE() -> Color {
        Color { r: 0.788235294117647, g: 0.8196078431372549, b: 0.4745098039215686, a: 1.0 }
    }

    /// MANILLA | #FFFA86 | rgb(1.000, 0.980, 0.525)
    #[classattr]
    pub fn MANILLA() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.5254901960784314, a: 1.0 }
    }

    /// OFF_BLUE | #5684AE | rgb(0.337, 0.518, 0.682)
    #[classattr]
    pub fn OFF_BLUE() -> Color {
        Color { r: 0.33725490196078434, g: 0.5176470588235295, b: 0.6823529411764706, a: 1.0 }
    }

    /// BATTLESHIP_GREY | #6B7C85 | rgb(0.420, 0.486, 0.522)
    #[classattr]
    pub fn BATTLESHIP_GREY() -> Color {
        Color { r: 0.4196078431372549, g: 0.48627450980392156, b: 0.5215686274509804, a: 1.0 }
    }

    /// BROWNY_GREEN | #6F6C0A | rgb(0.435, 0.424, 0.039)
    #[classattr]
    pub fn BROWNY_GREEN() -> Color {
        Color { r: 0.43529411764705883, g: 0.4235294117647059, b: 0.0392156862745098, a: 1.0 }
    }

    /// BRUISE | #7E4071 | rgb(0.494, 0.251, 0.443)
    #[classattr]
    pub fn BRUISE() -> Color {
        Color { r: 0.49411764705882355, g: 0.25098039215686274, b: 0.44313725490196076, a: 1.0 }
    }

    /// KELLEY_GREEN | #009337 | rgb(0.000, 0.576, 0.216)
    #[classattr]
    pub fn KELLEY_GREEN() -> Color {
        Color { r: 0.0, g: 0.5764705882352941, b: 0.21568627450980393, a: 1.0 }
    }

    /// SICKLY_YELLOW | #D0E429 | rgb(0.816, 0.894, 0.161)
    #[classattr]
    pub fn SICKLY_YELLOW() -> Color {
        Color { r: 0.8156862745098039, g: 0.8941176470588236, b: 0.1607843137254902, a: 1.0 }
    }

    /// SUNNY_YELLOW | #FFF917 | rgb(1.000, 0.976, 0.090)
    #[classattr]
    pub fn SUNNY_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9764705882352941, b: 0.09019607843137255, a: 1.0 }
    }

    /// AZUL | #1D5DEC | rgb(0.114, 0.365, 0.925)
    #[classattr]
    pub fn AZUL() -> Color {
        Color { r: 0.11372549019607843, g: 0.36470588235294116, b: 0.9254901960784314, a: 1.0 }
    }

    /// DARKGREEN | #054907 | rgb(0.020, 0.286, 0.027)
    #[classattr]
    pub fn DARKGREEN() -> Color {
        Color { r: 0.0196078431372549, g: 0.28627450980392155, b: 0.027450980392156862, a: 1.0 }
    }

    /// GREEN_YELLOW | #B5CE08 | rgb(0.710, 0.808, 0.031)
    #[classattr]
    pub fn GREEN_YELLOW() -> Color {
        Color { r: 0.7098039215686275, g: 0.807843137254902, b: 0.03137254901960784, a: 1.0 }
    }

    /// LICHEN | #8FB67B | rgb(0.561, 0.714, 0.482)
    #[classattr]
    pub fn LICHEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.7137254901960784, b: 0.4823529411764706, a: 1.0 }
    }

    /// LIGHT_LIGHT_GREEN | #C8FFB0 | rgb(0.784, 1.000, 0.690)
    #[classattr]
    pub fn LIGHT_LIGHT_GREEN() -> Color {
        Color { r: 0.7843137254901961, g: 1.0, b: 0.6901960784313725, a: 1.0 }
    }

    /// PALE_GOLD | #FDDE6C | rgb(0.992, 0.871, 0.424)
    #[classattr]
    pub fn PALE_GOLD() -> Color {
        Color { r: 0.9921568627450981, g: 0.8705882352941177, b: 0.4235294117647059, a: 1.0 }
    }

    /// SUN_YELLOW | #FFDF22 | rgb(1.000, 0.875, 0.133)
    #[classattr]
    pub fn SUN_YELLOW() -> Color {
        Color { r: 1.0, g: 0.8745098039215686, b: 0.13333333333333333, a: 1.0 }
    }

    /// TAN_GREEN | #A9BE70 | rgb(0.663, 0.745, 0.439)
    #[classattr]
    pub fn TAN_GREEN() -> Color {
        Color { r: 0.6627450980392157, g: 0.7450980392156863, b: 0.4392156862745098, a: 1.0 }
    }

    /// BURPLE | #6832E3 | rgb(0.408, 0.196, 0.890)
    #[classattr]
    pub fn BURPLE() -> Color {
        Color { r: 0.40784313725490196, g: 0.19607843137254902, b: 0.8901960784313725, a: 1.0 }
    }

    /// BUTTERSCOTCH | #FDB147 | rgb(0.992, 0.694, 0.278)
    #[classattr]
    pub fn BUTTERSCOTCH() -> Color {
        Color { r: 0.9921568627450981, g: 0.6941176470588235, b: 0.2784313725490196, a: 1.0 }
    }

    /// TOUPE | #C7AC7D | rgb(0.780, 0.675, 0.490)
    #[classattr]
    pub fn TOUPE() -> Color {
        Color { r: 0.7803921568627451, g: 0.6745098039215687, b: 0.49019607843137253, a: 1.0 }
    }

    /// DARK_CREAM | #FFF39A | rgb(1.000, 0.953, 0.604)
    #[classattr]
    pub fn DARK_CREAM() -> Color {
        Color { r: 1.0, g: 0.9529411764705882, b: 0.6039215686274509, a: 1.0 }
    }

    /// INDIAN_RED | #850E04 | rgb(0.522, 0.055, 0.016)
    #[classattr]
    pub fn INDIAN_RED() -> Color {
        Color { r: 0.5215686274509804, g: 0.054901960784313725, b: 0.01568627450980392, a: 1.0 }
    }

    /// LIGHT_LAVENDAR | #EFC0FE | rgb(0.937, 0.753, 0.996)
    #[classattr]
    pub fn LIGHT_LAVENDAR() -> Color {
        Color { r: 0.9372549019607843, g: 0.7529411764705882, b: 0.996078431372549, a: 1.0 }
    }

    /// POISON_GREEN | #40FD14 | rgb(0.251, 0.992, 0.078)
    #[classattr]
    pub fn POISON_GREEN() -> Color {
        Color { r: 0.25098039215686274, g: 0.9921568627450981, b: 0.0784313725490196, a: 1.0 }
    }

    /// BABY_PUKE_GREEN | #B6C406 | rgb(0.714, 0.769, 0.024)
    #[classattr]
    pub fn BABY_PUKE_GREEN() -> Color {
        Color { r: 0.7137254901960784, g: 0.7686274509803922, b: 0.023529411764705882, a: 1.0 }
    }

    /// BRIGHT_YELLOW_GREEN | #9DFF00 | rgb(0.616, 1.000, 0.000)
    #[classattr]
    pub fn BRIGHT_YELLOW_GREEN() -> Color {
        Color { r: 0.615686274509804, g: 1.0, b: 0.0, a: 1.0 }
    }

    /// CHARCOAL_GREY | #3C4142 | rgb(0.235, 0.255, 0.259)
    #[classattr]
    pub fn CHARCOAL_GREY() -> Color {
        Color { r: 0.23529411764705882, g: 0.2549019607843137, b: 0.25882352941176473, a: 1.0 }
    }

    /// SQUASH | #F2AB15 | rgb(0.949, 0.671, 0.082)
    #[classattr]
    pub fn SQUASH() -> Color {
        Color { r: 0.9490196078431372, g: 0.6705882352941176, b: 0.08235294117647059, a: 1.0 }
    }

    /// CINNAMON | #AC4F06 | rgb(0.675, 0.310, 0.024)
    #[classattr]
    pub fn CINNAMON() -> Color {
        Color { r: 0.6745098039215687, g: 0.30980392156862746, b: 0.023529411764705882, a: 1.0 }
    }

    /// LIGHT_PEA_GREEN | #C4FE82 | rgb(0.769, 0.996, 0.510)
    #[classattr]
    pub fn LIGHT_PEA_GREEN() -> Color {
        Color { r: 0.7686274509803922, g: 0.996078431372549, b: 0.5098039215686274, a: 1.0 }
    }

    /// RADIOACTIVE_GREEN | #2CFA1F | rgb(0.173, 0.980, 0.122)
    #[classattr]
    pub fn RADIOACTIVE_GREEN() -> Color {
        Color { r: 0.17254901960784313, g: 0.9803921568627451, b: 0.12156862745098039, a: 1.0 }
    }

    /// RAW_SIENNA | #9A6200 | rgb(0.604, 0.384, 0.000)
    #[classattr]
    pub fn RAW_SIENNA() -> Color {
        Color { r: 0.6039215686274509, g: 0.3843137254901961, b: 0.0, a: 1.0 }
    }

    /// BABY_PURPLE | #CA9BF7 | rgb(0.792, 0.608, 0.969)
    #[classattr]
    pub fn BABY_PURPLE() -> Color {
        Color { r: 0.792156862745098, g: 0.6078431372549019, b: 0.9686274509803922, a: 1.0 }
    }

    /// COCOA | #875F42 | rgb(0.529, 0.373, 0.259)
    #[classattr]
    pub fn COCOA() -> Color {
        Color { r: 0.5294117647058824, g: 0.37254901960784315, b: 0.25882352941176473, a: 1.0 }
    }

    /// LIGHT_ROYAL_BLUE | #3A2EFE | rgb(0.227, 0.180, 0.996)
    #[classattr]
    pub fn LIGHT_ROYAL_BLUE() -> Color {
        Color { r: 0.22745098039215686, g: 0.1803921568627451, b: 0.996078431372549, a: 1.0 }
    }

    /// ORANGEISH | #FD8D49 | rgb(0.992, 0.553, 0.286)
    #[classattr]
    pub fn ORANGEISH() -> Color {
        Color { r: 0.9921568627450981, g: 0.5529411764705883, b: 0.28627450980392155, a: 1.0 }
    }

    /// RUST_BROWN | #8B3103 | rgb(0.545, 0.192, 0.012)
    #[classattr]
    pub fn RUST_BROWN() -> Color {
        Color { r: 0.5450980392156862, g: 0.19215686274509805, b: 0.011764705882352941, a: 1.0 }
    }

    /// SAND_BROWN | #CBA560 | rgb(0.796, 0.647, 0.376)
    #[classattr]
    pub fn SAND_BROWN() -> Color {
        Color { r: 0.796078431372549, g: 0.6470588235294118, b: 0.3764705882352941, a: 1.0 }
    }

    /// SWAMP | #698339 | rgb(0.412, 0.514, 0.224)
    #[classattr]
    pub fn SWAMP() -> Color {
        Color { r: 0.4117647058823529, g: 0.5137254901960784, b: 0.2235294117647059, a: 1.0 }
    }

    /// TEALISH_GREEN | #0CDC73 | rgb(0.047, 0.863, 0.451)
    #[classattr]
    pub fn TEALISH_GREEN() -> Color {
        Color { r: 0.047058823529411764, g: 0.8627450980392157, b: 0.45098039215686275, a: 1.0 }
    }

    /// BURNT_SIENA | #B75203 | rgb(0.718, 0.322, 0.012)
    #[classattr]
    pub fn BURNT_SIENA() -> Color {
        Color { r: 0.7176470588235294, g: 0.3215686274509804, b: 0.011764705882352941, a: 1.0 }
    }

    /// CAMO | #7F8F4E | rgb(0.498, 0.561, 0.306)
    #[classattr]
    pub fn CAMO() -> Color {
        Color { r: 0.4980392156862745, g: 0.5607843137254902, b: 0.3058823529411765, a: 1.0 }
    }

    /// DUSK_BLUE | #26538D | rgb(0.149, 0.325, 0.553)
    #[classattr]
    pub fn DUSK_BLUE() -> Color {
        Color { r: 0.14901960784313725, g: 0.3254901960784314, b: 0.5529411764705883, a: 1.0 }
    }

    /// FERN | #63A950 | rgb(0.388, 0.663, 0.314)
    #[classattr]
    pub fn FERN() -> Color {
        Color { r: 0.38823529411764707, g: 0.6627450980392157, b: 0.3137254901960784, a: 1.0 }
    }

    /// OLD_ROSE | #C87F89 | rgb(0.784, 0.498, 0.537)
    #[classattr]
    pub fn OLD_ROSE() -> Color {
        Color { r: 0.7843137254901961, g: 0.4980392156862745, b: 0.5372549019607843, a: 1.0 }
    }

    /// PALE_LIGHT_GREEN | #B1FC99 | rgb(0.694, 0.988, 0.600)
    #[classattr]
    pub fn PALE_LIGHT_GREEN() -> Color {
        Color { r: 0.6941176470588235, g: 0.9882352941176471, b: 0.6, a: 1.0 }
    }

    /// PEACHY_PINK | #FF9A8A | rgb(1.000, 0.604, 0.541)
    #[classattr]
    pub fn PEACHY_PINK() -> Color {
        Color { r: 1.0, g: 0.6039215686274509, b: 0.5411764705882353, a: 1.0 }
    }

    /// ROSY_PINK | #F6688E | rgb(0.965, 0.408, 0.557)
    #[classattr]
    pub fn ROSY_PINK() -> Color {
        Color { r: 0.9647058823529412, g: 0.40784313725490196, b: 0.5568627450980392, a: 1.0 }
    }

    /// LIGHT_BLUISH_GREEN | #76FDA8 | rgb(0.463, 0.992, 0.659)
    #[classattr]
    pub fn LIGHT_BLUISH_GREEN() -> Color {
        Color { r: 0.4627450980392157, g: 0.9921568627450981, b: 0.6588235294117647, a: 1.0 }
    }

    /// LIGHT_BRIGHT_GREEN | #53FE5C | rgb(0.325, 0.996, 0.361)
    #[classattr]
    pub fn LIGHT_BRIGHT_GREEN() -> Color {
        Color { r: 0.3254901960784314, g: 0.996078431372549, b: 0.3607843137254902, a: 1.0 }
    }

    /// LIGHT_NEON_GREEN | #4EFD54 | rgb(0.306, 0.992, 0.329)
    #[classattr]
    pub fn LIGHT_NEON_GREEN() -> Color {
        Color { r: 0.3058823529411765, g: 0.9921568627450981, b: 0.32941176470588235, a: 1.0 }
    }

    /// LIGHT_SEAFOAM | #A0FEBF | rgb(0.627, 0.996, 0.749)
    #[classattr]
    pub fn LIGHT_SEAFOAM() -> Color {
        Color { r: 0.6274509803921569, g: 0.996078431372549, b: 0.7490196078431373, a: 1.0 }
    }

    /// TIFFANY_BLUE | #7BF2DA | rgb(0.482, 0.949, 0.855)
    #[classattr]
    pub fn TIFFANY_BLUE() -> Color {
        Color { r: 0.4823529411764706, g: 0.9490196078431372, b: 0.8549019607843137, a: 1.0 }
    }

    /// WASHED_OUT_GREEN | #BCF5A6 | rgb(0.737, 0.961, 0.651)
    #[classattr]
    pub fn WASHED_OUT_GREEN() -> Color {
        Color { r: 0.7372549019607844, g: 0.9607843137254902, b: 0.6509803921568628, a: 1.0 }
    }

    /// BROWNY_ORANGE | #CA6B02 | rgb(0.792, 0.420, 0.008)
    #[classattr]
    pub fn BROWNY_ORANGE() -> Color {
        Color { r: 0.792156862745098, g: 0.4196078431372549, b: 0.00784313725490196, a: 1.0 }
    }

    /// NICE_BLUE | #107AB0 | rgb(0.063, 0.478, 0.690)
    #[classattr]
    pub fn NICE_BLUE() -> Color {
        Color { r: 0.06274509803921569, g: 0.47843137254901963, b: 0.6901960784313725, a: 1.0 }
    }

    /// SAPPHIRE | #2138AB | rgb(0.129, 0.220, 0.671)
    #[classattr]
    pub fn SAPPHIRE() -> Color {
        Color { r: 0.12941176470588237, g: 0.2196078431372549, b: 0.6705882352941176, a: 1.0 }
    }

    /// GREYISH_TEAL | #719F91 | rgb(0.443, 0.624, 0.569)
    #[classattr]
    pub fn GREYISH_TEAL() -> Color {
        Color { r: 0.44313725490196076, g: 0.6235294117647059, b: 0.5686274509803921, a: 1.0 }
    }

    /// ORANGEY_YELLOW | #FDB915 | rgb(0.992, 0.725, 0.082)
    #[classattr]
    pub fn ORANGEY_YELLOW() -> Color {
        Color { r: 0.9921568627450981, g: 0.7254901960784313, b: 0.08235294117647059, a: 1.0 }
    }

    /// PARCHMENT | #FEFCAF | rgb(0.996, 0.988, 0.686)
    #[classattr]
    pub fn PARCHMENT() -> Color {
        Color { r: 0.996078431372549, g: 0.9882352941176471, b: 0.6862745098039216, a: 1.0 }
    }

    /// STRAW | #FCF679 | rgb(0.988, 0.965, 0.475)
    #[classattr]
    pub fn STRAW() -> Color {
        Color { r: 0.9882352941176471, g: 0.9647058823529412, b: 0.4745098039215686, a: 1.0 }
    }

    /// VERY_DARK_BROWN | #1D0200 | rgb(0.114, 0.008, 0.000)
    #[classattr]
    pub fn VERY_DARK_BROWN() -> Color {
        Color { r: 0.11372549019607843, g: 0.00784313725490196, b: 0.0, a: 1.0 }
    }

    /// TERRACOTA | #CB6843 | rgb(0.796, 0.408, 0.263)
    #[classattr]
    pub fn TERRACOTA() -> Color {
        Color { r: 0.796078431372549, g: 0.40784313725490196, b: 0.2627450980392157, a: 1.0 }
    }

    /// UGLY_BLUE | #31668A | rgb(0.192, 0.400, 0.541)
    #[classattr]
    pub fn UGLY_BLUE() -> Color {
        Color { r: 0.19215686274509805, g: 0.4, b: 0.5411764705882353, a: 1.0 }
    }

    /// CLEAR_BLUE | #247AFD | rgb(0.141, 0.478, 0.992)
    #[classattr]
    pub fn CLEAR_BLUE() -> Color {
        Color { r: 0.1411764705882353, g: 0.47843137254901963, b: 0.9921568627450981, a: 1.0 }
    }

    /// CREME | #FFFFB6 | rgb(1.000, 1.000, 0.714)
    #[classattr]
    pub fn CREME() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.7137254901960784, a: 1.0 }
    }

    /// FOAM_GREEN | #90FDA9 | rgb(0.565, 0.992, 0.663)
    #[classattr]
    pub fn FOAM_GREEN() -> Color {
        Color { r: 0.5647058823529412, g: 0.9921568627450981, b: 0.6627450980392157, a: 1.0 }
    }

    /// GREY_GREEN | #86A17D | rgb(0.525, 0.631, 0.490)
    #[classattr]
    pub fn GREY_GREEN() -> Color {
        Color { r: 0.5254901960784314, g: 0.6313725490196078, b: 0.49019607843137253, a: 1.0 }
    }

    /// LIGHT_GOLD | #FDDC5C | rgb(0.992, 0.863, 0.361)
    #[classattr]
    pub fn LIGHT_GOLD() -> Color {
        Color { r: 0.9921568627450981, g: 0.8627450980392157, b: 0.3607843137254902, a: 1.0 }
    }

    /// SEAFOAM_BLUE | #78D1B6 | rgb(0.471, 0.820, 0.714)
    #[classattr]
    pub fn SEAFOAM_BLUE() -> Color {
        Color { r: 0.47058823529411764, g: 0.8196078431372549, b: 0.7137254901960784, a: 1.0 }
    }

    /// TOPAZ | #13BBAF | rgb(0.075, 0.733, 0.686)
    #[classattr]
    pub fn TOPAZ() -> Color {
        Color { r: 0.07450980392156863, g: 0.7333333333333333, b: 0.6862745098039216, a: 1.0 }
    }

    /// VIOLET_PINK | #FB5FFC | rgb(0.984, 0.373, 0.988)
    #[classattr]
    pub fn VIOLET_PINK() -> Color {
        Color { r: 0.984313725490196, g: 0.37254901960784315, b: 0.9882352941176471, a: 1.0 }
    }

    /// WINTERGREEN | #20F986 | rgb(0.125, 0.976, 0.525)
    #[classattr]
    pub fn WINTERGREEN() -> Color {
        Color { r: 0.12549019607843137, g: 0.9764705882352941, b: 0.5254901960784314, a: 1.0 }
    }

    /// YELLOW_TAN | #FFE36E | rgb(1.000, 0.890, 0.431)
    #[classattr]
    pub fn YELLOW_TAN() -> Color {
        Color { r: 1.0, g: 0.8901960784313725, b: 0.43137254901960786, a: 1.0 }
    }

    /// DARK_FUCHSIA | #9D0759 | rgb(0.616, 0.027, 0.349)
    #[classattr]
    pub fn DARK_FUCHSIA() -> Color {
        Color { r: 0.615686274509804, g: 0.027450980392156862, b: 0.34901960784313724, a: 1.0 }
    }

    /// INDIGO_BLUE | #3A18B1 | rgb(0.227, 0.094, 0.694)
    #[classattr]
    pub fn INDIGO_BLUE() -> Color {
        Color { r: 0.22745098039215686, g: 0.09411764705882353, b: 0.6941176470588235, a: 1.0 }
    }

    /// LIGHT_YELLOWISH_GREEN | #C2FF89 | rgb(0.761, 1.000, 0.537)
    #[classattr]
    pub fn LIGHT_YELLOWISH_GREEN() -> Color {
        Color { r: 0.7607843137254902, g: 1.0, b: 0.5372549019607843, a: 1.0 }
    }

    /// PALE_MAGENTA | #D767AD | rgb(0.843, 0.404, 0.678)
    #[classattr]
    pub fn PALE_MAGENTA() -> Color {
        Color { r: 0.8431372549019608, g: 0.403921568627451, b: 0.6784313725490196, a: 1.0 }
    }

    /// RICH_PURPLE | #720058 | rgb(0.447, 0.000, 0.345)
    #[classattr]
    pub fn RICH_PURPLE() -> Color {
        Color { r: 0.4470588235294118, g: 0.0, b: 0.34509803921568627, a: 1.0 }
    }

    /// SUNFLOWER_YELLOW | #FFDA03 | rgb(1.000, 0.855, 0.012)
    #[classattr]
    pub fn SUNFLOWER_YELLOW() -> Color {
        Color { r: 1.0, g: 0.8549019607843137, b: 0.011764705882352941, a: 1.0 }
    }

    /// GREEN_BLUE | #01C08D | rgb(0.004, 0.753, 0.553)
    #[classattr]
    pub fn GREEN_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.7529411764705882, b: 0.5529411764705883, a: 1.0 }
    }

    /// LEATHER | #AC7434 | rgb(0.675, 0.455, 0.204)
    #[classattr]
    pub fn LEATHER() -> Color {
        Color { r: 0.6745098039215687, g: 0.4549019607843137, b: 0.20392156862745098, a: 1.0 }
    }

    /// RACING_GREEN | #014600 | rgb(0.004, 0.275, 0.000)
    #[classattr]
    pub fn RACING_GREEN() -> Color {
        Color { r: 0.00392156862745098, g: 0.27450980392156865, b: 0.0, a: 1.0 }
    }

    /// VIVID_PURPLE | #9900FA | rgb(0.600, 0.000, 0.980)
    #[classattr]
    pub fn VIVID_PURPLE() -> Color {
        Color { r: 0.6, g: 0.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// DARK_ROYAL_BLUE | #02066F | rgb(0.008, 0.024, 0.435)
    #[classattr]
    pub fn DARK_ROYAL_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.023529411764705882, b: 0.43529411764705883, a: 1.0 }
    }

    /// HAZEL | #8E7618 | rgb(0.557, 0.463, 0.094)
    #[classattr]
    pub fn HAZEL() -> Color {
        Color { r: 0.5568627450980392, g: 0.4627450980392157, b: 0.09411764705882353, a: 1.0 }
    }

    /// MUTED_PINK | #D1768F | rgb(0.820, 0.463, 0.561)
    #[classattr]
    pub fn MUTED_PINK() -> Color {
        Color { r: 0.8196078431372549, g: 0.4627450980392157, b: 0.5607843137254902, a: 1.0 }
    }

    /// BOOGER_GREEN | #96B403 | rgb(0.588, 0.706, 0.012)
    #[classattr]
    pub fn BOOGER_GREEN() -> Color {
        Color { r: 0.5882352941176471, g: 0.7058823529411765, b: 0.011764705882352941, a: 1.0 }
    }

    /// CANARY | #FDFF63 | rgb(0.992, 1.000, 0.388)
    #[classattr]
    pub fn CANARY() -> Color {
        Color { r: 0.9921568627450981, g: 1.0, b: 0.38823529411764707, a: 1.0 }
    }

    /// COOL_GREY | #95A3A6 | rgb(0.584, 0.639, 0.651)
    #[classattr]
    pub fn COOL_GREY() -> Color {
        Color { r: 0.5843137254901961, g: 0.6392156862745098, b: 0.6509803921568628, a: 1.0 }
    }

    /// DARK_TAUPE | #7F684E | rgb(0.498, 0.408, 0.306)
    #[classattr]
    pub fn DARK_TAUPE() -> Color {
        Color { r: 0.4980392156862745, g: 0.40784313725490196, b: 0.3058823529411765, a: 1.0 }
    }

    /// DARKISH_PURPLE | #751973 | rgb(0.459, 0.098, 0.451)
    #[classattr]
    pub fn DARKISH_PURPLE() -> Color {
        Color { r: 0.4588235294117647, g: 0.09803921568627451, b: 0.45098039215686275, a: 1.0 }
    }

    /// TRUE_GREEN | #089404 | rgb(0.031, 0.580, 0.016)
    #[classattr]
    pub fn TRUE_GREEN() -> Color {
        Color { r: 0.03137254901960784, g: 0.5803921568627451, b: 0.01568627450980392, a: 1.0 }
    }

    /// CORAL_PINK | #FF6163 | rgb(1.000, 0.380, 0.388)
    #[classattr]
    pub fn CORAL_PINK() -> Color {
        Color { r: 1.0, g: 0.3803921568627451, b: 0.38823529411764707, a: 1.0 }
    }

    /// DARK_SAGE | #598556 | rgb(0.349, 0.522, 0.337)
    #[classattr]
    pub fn DARK_SAGE() -> Color {
        Color { r: 0.34901960784313724, g: 0.5215686274509804, b: 0.33725490196078434, a: 1.0 }
    }

    /// DARK_SLATE_BLUE | #214761 | rgb(0.129, 0.278, 0.380)
    #[classattr]
    pub fn DARK_SLATE_BLUE() -> Color {
        Color { r: 0.12941176470588237, g: 0.2784313725490196, b: 0.3803921568627451, a: 1.0 }
    }

    /// FLAT_BLUE | #3C73A8 | rgb(0.235, 0.451, 0.659)
    #[classattr]
    pub fn FLAT_BLUE() -> Color {
        Color { r: 0.23529411764705882, g: 0.45098039215686275, b: 0.6588235294117647, a: 1.0 }
    }

    /// MUSHROOM | #BA9E88 | rgb(0.729, 0.620, 0.533)
    #[classattr]
    pub fn MUSHROOM() -> Color {
        Color { r: 0.7294117647058823, g: 0.6196078431372549, b: 0.5333333333333333, a: 1.0 }
    }

    /// RICH_BLUE | #021BF9 | rgb(0.008, 0.106, 0.976)
    #[classattr]
    pub fn RICH_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.10588235294117647, b: 0.9764705882352941, a: 1.0 }
    }

    /// DIRTY_PURPLE | #734A65 | rgb(0.451, 0.290, 0.396)
    #[classattr]
    pub fn DIRTY_PURPLE() -> Color {
        Color { r: 0.45098039215686275, g: 0.2901960784313726, b: 0.396078431372549, a: 1.0 }
    }

    /// GREENBLUE | #23C48B | rgb(0.137, 0.769, 0.545)
    #[classattr]
    pub fn GREENBLUE() -> Color {
        Color { r: 0.13725490196078433, g: 0.7686274509803922, b: 0.5450980392156862, a: 1.0 }
    }

    /// ICKY_GREEN | #8FAE22 | rgb(0.561, 0.682, 0.133)
    #[classattr]
    pub fn ICKY_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.6823529411764706, b: 0.13333333333333333, a: 1.0 }
    }

    /// LIGHT_KHAKI | #E6F2A2 | rgb(0.902, 0.949, 0.635)
    #[classattr]
    pub fn LIGHT_KHAKI() -> Color {
        Color { r: 0.9019607843137255, g: 0.9490196078431372, b: 0.6352941176470588, a: 1.0 }
    }

    /// WARM_BLUE | #4B57DB | rgb(0.294, 0.341, 0.859)
    #[classattr]
    pub fn WARM_BLUE() -> Color {
        Color { r: 0.29411764705882354, g: 0.3411764705882353, b: 0.8588235294117647, a: 1.0 }
    }

    /// DARK_HOT_PINK | #D90166 | rgb(0.851, 0.004, 0.400)
    #[classattr]
    pub fn DARK_HOT_PINK() -> Color {
        Color { r: 0.8509803921568627, g: 0.00392156862745098, b: 0.4, a: 1.0 }
    }

    /// DEEP_SEA_BLUE | #015482 | rgb(0.004, 0.329, 0.510)
    #[classattr]
    pub fn DEEP_SEA_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.32941176470588235, b: 0.5098039215686274, a: 1.0 }
    }

    /// CARMINE | #9D0216 | rgb(0.616, 0.008, 0.086)
    #[classattr]
    pub fn CARMINE() -> Color {
        Color { r: 0.615686274509804, g: 0.00784313725490196, b: 0.08627450980392157, a: 1.0 }
    }

    /// DARK_YELLOW_GREEN | #728F02 | rgb(0.447, 0.561, 0.008)
    #[classattr]
    pub fn DARK_YELLOW_GREEN() -> Color {
        Color { r: 0.4470588235294118, g: 0.5607843137254902, b: 0.00784313725490196, a: 1.0 }
    }

    /// PALE_PEACH | #FFE5AD | rgb(1.000, 0.898, 0.678)
    #[classattr]
    pub fn PALE_PEACH() -> Color {
        Color { r: 1.0, g: 0.8980392156862745, b: 0.6784313725490196, a: 1.0 }
    }

    /// PLUM_PURPLE | #4E0550 | rgb(0.306, 0.020, 0.314)
    #[classattr]
    pub fn PLUM_PURPLE() -> Color {
        Color { r: 0.3058823529411765, g: 0.0196078431372549, b: 0.3137254901960784, a: 1.0 }
    }

    /// GOLDEN_ROD | #F9BC08 | rgb(0.976, 0.737, 0.031)
    #[classattr]
    pub fn GOLDEN_ROD() -> Color {
        Color { r: 0.9764705882352941, g: 0.7372549019607844, b: 0.03137254901960784, a: 1.0 }
    }

    /// NEON_RED | #FF073A | rgb(1.000, 0.027, 0.227)
    #[classattr]
    pub fn NEON_RED() -> Color {
        Color { r: 1.0, g: 0.027450980392156862, b: 0.22745098039215686, a: 1.0 }
    }

    /// OLD_PINK | #C77986 | rgb(0.780, 0.475, 0.525)
    #[classattr]
    pub fn OLD_PINK() -> Color {
        Color { r: 0.7803921568627451, g: 0.4745098039215686, b: 0.5254901960784314, a: 1.0 }
    }

    /// VERY_PALE_BLUE | #D6FFFE | rgb(0.839, 1.000, 0.996)
    #[classattr]
    pub fn VERY_PALE_BLUE() -> Color {
        Color { r: 0.8392156862745098, g: 1.0, b: 0.996078431372549, a: 1.0 }
    }

    /// BLOOD_ORANGE | #FE4B03 | rgb(0.996, 0.294, 0.012)
    #[classattr]
    pub fn BLOOD_ORANGE() -> Color {
        Color { r: 0.996078431372549, g: 0.29411764705882354, b: 0.011764705882352941, a: 1.0 }
    }

    /// GRAPEFRUIT | #FD5956 | rgb(0.992, 0.349, 0.337)
    #[classattr]
    pub fn GRAPEFRUIT() -> Color {
        Color { r: 0.9921568627450981, g: 0.34901960784313724, b: 0.33725490196078434, a: 1.0 }
    }

    /// SAND_YELLOW | #FCE166 | rgb(0.988, 0.882, 0.400)
    #[classattr]
    pub fn SAND_YELLOW() -> Color {
        Color { r: 0.9882352941176471, g: 0.8823529411764706, b: 0.4, a: 1.0 }
    }

    /// CLAY_BROWN | #B2713D | rgb(0.698, 0.443, 0.239)
    #[classattr]
    pub fn CLAY_BROWN() -> Color {
        Color { r: 0.6980392156862745, g: 0.44313725490196076, b: 0.23921568627450981, a: 1.0 }
    }

    /// DARK_BLUE_GREY | #1F3B4D | rgb(0.122, 0.231, 0.302)
    #[classattr]
    pub fn DARK_BLUE_GREY() -> Color {
        Color { r: 0.12156862745098039, g: 0.23137254901960785, b: 0.30196078431372547, a: 1.0 }
    }

    /// FLAT_GREEN | #699D4C | rgb(0.412, 0.616, 0.298)
    #[classattr]
    pub fn FLAT_GREEN() -> Color {
        Color { r: 0.4117647058823529, g: 0.615686274509804, b: 0.2980392156862745, a: 1.0 }
    }

    /// LIGHT_GREEN_BLUE | #56FCA2 | rgb(0.337, 0.988, 0.635)
    #[classattr]
    pub fn LIGHT_GREEN_BLUE() -> Color {
        Color { r: 0.33725490196078434, g: 0.9882352941176471, b: 0.6352941176470588, a: 1.0 }
    }

    /// WARM_PINK | #FB5581 | rgb(0.984, 0.333, 0.506)
    #[classattr]
    pub fn WARM_PINK() -> Color {
        Color { r: 0.984313725490196, g: 0.3333333333333333, b: 0.5058823529411764, a: 1.0 }
    }

    /// DODGER_BLUE | #3E82FC | rgb(0.243, 0.510, 0.988)
    #[classattr]
    pub fn DODGER_BLUE() -> Color {
        Color { r: 0.24313725490196078, g: 0.5098039215686274, b: 0.9882352941176471, a: 1.0 }
    }

    /// GROSS_GREEN | #A0BF16 | rgb(0.627, 0.749, 0.086)
    #[classattr]
    pub fn GROSS_GREEN() -> Color {
        Color { r: 0.6274509803921569, g: 0.7490196078431373, b: 0.08627450980392157, a: 1.0 }
    }

    /// ICE | #D6FFFA | rgb(0.839, 1.000, 0.980)
    #[classattr]
    pub fn ICE() -> Color {
        Color { r: 0.8392156862745098, g: 1.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// METALLIC_BLUE | #4F738E | rgb(0.310, 0.451, 0.557)
    #[classattr]
    pub fn METALLIC_BLUE() -> Color {
        Color { r: 0.30980392156862746, g: 0.45098039215686275, b: 0.5568627450980392, a: 1.0 }
    }

    /// PALE_SALMON | #FFB19A | rgb(1.000, 0.694, 0.604)
    #[classattr]
    pub fn PALE_SALMON() -> Color {
        Color { r: 1.0, g: 0.6941176470588235, b: 0.6039215686274509, a: 1.0 }
    }

    /// SAP_GREEN | #5C8B15 | rgb(0.361, 0.545, 0.082)
    #[classattr]
    pub fn SAP_GREEN() -> Color {
        Color { r: 0.3607843137254902, g: 0.5450980392156862, b: 0.08235294117647059, a: 1.0 }
    }

    /// ALGAE | #54AC68 | rgb(0.329, 0.675, 0.408)
    #[classattr]
    pub fn ALGAE() -> Color {
        Color { r: 0.32941176470588235, g: 0.6745098039215687, b: 0.40784313725490196, a: 1.0 }
    }

    /// BLUEY_GREY | #89A0B0 | rgb(0.537, 0.627, 0.690)
    #[classattr]
    pub fn BLUEY_GREY() -> Color {
        Color { r: 0.5372549019607843, g: 0.6274509803921569, b: 0.6901960784313725, a: 1.0 }
    }

    /// GREENY_GREY | #7EA07A | rgb(0.494, 0.627, 0.478)
    #[classattr]
    pub fn GREENY_GREY() -> Color {
        Color { r: 0.49411764705882355, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 }
    }

    /// HIGHLIGHTER_GREEN | #1BFC06 | rgb(0.106, 0.988, 0.024)
    #[classattr]
    pub fn HIGHLIGHTER_GREEN() -> Color {
        Color { r: 0.10588235294117647, g: 0.9882352941176471, b: 0.023529411764705882, a: 1.0 }
    }

    /// LIGHT_LIGHT_BLUE | #CAFFFB | rgb(0.792, 1.000, 0.984)
    #[classattr]
    pub fn LIGHT_LIGHT_BLUE() -> Color {
        Color { r: 0.792156862745098, g: 1.0, b: 0.984313725490196, a: 1.0 }
    }

    /// LIGHT_MINT | #B6FFBB | rgb(0.714, 1.000, 0.733)
    #[classattr]
    pub fn LIGHT_MINT() -> Color {
        Color { r: 0.7137254901960784, g: 1.0, b: 0.7333333333333333, a: 1.0 }
    }

    /// RAW_UMBER | #A75E09 | rgb(0.655, 0.369, 0.035)
    #[classattr]
    pub fn RAW_UMBER() -> Color {
        Color { r: 0.6549019607843137, g: 0.3686274509803922, b: 0.03529411764705882, a: 1.0 }
    }

    /// VIVID_BLUE | #152EFF | rgb(0.082, 0.180, 1.000)
    #[classattr]
    pub fn VIVID_BLUE() -> Color {
        Color { r: 0.08235294117647059, g: 0.1803921568627451, b: 1.0, a: 1.0 }
    }

    /// DEEP_LAVENDER | #8D5EB7 | rgb(0.553, 0.369, 0.718)
    #[classattr]
    pub fn DEEP_LAVENDER() -> Color {
        Color { r: 0.5529411764705883, g: 0.3686274509803922, b: 0.7176470588235294, a: 1.0 }
    }

    /// DULL_TEAL | #5F9E8F | rgb(0.373, 0.620, 0.561)
    #[classattr]
    pub fn DULL_TEAL() -> Color {
        Color { r: 0.37254901960784315, g: 0.6196078431372549, b: 0.5607843137254902, a: 1.0 }
    }

    /// LIGHT_GREENISH_BLUE | #63F7B4 | rgb(0.388, 0.969, 0.706)
    #[classattr]
    pub fn LIGHT_GREENISH_BLUE() -> Color {
        Color { r: 0.38823529411764707, g: 0.9686274509803922, b: 0.7058823529411765, a: 1.0 }
    }

    /// MUD_GREEN | #606602 | rgb(0.376, 0.400, 0.008)
    #[classattr]
    pub fn MUD_GREEN() -> Color {
        Color { r: 0.3764705882352941, g: 0.4, b: 0.00784313725490196, a: 1.0 }
    }

    /// PINKY | #FC86AA | rgb(0.988, 0.525, 0.667)
    #[classattr]
    pub fn PINKY() -> Color {
        Color { r: 0.9882352941176471, g: 0.5254901960784314, b: 0.6666666666666666, a: 1.0 }
    }

    /// RED_WINE | #8C0034 | rgb(0.549, 0.000, 0.204)
    #[classattr]
    pub fn RED_WINE() -> Color {
        Color { r: 0.5490196078431373, g: 0.0, b: 0.20392156862745098, a: 1.0 }
    }

    /// SHIT_GREEN | #758000 | rgb(0.459, 0.502, 0.000)
    #[classattr]
    pub fn SHIT_GREEN() -> Color {
        Color { r: 0.4588235294117647, g: 0.5019607843137255, b: 0.0, a: 1.0 }
    }

    /// TAN_BROWN | #AB7E4C | rgb(0.671, 0.494, 0.298)
    #[classattr]
    pub fn TAN_BROWN() -> Color {
        Color { r: 0.6705882352941176, g: 0.49411764705882355, b: 0.2980392156862745, a: 1.0 }
    }

    /// DARKBLUE | #030764 | rgb(0.012, 0.027, 0.392)
    #[classattr]
    pub fn DARKBLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.027450980392156862, b: 0.39215686274509803, a: 1.0 }
    }

    /// ROSA | #FE86A4 | rgb(0.996, 0.525, 0.643)
    #[classattr]
    pub fn ROSA() -> Color {
        Color { r: 0.996078431372549, g: 0.5254901960784314, b: 0.6431372549019608, a: 1.0 }
    }

    /// LIPSTICK | #D5174E | rgb(0.835, 0.090, 0.306)
    #[classattr]
    pub fn LIPSTICK() -> Color {
        Color { r: 0.8352941176470589, g: 0.09019607843137255, b: 0.3058823529411765, a: 1.0 }
    }

    /// PALE_MAUVE | #FED0FC | rgb(0.996, 0.816, 0.988)
    #[classattr]
    pub fn PALE_MAUVE() -> Color {
        Color { r: 0.996078431372549, g: 0.8156862745098039, b: 0.9882352941176471, a: 1.0 }
    }

    /// CLARET | #680018 | rgb(0.408, 0.000, 0.094)
    #[classattr]
    pub fn CLARET() -> Color {
        Color { r: 0.40784313725490196, g: 0.0, b: 0.09411764705882353, a: 1.0 }
    }

    /// DANDELION | #FEDF08 | rgb(0.996, 0.875, 0.031)
    #[classattr]
    pub fn DANDELION() -> Color {
        Color { r: 0.996078431372549, g: 0.8745098039215686, b: 0.03137254901960784, a: 1.0 }
    }

    /// ORANGERED | #FE420F | rgb(0.996, 0.259, 0.059)
    #[classattr]
    pub fn ORANGERED() -> Color {
        Color { r: 0.996078431372549, g: 0.25882352941176473, b: 0.058823529411764705, a: 1.0 }
    }

    /// POOP_GREEN | #6F7C00 | rgb(0.435, 0.486, 0.000)
    #[classattr]
    pub fn POOP_GREEN() -> Color {
        Color { r: 0.43529411764705883, g: 0.48627450980392156, b: 0.0, a: 1.0 }
    }

    /// RUBY | #CA0147 | rgb(0.792, 0.004, 0.278)
    #[classattr]
    pub fn RUBY() -> Color {
        Color { r: 0.792156862745098, g: 0.00392156862745098, b: 0.2784313725490196, a: 1.0 }
    }

    /// DARK | #1B2431 | rgb(0.106, 0.141, 0.192)
    #[classattr]
    pub fn DARK() -> Color {
        Color { r: 0.10588235294117647, g: 0.1411764705882353, b: 0.19215686274509805, a: 1.0 }
    }

    /// GREENISH_TURQUOISE | #00FBB0 | rgb(0.000, 0.984, 0.690)
    #[classattr]
    pub fn GREENISH_TURQUOISE() -> Color {
        Color { r: 0.0, g: 0.984313725490196, b: 0.6901960784313725, a: 1.0 }
    }

    /// PASTEL_RED | #DB5856 | rgb(0.859, 0.345, 0.337)
    #[classattr]
    pub fn PASTEL_RED() -> Color {
        Color { r: 0.8588235294117647, g: 0.34509803921568627, b: 0.33725490196078434, a: 1.0 }
    }

    /// PISS_YELLOW | #DDD618 | rgb(0.867, 0.839, 0.094)
    #[classattr]
    pub fn PISS_YELLOW() -> Color {
        Color { r: 0.8666666666666667, g: 0.8392156862745098, b: 0.09411764705882353, a: 1.0 }
    }

    /// BRIGHT_CYAN | #41FDFE | rgb(0.255, 0.992, 0.996)
    #[classattr]
    pub fn BRIGHT_CYAN() -> Color {
        Color { r: 0.2549019607843137, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 }
    }

    /// DARK_CORAL | #CF524E | rgb(0.812, 0.322, 0.306)
    #[classattr]
    pub fn DARK_CORAL() -> Color {
        Color { r: 0.8117647058823529, g: 0.3215686274509804, b: 0.3058823529411765, a: 1.0 }
    }

    /// ALGAE_GREEN | #21C36F | rgb(0.129, 0.765, 0.435)
    #[classattr]
    pub fn ALGAE_GREEN() -> Color {
        Color { r: 0.12941176470588237, g: 0.7647058823529411, b: 0.43529411764705883, a: 1.0 }
    }

    /// DARKISH_RED | #A90308 | rgb(0.663, 0.012, 0.031)
    #[classattr]
    pub fn DARKISH_RED() -> Color {
        Color { r: 0.6627450980392157, g: 0.011764705882352941, b: 0.03137254901960784, a: 1.0 }
    }

    /// REDDY_BROWN | #6E1005 | rgb(0.431, 0.063, 0.020)
    #[classattr]
    pub fn REDDY_BROWN() -> Color {
        Color { r: 0.43137254901960786, g: 0.06274509803921569, b: 0.0196078431372549, a: 1.0 }
    }

    /// BLUSH_PINK | #FE828C | rgb(0.996, 0.510, 0.549)
    #[classattr]
    pub fn BLUSH_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.5098039215686274, b: 0.5490196078431373, a: 1.0 }
    }

    /// CAMOUFLAGE_GREEN | #4B6113 | rgb(0.294, 0.380, 0.075)
    #[classattr]
    pub fn CAMOUFLAGE_GREEN() -> Color {
        Color { r: 0.29411764705882354, g: 0.3803921568627451, b: 0.07450980392156863, a: 1.0 }
    }

    /// LAWN_GREEN | #4DA409 | rgb(0.302, 0.643, 0.035)
    #[classattr]
    pub fn LAWN_GREEN() -> Color {
        Color { r: 0.30196078431372547, g: 0.6431372549019608, b: 0.03529411764705882, a: 1.0 }
    }

    /// PUTTY | #BEAE8A | rgb(0.745, 0.682, 0.541)
    #[classattr]
    pub fn PUTTY() -> Color {
        Color { r: 0.7450980392156863, g: 0.6823529411764706, b: 0.5411764705882353, a: 1.0 }
    }

    /// VIBRANT_BLUE | #0339F8 | rgb(0.012, 0.224, 0.973)
    #[classattr]
    pub fn VIBRANT_BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.2235294117647059, b: 0.9725490196078431, a: 1.0 }
    }

    /// DARK_SAND | #A88F59 | rgb(0.659, 0.561, 0.349)
    #[classattr]
    pub fn DARK_SAND() -> Color {
        Color { r: 0.6588235294117647, g: 0.5607843137254902, b: 0.34901960784313724, a: 1.0 }
    }

    /// PURPLE_BLUE | #5D21D0 | rgb(0.365, 0.129, 0.816)
    #[classattr]
    pub fn PURPLE_BLUE() -> Color {
        Color { r: 0.36470588235294116, g: 0.12941176470588237, b: 0.8156862745098039, a: 1.0 }
    }

    /// SAFFRON | #FEB209 | rgb(0.996, 0.698, 0.035)
    #[classattr]
    pub fn SAFFRON() -> Color {
        Color { r: 0.996078431372549, g: 0.6980392156862745, b: 0.03529411764705882, a: 1.0 }
    }

    /// TWILIGHT | #4E518B | rgb(0.306, 0.318, 0.545)
    #[classattr]
    pub fn TWILIGHT() -> Color {
        Color { r: 0.3058823529411765, g: 0.3176470588235294, b: 0.5450980392156862, a: 1.0 }
    }

    /// WARM_BROWN | #964E02 | rgb(0.588, 0.306, 0.008)
    #[classattr]
    pub fn WARM_BROWN() -> Color {
        Color { r: 0.5882352941176471, g: 0.3058823529411765, b: 0.00784313725490196, a: 1.0 }
    }

    /// BLUEGREY | #85A3B2 | rgb(0.522, 0.639, 0.698)
    #[classattr]
    pub fn BLUEGREY() -> Color {
        Color { r: 0.5215686274509804, g: 0.6392156862745098, b: 0.6980392156862745, a: 1.0 }
    }

    /// BUBBLE_GUM_PINK | #FF69AF | rgb(1.000, 0.412, 0.686)
    #[classattr]
    pub fn BUBBLE_GUM_PINK() -> Color {
        Color { r: 1.0, g: 0.4117647058823529, b: 0.6862745098039216, a: 1.0 }
    }

    /// DUCK_EGG_BLUE | #C3FBF4 | rgb(0.765, 0.984, 0.957)
    #[classattr]
    pub fn DUCK_EGG_BLUE() -> Color {
        Color { r: 0.7647058823529411, g: 0.984313725490196, b: 0.9568627450980393, a: 1.0 }
    }

    /// GREENISH_CYAN | #2AFEB7 | rgb(0.165, 0.996, 0.718)
    #[classattr]
    pub fn GREENISH_CYAN() -> Color {
        Color { r: 0.16470588235294117, g: 0.996078431372549, b: 0.7176470588235294, a: 1.0 }
    }

    /// PETROL | #005F6A | rgb(0.000, 0.373, 0.416)
    #[classattr]
    pub fn PETROL() -> Color {
        Color { r: 0.0, g: 0.37254901960784315, b: 0.41568627450980394, a: 1.0 }
    }

    /// ROYAL | #0C1793 | rgb(0.047, 0.090, 0.576)
    #[classattr]
    pub fn ROYAL() -> Color {
        Color { r: 0.047058823529411764, g: 0.09019607843137255, b: 0.5764705882352941, a: 1.0 }
    }

    /// BUTTER | #FFFF81 | rgb(1.000, 1.000, 0.506)
    #[classattr]
    pub fn BUTTER() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.5058823529411764, a: 1.0 }
    }

    /// DUSTY_ORANGE | #F0833A | rgb(0.941, 0.514, 0.227)
    #[classattr]
    pub fn DUSTY_ORANGE() -> Color {
        Color { r: 0.9411764705882353, g: 0.5137254901960784, b: 0.22745098039215686, a: 1.0 }
    }

    /// OFF_YELLOW | #F1F33F | rgb(0.945, 0.953, 0.247)
    #[classattr]
    pub fn OFF_YELLOW() -> Color {
        Color { r: 0.9450980392156862, g: 0.9529411764705882, b: 0.24705882352941178, a: 1.0 }
    }

    /// PALE_OLIVE_GREEN | #B1D27B | rgb(0.694, 0.824, 0.482)
    #[classattr]
    pub fn PALE_OLIVE_GREEN() -> Color {
        Color { r: 0.6941176470588235, g: 0.8235294117647058, b: 0.4823529411764706, a: 1.0 }
    }

    /// ORANGISH | #FC824A | rgb(0.988, 0.510, 0.290)
    #[classattr]
    pub fn ORANGISH() -> Color {
        Color { r: 0.9882352941176471, g: 0.5098039215686274, b: 0.2901960784313726, a: 1.0 }
    }

    /// LEAF | #71AA34 | rgb(0.443, 0.667, 0.204)
    #[classattr]
    pub fn LEAF() -> Color {
        Color { r: 0.44313725490196076, g: 0.6666666666666666, b: 0.20392156862745098, a: 1.0 }
    }

    /// LIGHT_BLUE_GREY | #B7C9E2 | rgb(0.718, 0.788, 0.886)
    #[classattr]
    pub fn LIGHT_BLUE_GREY() -> Color {
        Color { r: 0.7176470588235294, g: 0.788235294117647, b: 0.8862745098039215, a: 1.0 }
    }

    /// DRIED_BLOOD | #4B0101 | rgb(0.294, 0.004, 0.004)
    #[classattr]
    pub fn DRIED_BLOOD() -> Color {
        Color { r: 0.29411764705882354, g: 0.00392156862745098, b: 0.00392156862745098, a: 1.0 }
    }

    /// LIGHTISH_PURPLE | #A552E6 | rgb(0.647, 0.322, 0.902)
    #[classattr]
    pub fn LIGHTISH_PURPLE() -> Color {
        Color { r: 0.6470588235294118, g: 0.3215686274509804, b: 0.9019607843137255, a: 1.0 }
    }

    /// RUSTY_RED | #AF2F0D | rgb(0.686, 0.184, 0.051)
    #[classattr]
    pub fn RUSTY_RED() -> Color {
        Color { r: 0.6862745098039216, g: 0.1843137254901961, b: 0.050980392156862744, a: 1.0 }
    }

    /// LAVENDER_BLUE | #8B88F8 | rgb(0.545, 0.533, 0.973)
    #[classattr]
    pub fn LAVENDER_BLUE() -> Color {
        Color { r: 0.5450980392156862, g: 0.5333333333333333, b: 0.9725490196078431, a: 1.0 }
    }

    /// LIGHT_GRASS_GREEN | #9AF764 | rgb(0.604, 0.969, 0.392)
    #[classattr]
    pub fn LIGHT_GRASS_GREEN() -> Color {
        Color { r: 0.6039215686274509, g: 0.9686274509803922, b: 0.39215686274509803, a: 1.0 }
    }

    /// LIGHT_MINT_GREEN | #A6FBB2 | rgb(0.651, 0.984, 0.698)
    #[classattr]
    pub fn LIGHT_MINT_GREEN() -> Color {
        Color { r: 0.6509803921568628, g: 0.984313725490196, b: 0.6980392156862745, a: 1.0 }
    }

    /// SUNFLOWER | #FFC512 | rgb(1.000, 0.773, 0.071)
    #[classattr]
    pub fn SUNFLOWER() -> Color {
        Color { r: 1.0, g: 0.7725490196078432, b: 0.07058823529411765, a: 1.0 }
    }

    /// VELVET | #750851 | rgb(0.459, 0.031, 0.318)
    #[classattr]
    pub fn VELVET() -> Color {
        Color { r: 0.4588235294117647, g: 0.03137254901960784, b: 0.3176470588235294, a: 1.0 }
    }

    /// BRICK_ORANGE | #C14A09 | rgb(0.757, 0.290, 0.035)
    #[classattr]
    pub fn BRICK_ORANGE() -> Color {
        Color { r: 0.7568627450980392, g: 0.2901960784313726, b: 0.03529411764705882, a: 1.0 }
    }

    /// LIGHTISH_RED | #FE2F4A | rgb(0.996, 0.184, 0.290)
    #[classattr]
    pub fn LIGHTISH_RED() -> Color {
        Color { r: 0.996078431372549, g: 0.1843137254901961, b: 0.2901960784313726, a: 1.0 }
    }

    /// PURE_BLUE | #0203E2 | rgb(0.008, 0.012, 0.886)
    #[classattr]
    pub fn PURE_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.011764705882352941, b: 0.8862745098039215, a: 1.0 }
    }

    /// TWILIGHT_BLUE | #0A437A | rgb(0.039, 0.263, 0.478)
    #[classattr]
    pub fn TWILIGHT_BLUE() -> Color {
        Color { r: 0.0392156862745098, g: 0.2627450980392157, b: 0.47843137254901963, a: 1.0 }
    }

    /// VIOLET_RED | #A50055 | rgb(0.647, 0.000, 0.333)
    #[classattr]
    pub fn VIOLET_RED() -> Color {
        Color { r: 0.6470588235294118, g: 0.0, b: 0.3333333333333333, a: 1.0 }
    }

    /// YELLOWY_BROWN | #AE8B0C | rgb(0.682, 0.545, 0.047)
    #[classattr]
    pub fn YELLOWY_BROWN() -> Color {
        Color { r: 0.6823529411764706, g: 0.5450980392156862, b: 0.047058823529411764, a: 1.0 }
    }

    /// CARNATION | #FD798F | rgb(0.992, 0.475, 0.561)
    #[classattr]
    pub fn CARNATION() -> Color {
        Color { r: 0.9921568627450981, g: 0.4745098039215686, b: 0.5607843137254902, a: 1.0 }
    }

    /// MUDDY_YELLOW | #BFAC05 | rgb(0.749, 0.675, 0.020)
    #[classattr]
    pub fn MUDDY_YELLOW() -> Color {
        Color { r: 0.7490196078431373, g: 0.6745098039215687, b: 0.0196078431372549, a: 1.0 }
    }

    /// DARK_SEAFOAM_GREEN | #3EAF76 | rgb(0.243, 0.686, 0.463)
    #[classattr]
    pub fn DARK_SEAFOAM_GREEN() -> Color {
        Color { r: 0.24313725490196078, g: 0.6862745098039216, b: 0.4627450980392157, a: 1.0 }
    }

    /// DEEP_ROSE | #C74767 | rgb(0.780, 0.278, 0.404)
    #[classattr]
    pub fn DEEP_ROSE() -> Color {
        Color { r: 0.7803921568627451, g: 0.2784313725490196, b: 0.403921568627451, a: 1.0 }
    }

    /// DUSTY_RED | #B9484E | rgb(0.725, 0.282, 0.306)
    #[classattr]
    pub fn DUSTY_RED() -> Color {
        Color { r: 0.7254901960784313, g: 0.2823529411764706, b: 0.3058823529411765, a: 1.0 }
    }

    /// GREY_BLUE | #647D8E | rgb(0.392, 0.490, 0.557)
    #[classattr]
    pub fn GREY_BLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.49019607843137253, b: 0.5568627450980392, a: 1.0 }
    }

    /// LEMON_LIME | #BFFE28 | rgb(0.749, 0.996, 0.157)
    #[classattr]
    pub fn LEMON_LIME() -> Color {
        Color { r: 0.7490196078431373, g: 0.996078431372549, b: 0.1568627450980392, a: 1.0 }
    }

    /// PURPLE_PINK | #D725DE | rgb(0.843, 0.145, 0.871)
    #[classattr]
    pub fn PURPLE_PINK() -> Color {
        Color { r: 0.8431372549019608, g: 0.1450980392156863, b: 0.8705882352941177, a: 1.0 }
    }

    /// BROWN_YELLOW | #B29705 | rgb(0.698, 0.592, 0.020)
    #[classattr]
    pub fn BROWN_YELLOW() -> Color {
        Color { r: 0.6980392156862745, g: 0.592156862745098, b: 0.0196078431372549, a: 1.0 }
    }

    /// PURPLE_BROWN | #673A3F | rgb(0.404, 0.227, 0.247)
    #[classattr]
    pub fn PURPLE_BROWN() -> Color {
        Color { r: 0.403921568627451, g: 0.22745098039215686, b: 0.24705882352941178, a: 1.0 }
    }

    /// WISTERIA | #A87DC2 | rgb(0.659, 0.490, 0.761)
    #[classattr]
    pub fn WISTERIA() -> Color {
        Color { r: 0.6588235294117647, g: 0.49019607843137253, b: 0.7607843137254902, a: 1.0 }
    }

    /// BANANA_YELLOW | #FAFE4B | rgb(0.980, 0.996, 0.294)
    #[classattr]
    pub fn BANANA_YELLOW() -> Color {
        Color { r: 0.9803921568627451, g: 0.996078431372549, b: 0.29411764705882354, a: 1.0 }
    }

    /// LIPSTICK_RED | #C0022F | rgb(0.753, 0.008, 0.184)
    #[classattr]
    pub fn LIPSTICK_RED() -> Color {
        Color { r: 0.7529411764705882, g: 0.00784313725490196, b: 0.1843137254901961, a: 1.0 }
    }

    /// WATER_BLUE | #0E87CC | rgb(0.055, 0.529, 0.800)
    #[classattr]
    pub fn WATER_BLUE() -> Color {
        Color { r: 0.054901960784313725, g: 0.5294117647058824, b: 0.8, a: 1.0 }
    }

    /// BROWN_GREY | #8D8468 | rgb(0.553, 0.518, 0.408)
    #[classattr]
    pub fn BROWN_GREY() -> Color {
        Color { r: 0.5529411764705883, g: 0.5176470588235295, b: 0.40784313725490196, a: 1.0 }
    }

    /// VIBRANT_PURPLE | #AD03DE | rgb(0.678, 0.012, 0.871)
    #[classattr]
    pub fn VIBRANT_PURPLE() -> Color {
        Color { r: 0.6784313725490196, g: 0.011764705882352941, b: 0.8705882352941177, a: 1.0 }
    }

    /// BABY_GREEN | #8CFF9E | rgb(0.549, 1.000, 0.620)
    #[classattr]
    pub fn BABY_GREEN() -> Color {
        Color { r: 0.5490196078431373, g: 1.0, b: 0.6196078431372549, a: 1.0 }
    }

    /// BARF_GREEN | #94AC02 | rgb(0.580, 0.675, 0.008)
    #[classattr]
    pub fn BARF_GREEN() -> Color {
        Color { r: 0.5803921568627451, g: 0.6745098039215687, b: 0.00784313725490196, a: 1.0 }
    }

    /// EGGSHELL_BLUE | #C4FFF7 | rgb(0.769, 1.000, 0.969)
    #[classattr]
    pub fn EGGSHELL_BLUE() -> Color {
        Color { r: 0.7686274509803922, g: 1.0, b: 0.9686274509803922, a: 1.0 }
    }

    /// SANDY_YELLOW | #FDEE73 | rgb(0.992, 0.933, 0.451)
    #[classattr]
    pub fn SANDY_YELLOW() -> Color {
        Color { r: 0.9921568627450981, g: 0.9333333333333333, b: 0.45098039215686275, a: 1.0 }
    }

    /// COOL_GREEN | #33B864 | rgb(0.200, 0.722, 0.392)
    #[classattr]
    pub fn COOL_GREEN() -> Color {
        Color { r: 0.2, g: 0.7215686274509804, b: 0.39215686274509803, a: 1.0 }
    }

    /// PALE | #FFF9D0 | rgb(1.000, 0.976, 0.816)
    #[classattr]
    pub fn PALE() -> Color {
        Color { r: 1.0, g: 0.9764705882352941, b: 0.8156862745098039, a: 1.0 }
    }

    /// BLUE_GREY | #758DA3 | rgb(0.459, 0.553, 0.639)
    #[classattr]
    pub fn BLUE_GREY() -> Color {
        Color { r: 0.4588235294117647, g: 0.5529411764705883, b: 0.6392156862745098, a: 1.0 }
    }

    /// HOT_MAGENTA | #F504C9 | rgb(0.961, 0.016, 0.788)
    #[classattr]
    pub fn HOT_MAGENTA() -> Color {
        Color { r: 0.9607843137254902, g: 0.01568627450980392, b: 0.788235294117647, a: 1.0 }
    }

    /// GREYBLUE | #77A1B5 | rgb(0.467, 0.631, 0.710)
    #[classattr]
    pub fn GREYBLUE() -> Color {
        Color { r: 0.4666666666666667, g: 0.6313725490196078, b: 0.7098039215686275, a: 1.0 }
    }

    /// PURPLEY | #8756E4 | rgb(0.529, 0.337, 0.894)
    #[classattr]
    pub fn PURPLEY() -> Color {
        Color { r: 0.5294117647058824, g: 0.33725490196078434, b: 0.8941176470588236, a: 1.0 }
    }

    /// BABY_SHIT_GREEN | #889717 | rgb(0.533, 0.592, 0.090)
    #[classattr]
    pub fn BABY_SHIT_GREEN() -> Color {
        Color { r: 0.5333333333333333, g: 0.592156862745098, b: 0.09019607843137255, a: 1.0 }
    }

    /// BROWNISH_PINK | #C27E79 | rgb(0.761, 0.494, 0.475)
    #[classattr]
    pub fn BROWNISH_PINK() -> Color {
        Color { r: 0.7607843137254902, g: 0.49411764705882355, b: 0.4745098039215686, a: 1.0 }
    }

    /// DARK_AQUAMARINE | #017371 | rgb(0.004, 0.451, 0.443)
    #[classattr]
    pub fn DARK_AQUAMARINE() -> Color {
        Color { r: 0.00392156862745098, g: 0.45098039215686275, b: 0.44313725490196076, a: 1.0 }
    }

    /// DIARRHEA | #9F8303 | rgb(0.624, 0.514, 0.012)
    #[classattr]
    pub fn DIARRHEA() -> Color {
        Color { r: 0.6235294117647059, g: 0.5137254901960784, b: 0.011764705882352941, a: 1.0 }
    }

    /// LIGHT_MUSTARD | #F7D560 | rgb(0.969, 0.835, 0.376)
    #[classattr]
    pub fn LIGHT_MUSTARD() -> Color {
        Color { r: 0.9686274509803922, g: 0.8352941176470589, b: 0.3764705882352941, a: 1.0 }
    }

    /// PALE_SKY_BLUE | #BDF6FE | rgb(0.741, 0.965, 0.996)
    #[classattr]
    pub fn PALE_SKY_BLUE() -> Color {
        Color { r: 0.7411764705882353, g: 0.9647058823529412, b: 0.996078431372549, a: 1.0 }
    }

    /// TURTLE_GREEN | #75B84F | rgb(0.459, 0.722, 0.310)
    #[classattr]
    pub fn TURTLE_GREEN() -> Color {
        Color { r: 0.4588235294117647, g: 0.7215686274509804, b: 0.30980392156862746, a: 1.0 }
    }

    /// BRIGHT_OLIVE | #9CBB04 | rgb(0.612, 0.733, 0.016)
    #[classattr]
    pub fn BRIGHT_OLIVE() -> Color {
        Color { r: 0.611764705882353, g: 0.7333333333333333, b: 0.01568627450980392, a: 1.0 }
    }

    /// DARK_GREY_BLUE | #29465B | rgb(0.161, 0.275, 0.357)
    #[classattr]
    pub fn DARK_GREY_BLUE() -> Color {
        Color { r: 0.1607843137254902, g: 0.27450980392156865, b: 0.3568627450980392, a: 1.0 }
    }

    /// GREENY_BROWN | #696006 | rgb(0.412, 0.376, 0.024)
    #[classattr]
    pub fn GREENY_BROWN() -> Color {
        Color { r: 0.4117647058823529, g: 0.3764705882352941, b: 0.023529411764705882, a: 1.0 }
    }

    /// LEMON_GREEN | #ADF802 | rgb(0.678, 0.973, 0.008)
    #[classattr]
    pub fn LEMON_GREEN() -> Color {
        Color { r: 0.6784313725490196, g: 0.9725490196078431, b: 0.00784313725490196, a: 1.0 }
    }

    /// LIGHT_PERIWINKLE | #C1C6FC | rgb(0.757, 0.776, 0.988)
    #[classattr]
    pub fn LIGHT_PERIWINKLE() -> Color {
        Color { r: 0.7568627450980392, g: 0.7764705882352941, b: 0.9882352941176471, a: 1.0 }
    }

    /// SEAWEED_GREEN | #35AD6B | rgb(0.208, 0.678, 0.420)
    #[classattr]
    pub fn SEAWEED_GREEN() -> Color {
        Color { r: 0.20784313725490197, g: 0.6784313725490196, b: 0.4196078431372549, a: 1.0 }
    }

    /// SUNSHINE_YELLOW | #FFFD37 | rgb(1.000, 0.992, 0.216)
    #[classattr]
    pub fn SUNSHINE_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.21568627450980393, a: 1.0 }
    }

    /// UGLY_PURPLE | #A442A0 | rgb(0.643, 0.259, 0.627)
    #[classattr]
    pub fn UGLY_PURPLE() -> Color {
        Color { r: 0.6431372549019608, g: 0.25882352941176473, b: 0.6274509803921569, a: 1.0 }
    }

    /// MEDIUM_PINK | #F36196 | rgb(0.953, 0.380, 0.588)
    #[classattr]
    pub fn MEDIUM_PINK() -> Color {
        Color { r: 0.9529411764705882, g: 0.3803921568627451, b: 0.5882352941176471, a: 1.0 }
    }

    /// PUKE_BROWN | #947706 | rgb(0.580, 0.467, 0.024)
    #[classattr]
    pub fn PUKE_BROWN() -> Color {
        Color { r: 0.5803921568627451, g: 0.4666666666666667, b: 0.023529411764705882, a: 1.0 }
    }

    /// VERY_LIGHT_PINK | #FFF4F2 | rgb(1.000, 0.957, 0.949)
    #[classattr]
    pub fn VERY_LIGHT_PINK() -> Color {
        Color { r: 1.0, g: 0.9568627450980393, b: 0.9490196078431372, a: 1.0 }
    }

    /// VIRIDIAN | #1E9167 | rgb(0.118, 0.569, 0.404)
    #[classattr]
    pub fn VIRIDIAN() -> Color {
        Color { r: 0.11764705882352941, g: 0.5686274509803921, b: 0.403921568627451, a: 1.0 }
    }

    /// BILE | #B5C306 | rgb(0.710, 0.765, 0.024)
    #[classattr]
    pub fn BILE() -> Color {
        Color { r: 0.7098039215686275, g: 0.7647058823529411, b: 0.023529411764705882, a: 1.0 }
    }

    /// FADED_YELLOW | #FEFF7F | rgb(0.996, 1.000, 0.498)
    #[classattr]
    pub fn FADED_YELLOW() -> Color {
        Color { r: 0.996078431372549, g: 1.0, b: 0.4980392156862745, a: 1.0 }
    }

    /// VERY_PALE_GREEN | #CFFDBC | rgb(0.812, 0.992, 0.737)
    #[classattr]
    pub fn VERY_PALE_GREEN() -> Color {
        Color { r: 0.8117647058823529, g: 0.9921568627450981, b: 0.7372549019607844, a: 1.0 }
    }

    /// VIBRANT_GREEN | #0ADD08 | rgb(0.039, 0.867, 0.031)
    #[classattr]
    pub fn VIBRANT_GREEN() -> Color {
        Color { r: 0.0392156862745098, g: 0.8666666666666667, b: 0.03137254901960784, a: 1.0 }
    }

    /// BRIGHT_LIME | #87FD05 | rgb(0.529, 0.992, 0.020)
    #[classattr]
    pub fn BRIGHT_LIME() -> Color {
        Color { r: 0.5294117647058824, g: 0.9921568627450981, b: 0.0196078431372549, a: 1.0 }
    }

    /// SPEARMINT | #1EF876 | rgb(0.118, 0.973, 0.463)
    #[classattr]
    pub fn SPEARMINT() -> Color {
        Color { r: 0.11764705882352941, g: 0.9725490196078431, b: 0.4627450980392157, a: 1.0 }
    }

    /// LIGHT_AQUAMARINE | #7BFDC7 | rgb(0.482, 0.992, 0.780)
    #[classattr]
    pub fn LIGHT_AQUAMARINE() -> Color {
        Color { r: 0.4823529411764706, g: 0.9921568627450981, b: 0.7803921568627451, a: 1.0 }
    }

    /// LIGHT_SAGE | #BCECAC | rgb(0.737, 0.925, 0.675)
    #[classattr]
    pub fn LIGHT_SAGE() -> Color {
        Color { r: 0.7372549019607844, g: 0.9254901960784314, b: 0.6745098039215687, a: 1.0 }
    }

    /// YELLOWGREEN | #BBF90F | rgb(0.733, 0.976, 0.059)
    #[classattr]
    pub fn YELLOWGREEN() -> Color {
        Color { r: 0.7333333333333333, g: 0.9764705882352941, b: 0.058823529411764705, a: 1.0 }
    }

    /// BABY_POO | #AB9004 | rgb(0.671, 0.565, 0.016)
    #[classattr]
    pub fn BABY_POO() -> Color {
        Color { r: 0.6705882352941176, g: 0.5647058823529412, b: 0.01568627450980392, a: 1.0 }
    }

    /// DARK_SEAFOAM | #1FB57A | rgb(0.122, 0.710, 0.478)
    #[classattr]
    pub fn DARK_SEAFOAM() -> Color {
        Color { r: 0.12156862745098039, g: 0.7098039215686275, b: 0.47843137254901963, a: 1.0 }
    }

    /// DEEP_TEAL | #00555A | rgb(0.000, 0.333, 0.353)
    #[classattr]
    pub fn DEEP_TEAL() -> Color {
        Color { r: 0.0, g: 0.3333333333333333, b: 0.35294117647058826, a: 1.0 }
    }

    /// HEATHER | #A484AC | rgb(0.643, 0.518, 0.675)
    #[classattr]
    pub fn HEATHER() -> Color {
        Color { r: 0.6431372549019608, g: 0.5176470588235295, b: 0.6745098039215687, a: 1.0 }
    }

    /// RUST_ORANGE | #C45508 | rgb(0.769, 0.333, 0.031)
    #[classattr]
    pub fn RUST_ORANGE() -> Color {
        Color { r: 0.7686274509803922, g: 0.3333333333333333, b: 0.03137254901960784, a: 1.0 }
    }

    /// DIRTY_BLUE | #3F829D | rgb(0.247, 0.510, 0.616)
    #[classattr]
    pub fn DIRTY_BLUE() -> Color {
        Color { r: 0.24705882352941178, g: 0.5098039215686274, b: 0.615686274509804, a: 1.0 }
    }

    /// FERN_GREEN | #548D44 | rgb(0.329, 0.553, 0.267)
    #[classattr]
    pub fn FERN_GREEN() -> Color {
        Color { r: 0.32941176470588235, g: 0.5529411764705883, b: 0.26666666666666666, a: 1.0 }
    }

    /// BRIGHT_LILAC | #C95EFB | rgb(0.788, 0.369, 0.984)
    #[classattr]
    pub fn BRIGHT_LILAC() -> Color {
        Color { r: 0.788235294117647, g: 0.3686274509803922, b: 0.984313725490196, a: 1.0 }
    }

    /// WEIRD_GREEN | #3AE57F | rgb(0.227, 0.898, 0.498)
    #[classattr]
    pub fn WEIRD_GREEN() -> Color {
        Color { r: 0.22745098039215686, g: 0.8980392156862745, b: 0.4980392156862745, a: 1.0 }
    }

    /// PEACOCK_BLUE | #016795 | rgb(0.004, 0.404, 0.584)
    #[classattr]
    pub fn PEACOCK_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.403921568627451, b: 0.5843137254901961, a: 1.0 }
    }

    /// AVOCADO_GREEN | #87A922 | rgb(0.529, 0.663, 0.133)
    #[classattr]
    pub fn AVOCADO_GREEN() -> Color {
        Color { r: 0.5294117647058824, g: 0.6627450980392157, b: 0.13333333333333333, a: 1.0 }
    }

    /// FADED_ORANGE | #F0944D | rgb(0.941, 0.580, 0.302)
    #[classattr]
    pub fn FADED_ORANGE() -> Color {
        Color { r: 0.9411764705882353, g: 0.5803921568627451, b: 0.30196078431372547, a: 1.0 }
    }

    /// GRAPE_PURPLE | #5D1451 | rgb(0.365, 0.078, 0.318)
    #[classattr]
    pub fn GRAPE_PURPLE() -> Color {
        Color { r: 0.36470588235294116, g: 0.0784313725490196, b: 0.3176470588235294, a: 1.0 }
    }

    /// HOT_GREEN | #25FF29 | rgb(0.145, 1.000, 0.161)
    #[classattr]
    pub fn HOT_GREEN() -> Color {
        Color { r: 0.1450980392156863, g: 1.0, b: 0.1607843137254902, a: 1.0 }
    }

    /// LIME_YELLOW | #D0FE1D | rgb(0.816, 0.996, 0.114)
    #[classattr]
    pub fn LIME_YELLOW() -> Color {
        Color { r: 0.8156862745098039, g: 0.996078431372549, b: 0.11372549019607843, a: 1.0 }
    }

    /// MANGO | #FFA62B | rgb(1.000, 0.651, 0.169)
    #[classattr]
    pub fn MANGO() -> Color {
        Color { r: 1.0, g: 0.6509803921568628, b: 0.16862745098039217, a: 1.0 }
    }

    /// SHAMROCK | #01B44C | rgb(0.004, 0.706, 0.298)
    #[classattr]
    pub fn SHAMROCK() -> Color {
        Color { r: 0.00392156862745098, g: 0.7058823529411765, b: 0.2980392156862745, a: 1.0 }
    }

    /// BUBBLEGUM | #FF6CB5 | rgb(1.000, 0.424, 0.710)
    #[classattr]
    pub fn BUBBLEGUM() -> Color {
        Color { r: 1.0, g: 0.4235294117647059, b: 0.7098039215686275, a: 1.0 }
    }

    /// PURPLISH_BROWN | #6B4247 | rgb(0.420, 0.259, 0.278)
    #[classattr]
    pub fn PURPLISH_BROWN() -> Color {
        Color { r: 0.4196078431372549, g: 0.25882352941176473, b: 0.2784313725490196, a: 1.0 }
    }

    /// VOMIT_YELLOW | #C7C10C | rgb(0.780, 0.757, 0.047)
    #[classattr]
    pub fn VOMIT_YELLOW() -> Color {
        Color { r: 0.7803921568627451, g: 0.7568627450980392, b: 0.047058823529411764, a: 1.0 }
    }

    /// PALE_CYAN | #B7FFFA | rgb(0.718, 1.000, 0.980)
    #[classattr]
    pub fn PALE_CYAN() -> Color {
        Color { r: 0.7176470588235294, g: 1.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// KEY_LIME | #AEFF6E | rgb(0.682, 1.000, 0.431)
    #[classattr]
    pub fn KEY_LIME() -> Color {
        Color { r: 0.6823529411764706, g: 1.0, b: 0.43137254901960786, a: 1.0 }
    }

    /// TOMATO_RED | #EC2D01 | rgb(0.925, 0.176, 0.004)
    #[classattr]
    pub fn TOMATO_RED() -> Color {
        Color { r: 0.9254901960784314, g: 0.17647058823529413, b: 0.00392156862745098, a: 1.0 }
    }

    /// LIGHTGREEN | #76FF7B | rgb(0.463, 1.000, 0.482)
    #[classattr]
    pub fn LIGHTGREEN() -> Color {
        Color { r: 0.4627450980392157, g: 1.0, b: 0.4823529411764706, a: 1.0 }
    }

    /// MERLOT | #730039 | rgb(0.451, 0.000, 0.224)
    #[classattr]
    pub fn MERLOT() -> Color {
        Color { r: 0.45098039215686275, g: 0.0, b: 0.2235294117647059, a: 1.0 }
    }

    /// NIGHT_BLUE | #040348 | rgb(0.016, 0.012, 0.282)
    #[classattr]
    pub fn NIGHT_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.011764705882352941, b: 0.2823529411764706, a: 1.0 }
    }

    /// PURPLEISH_PINK | #DF4EC8 | rgb(0.875, 0.306, 0.784)
    #[classattr]
    pub fn PURPLEISH_PINK() -> Color {
        Color { r: 0.8745098039215686, g: 0.3058823529411765, b: 0.7843137254901961, a: 1.0 }
    }

    /// APPLE | #6ECB3C | rgb(0.431, 0.796, 0.235)
    #[classattr]
    pub fn APPLE() -> Color {
        Color { r: 0.43137254901960786, g: 0.796078431372549, b: 0.23529411764705882, a: 1.0 }
    }

    /// BABY_POOP_GREEN | #8F9805 | rgb(0.561, 0.596, 0.020)
    #[classattr]
    pub fn BABY_POOP_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.596078431372549, b: 0.0196078431372549, a: 1.0 }
    }

    /// GREEN_APPLE | #5EDC1F | rgb(0.369, 0.863, 0.122)
    #[classattr]
    pub fn GREEN_APPLE() -> Color {
        Color { r: 0.3686274509803922, g: 0.8627450980392157, b: 0.12156862745098039, a: 1.0 }
    }

    /// HELIOTROPE | #D94FF5 | rgb(0.851, 0.310, 0.961)
    #[classattr]
    pub fn HELIOTROPE() -> Color {
        Color { r: 0.8509803921568627, g: 0.30980392156862746, b: 0.9607843137254902, a: 1.0 }
    }

    /// YELLOW_GREEN | #C8FD3D | rgb(0.784, 0.992, 0.239)
    #[classattr]
    pub fn YELLOW_GREEN() -> Color {
        Color { r: 0.7843137254901961, g: 0.9921568627450981, b: 0.23921568627450981, a: 1.0 }
    }

    /// ALMOST_BLACK | #070D0D | rgb(0.027, 0.051, 0.051)
    #[classattr]
    pub fn ALMOST_BLACK() -> Color {
        Color { r: 0.027450980392156862, g: 0.050980392156862744, b: 0.050980392156862744, a: 1.0 }
    }

    /// COOL_BLUE | #4984B8 | rgb(0.286, 0.518, 0.722)
    #[classattr]
    pub fn COOL_BLUE() -> Color {
        Color { r: 0.28627450980392155, g: 0.5176470588235295, b: 0.7215686274509804, a: 1.0 }
    }

    /// LEAFY_GREEN | #51B73B | rgb(0.318, 0.718, 0.231)
    #[classattr]
    pub fn LEAFY_GREEN() -> Color {
        Color { r: 0.3176470588235294, g: 0.7176470588235294, b: 0.23137254901960785, a: 1.0 }
    }

    /// MUSTARD_BROWN | #AC7E04 | rgb(0.675, 0.494, 0.016)
    #[classattr]
    pub fn MUSTARD_BROWN() -> Color {
        Color { r: 0.6745098039215687, g: 0.49411764705882355, b: 0.01568627450980392, a: 1.0 }
    }

    /// DUSK | #4E5481 | rgb(0.306, 0.329, 0.506)
    #[classattr]
    pub fn DUSK() -> Color {
        Color { r: 0.3058823529411765, g: 0.32941176470588235, b: 0.5058823529411764, a: 1.0 }
    }

    /// DULL_BROWN | #876E4B | rgb(0.529, 0.431, 0.294)
    #[classattr]
    pub fn DULL_BROWN() -> Color {
        Color { r: 0.5294117647058824, g: 0.43137254901960786, b: 0.29411764705882354, a: 1.0 }
    }

    /// FROG_GREEN | #58BC08 | rgb(0.345, 0.737, 0.031)
    #[classattr]
    pub fn FROG_GREEN() -> Color {
        Color { r: 0.34509803921568627, g: 0.7372549019607844, b: 0.03137254901960784, a: 1.0 }
    }

    /// VIVID_GREEN | #2FEF10 | rgb(0.184, 0.937, 0.063)
    #[classattr]
    pub fn VIVID_GREEN() -> Color {
        Color { r: 0.1843137254901961, g: 0.9372549019607843, b: 0.06274509803921569, a: 1.0 }
    }

    /// BRIGHT_LIGHT_GREEN | #2DFE54 | rgb(0.176, 0.996, 0.329)
    #[classattr]
    pub fn BRIGHT_LIGHT_GREEN() -> Color {
        Color { r: 0.17647058823529413, g: 0.996078431372549, b: 0.32941176470588235, a: 1.0 }
    }

    /// FLURO_GREEN | #0AFF02 | rgb(0.039, 1.000, 0.008)
    #[classattr]
    pub fn FLURO_GREEN() -> Color {
        Color { r: 0.0392156862745098, g: 1.0, b: 0.00784313725490196, a: 1.0 }
    }

    /// KIWI | #9CEF43 | rgb(0.612, 0.937, 0.263)
    #[classattr]
    pub fn KIWI() -> Color {
        Color { r: 0.611764705882353, g: 0.9372549019607843, b: 0.2627450980392157, a: 1.0 }
    }

    /// SEAWEED | #18D17B | rgb(0.094, 0.820, 0.482)
    #[classattr]
    pub fn SEAWEED() -> Color {
        Color { r: 0.09411764705882353, g: 0.8196078431372549, b: 0.4823529411764706, a: 1.0 }
    }

    /// NAVY_GREEN | #35530A | rgb(0.208, 0.325, 0.039)
    #[classattr]
    pub fn NAVY_GREEN() -> Color {
        Color { r: 0.20784313725490197, g: 0.3254901960784314, b: 0.0392156862745098, a: 1.0 }
    }

    /// ULTRAMARINE_BLUE | #1805DB | rgb(0.094, 0.020, 0.859)
    #[classattr]
    pub fn ULTRAMARINE_BLUE() -> Color {
        Color { r: 0.09411764705882353, g: 0.0196078431372549, b: 0.8588235294117647, a: 1.0 }
    }

    /// IRIS | #6258C4 | rgb(0.384, 0.345, 0.769)
    #[classattr]
    pub fn IRIS() -> Color {
        Color { r: 0.3843137254901961, g: 0.34509803921568627, b: 0.7686274509803922, a: 1.0 }
    }

    /// PASTEL_ORANGE | #FF964F | rgb(1.000, 0.588, 0.310)
    #[classattr]
    pub fn PASTEL_ORANGE() -> Color {
        Color { r: 1.0, g: 0.5882352941176471, b: 0.30980392156862746, a: 1.0 }
    }

    /// YELLOWISH_ORANGE | #FFAB0F | rgb(1.000, 0.671, 0.059)
    #[classattr]
    pub fn YELLOWISH_ORANGE() -> Color {
        Color { r: 1.0, g: 0.6705882352941176, b: 0.058823529411764705, a: 1.0 }
    }

    /// PERRYWINKLE | #8F8CE7 | rgb(0.561, 0.549, 0.906)
    #[classattr]
    pub fn PERRYWINKLE() -> Color {
        Color { r: 0.5607843137254902, g: 0.5490196078431373, b: 0.9058823529411765, a: 1.0 }
    }

    /// TEALISH | #24BCA8 | rgb(0.141, 0.737, 0.659)
    #[classattr]
    pub fn TEALISH() -> Color {
        Color { r: 0.1411764705882353, g: 0.7372549019607844, b: 0.6588235294117647, a: 1.0 }
    }

    /// DARK_PLUM | #3F012C | rgb(0.247, 0.004, 0.173)
    #[classattr]
    pub fn DARK_PLUM() -> Color {
        Color { r: 0.24705882352941178, g: 0.00392156862745098, b: 0.17254901960784313, a: 1.0 }
    }

    /// PEAR | #CBF85F | rgb(0.796, 0.973, 0.373)
    #[classattr]
    pub fn PEAR() -> Color {
        Color { r: 0.796078431372549, g: 0.9725490196078431, b: 0.37254901960784315, a: 1.0 }
    }

    /// PINKISH_ORANGE | #FF724C | rgb(1.000, 0.447, 0.298)
    #[classattr]
    pub fn PINKISH_ORANGE() -> Color {
        Color { r: 1.0, g: 0.4470588235294118, b: 0.2980392156862745, a: 1.0 }
    }

    /// MIDNIGHT_PURPLE | #280137 | rgb(0.157, 0.004, 0.216)
    #[classattr]
    pub fn MIDNIGHT_PURPLE() -> Color {
        Color { r: 0.1568627450980392, g: 0.00392156862745098, b: 0.21568627450980393, a: 1.0 }
    }

    /// LIGHT_URPLE | #B36FF6 | rgb(0.702, 0.435, 0.965)
    #[classattr]
    pub fn LIGHT_URPLE() -> Color {
        Color { r: 0.7019607843137254, g: 0.43529411764705883, b: 0.9647058823529412, a: 1.0 }
    }

    /// DARK_MINT | #48C072 | rgb(0.282, 0.753, 0.447)
    #[classattr]
    pub fn DARK_MINT() -> Color {
        Color { r: 0.2823529411764706, g: 0.7529411764705882, b: 0.4470588235294118, a: 1.0 }
    }

    /// GREENISH_TAN | #BCCB7A | rgb(0.737, 0.796, 0.478)
    #[classattr]
    pub fn GREENISH_TAN() -> Color {
        Color { r: 0.7372549019607844, g: 0.796078431372549, b: 0.47843137254901963, a: 1.0 }
    }

    /// LIGHT_BURGUNDY | #A8415B | rgb(0.659, 0.255, 0.357)
    #[classattr]
    pub fn LIGHT_BURGUNDY() -> Color {
        Color { r: 0.6588235294117647, g: 0.2549019607843137, b: 0.3568627450980392, a: 1.0 }
    }

    /// TURQUOISE_BLUE | #06B1C4 | rgb(0.024, 0.694, 0.769)
    #[classattr]
    pub fn TURQUOISE_BLUE() -> Color {
        Color { r: 0.023529411764705882, g: 0.6941176470588235, b: 0.7686274509803922, a: 1.0 }
    }

    /// UGLY_PINK | #CD7584 | rgb(0.804, 0.459, 0.518)
    #[classattr]
    pub fn UGLY_PINK() -> Color {
        Color { r: 0.803921568627451, g: 0.4588235294117647, b: 0.5176470588235295, a: 1.0 }
    }

    /// SANDY | #F1DA7A | rgb(0.945, 0.855, 0.478)
    #[classattr]
    pub fn SANDY() -> Color {
        Color { r: 0.9450980392156862, g: 0.8549019607843137, b: 0.47843137254901963, a: 1.0 }
    }

    /// ELECTRIC_PINK | #FF0490 | rgb(1.000, 0.016, 0.565)
    #[classattr]
    pub fn ELECTRIC_PINK() -> Color {
        Color { r: 1.0, g: 0.01568627450980392, b: 0.5647058823529412, a: 1.0 }
    }

    /// MUTED_PURPLE | #805B87 | rgb(0.502, 0.357, 0.529)
    #[classattr]
    pub fn MUTED_PURPLE() -> Color {
        Color { r: 0.5019607843137255, g: 0.3568627450980392, b: 0.5294117647058824, a: 1.0 }
    }

    /// MID_GREEN | #50A747 | rgb(0.314, 0.655, 0.278)
    #[classattr]
    pub fn MID_GREEN() -> Color {
        Color { r: 0.3137254901960784, g: 0.6549019607843137, b: 0.2784313725490196, a: 1.0 }
    }

    /// GREYISH | #A8A495 | rgb(0.659, 0.643, 0.584)
    #[classattr]
    pub fn GREYISH() -> Color {
        Color { r: 0.6588235294117647, g: 0.6431372549019608, b: 0.5843137254901961, a: 1.0 }
    }

    /// NEON_YELLOW | #CFFF04 | rgb(0.812, 1.000, 0.016)
    #[classattr]
    pub fn NEON_YELLOW() -> Color {
        Color { r: 0.8117647058823529, g: 1.0, b: 0.01568627450980392, a: 1.0 }
    }

    /// BANANA | #FFFF7E | rgb(1.000, 1.000, 0.494)
    #[classattr]
    pub fn BANANA() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.49411764705882355, a: 1.0 }
    }

    /// CARNATION_PINK | #FF7FA7 | rgb(1.000, 0.498, 0.655)
    #[classattr]
    pub fn CARNATION_PINK() -> Color {
        Color { r: 1.0, g: 0.4980392156862745, b: 0.6549019607843137, a: 1.0 }
    }

    /// TOMATO | #EF4026 | rgb(0.937, 0.251, 0.149)
    #[classattr]
    pub fn TOMATO() -> Color {
        Color { r: 0.9372549019607843, g: 0.25098039215686274, b: 0.14901960784313725, a: 1.0 }
    }

    /// SEA | #3C9992 | rgb(0.235, 0.600, 0.573)
    #[classattr]
    pub fn SEA() -> Color {
        Color { r: 0.23529411764705882, g: 0.6, b: 0.5725490196078431, a: 1.0 }
    }

    /// MUDDY_BROWN | #886806 | rgb(0.533, 0.408, 0.024)
    #[classattr]
    pub fn MUDDY_BROWN() -> Color {
        Color { r: 0.5333333333333333, g: 0.40784313725490196, b: 0.023529411764705882, a: 1.0 }
    }

    /// TURQUOISE_GREEN | #04F489 | rgb(0.016, 0.957, 0.537)
    #[classattr]
    pub fn TURQUOISE_GREEN() -> Color {
        Color { r: 0.01568627450980392, g: 0.9568627450980393, b: 0.5372549019607843, a: 1.0 }
    }

    /// BUFF | #FEF69E | rgb(0.996, 0.965, 0.620)
    #[classattr]
    pub fn BUFF() -> Color {
        Color { r: 0.996078431372549, g: 0.9647058823529412, b: 0.6196078431372549, a: 1.0 }
    }

    /// FAWN | #CFAF7B | rgb(0.812, 0.686, 0.482)
    #[classattr]
    pub fn FAWN() -> Color {
        Color { r: 0.8117647058823529, g: 0.6862745098039216, b: 0.4823529411764706, a: 1.0 }
    }

    /// MUTED_BLUE | #3B719F | rgb(0.231, 0.443, 0.624)
    #[classattr]
    pub fn MUTED_BLUE() -> Color {
        Color { r: 0.23137254901960785, g: 0.44313725490196076, b: 0.6235294117647059, a: 1.0 }
    }

    /// PALE_ROSE | #FDC1C5 | rgb(0.992, 0.757, 0.773)
    #[classattr]
    pub fn PALE_ROSE() -> Color {
        Color { r: 0.9921568627450981, g: 0.7568627450980392, b: 0.7725490196078432, a: 1.0 }
    }

    /// DARK_MINT_GREEN | #20C073 | rgb(0.125, 0.753, 0.451)
    #[classattr]
    pub fn DARK_MINT_GREEN() -> Color {
        Color { r: 0.12549019607843137, g: 0.7529411764705882, b: 0.45098039215686275, a: 1.0 }
    }

    /// AMETHYST | #9B5FC0 | rgb(0.608, 0.373, 0.753)
    #[classattr]
    pub fn AMETHYST() -> Color {
        Color { r: 0.6078431372549019, g: 0.37254901960784315, b: 0.7529411764705882, a: 1.0 }
    }

    /// BLUE_GREEN | #0F9B8E | rgb(0.059, 0.608, 0.557)
    #[classattr]
    pub fn BLUE_GREEN() -> Color {
        Color { r: 0.058823529411764705, g: 0.6078431372549019, b: 0.5568627450980392, a: 1.0 }
    }

    /// CHESTNUT | #742802 | rgb(0.455, 0.157, 0.008)
    #[classattr]
    pub fn CHESTNUT() -> Color {
        Color { r: 0.4549019607843137, g: 0.1568627450980392, b: 0.00784313725490196, a: 1.0 }
    }

    /// SICK_GREEN | #9DB92C | rgb(0.616, 0.725, 0.173)
    #[classattr]
    pub fn SICK_GREEN() -> Color {
        Color { r: 0.615686274509804, g: 0.7254901960784313, b: 0.17254901960784313, a: 1.0 }
    }

    /// PEA | #A4BF20 | rgb(0.643, 0.749, 0.125)
    #[classattr]
    pub fn PEA() -> Color {
        Color { r: 0.6431372549019608, g: 0.7490196078431373, b: 0.12549019607843137, a: 1.0 }
    }

    /// RUSTY_ORANGE | #CD5909 | rgb(0.804, 0.349, 0.035)
    #[classattr]
    pub fn RUSTY_ORANGE() -> Color {
        Color { r: 0.803921568627451, g: 0.34901960784313724, b: 0.03529411764705882, a: 1.0 }
    }

    /// STONE | #ADA587 | rgb(0.678, 0.647, 0.529)
    #[classattr]
    pub fn STONE() -> Color {
        Color { r: 0.6784313725490196, g: 0.6470588235294118, b: 0.5294117647058824, a: 1.0 }
    }

    /// ROSE_RED | #BE013C | rgb(0.745, 0.004, 0.235)
    #[classattr]
    pub fn ROSE_RED() -> Color {
        Color { r: 0.7450980392156863, g: 0.00392156862745098, b: 0.23529411764705882, a: 1.0 }
    }

    /// PALE_AQUA | #B8FFEB | rgb(0.722, 1.000, 0.922)
    #[classattr]
    pub fn PALE_AQUA() -> Color {
        Color { r: 0.7215686274509804, g: 1.0, b: 0.9215686274509803, a: 1.0 }
    }

    /// DEEP_ORANGE | #DC4D01 | rgb(0.863, 0.302, 0.004)
    #[classattr]
    pub fn DEEP_ORANGE() -> Color {
        Color { r: 0.8627450980392157, g: 0.30196078431372547, b: 0.00392156862745098, a: 1.0 }
    }

    /// EARTH | #A2653E | rgb(0.635, 0.396, 0.243)
    #[classattr]
    pub fn EARTH() -> Color {
        Color { r: 0.6352941176470588, g: 0.396078431372549, b: 0.24313725490196078, a: 1.0 }
    }

    /// MOSSY_GREEN | #638B27 | rgb(0.388, 0.545, 0.153)
    #[classattr]
    pub fn MOSSY_GREEN() -> Color {
        Color { r: 0.38823529411764707, g: 0.5450980392156862, b: 0.15294117647058825, a: 1.0 }
    }

    /// GRASSY_GREEN | #419C03 | rgb(0.255, 0.612, 0.012)
    #[classattr]
    pub fn GRASSY_GREEN() -> Color {
        Color { r: 0.2549019607843137, g: 0.611764705882353, b: 0.011764705882352941, a: 1.0 }
    }

    /// PALE_LIME_GREEN | #B1FF65 | rgb(0.694, 1.000, 0.396)
    #[classattr]
    pub fn PALE_LIME_GREEN() -> Color {
        Color { r: 0.6941176470588235, g: 1.0, b: 0.396078431372549, a: 1.0 }
    }

    /// LIGHT_GREY_BLUE | #9DBCD4 | rgb(0.616, 0.737, 0.831)
    #[classattr]
    pub fn LIGHT_GREY_BLUE() -> Color {
        Color { r: 0.615686274509804, g: 0.7372549019607844, b: 0.8313725490196079, a: 1.0 }
    }

    /// PALE_GREY | #FDFDFE | rgb(0.992, 0.992, 0.996)
    #[classattr]
    pub fn PALE_GREY() -> Color {
        Color { r: 0.9921568627450981, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 }
    }

    /// ASPARAGUS | #77AB56 | rgb(0.467, 0.671, 0.337)
    #[classattr]
    pub fn ASPARAGUS() -> Color {
        Color { r: 0.4666666666666667, g: 0.6705882352941176, b: 0.33725490196078434, a: 1.0 }
    }

    /// BLUEBERRY | #464196 | rgb(0.275, 0.255, 0.588)
    #[classattr]
    pub fn BLUEBERRY() -> Color {
        Color { r: 0.27450980392156865, g: 0.2549019607843137, b: 0.5882352941176471, a: 1.0 }
    }

    /// PURPLE_RED | #990147 | rgb(0.600, 0.004, 0.278)
    #[classattr]
    pub fn PURPLE_RED() -> Color {
        Color { r: 0.6, g: 0.00392156862745098, b: 0.2784313725490196, a: 1.0 }
    }

    /// PALE_LIME | #BEFD73 | rgb(0.745, 0.992, 0.451)
    #[classattr]
    pub fn PALE_LIME() -> Color {
        Color { r: 0.7450980392156863, g: 0.9921568627450981, b: 0.45098039215686275, a: 1.0 }
    }

    /// GREENISH_TEAL | #32BF84 | rgb(0.196, 0.749, 0.518)
    #[classattr]
    pub fn GREENISH_TEAL() -> Color {
        Color { r: 0.19607843137254902, g: 0.7490196078431373, b: 0.5176470588235295, a: 1.0 }
    }

    /// CARAMEL | #AF6F09 | rgb(0.686, 0.435, 0.035)
    #[classattr]
    pub fn CARAMEL() -> Color {
        Color { r: 0.6862745098039216, g: 0.43529411764705883, b: 0.03529411764705882, a: 1.0 }
    }

    /// DEEP_MAGENTA | #A0025C | rgb(0.627, 0.008, 0.361)
    #[classattr]
    pub fn DEEP_MAGENTA() -> Color {
        Color { r: 0.6274509803921569, g: 0.00784313725490196, b: 0.3607843137254902, a: 1.0 }
    }

    /// LIGHT_PEACH | #FFD8B1 | rgb(1.000, 0.847, 0.694)
    #[classattr]
    pub fn LIGHT_PEACH() -> Color {
        Color { r: 1.0, g: 0.8470588235294118, b: 0.6941176470588235, a: 1.0 }
    }

    /// MILK_CHOCOLATE | #7F4E1E | rgb(0.498, 0.306, 0.118)
    #[classattr]
    pub fn MILK_CHOCOLATE() -> Color {
        Color { r: 0.4980392156862745, g: 0.3058823529411765, b: 0.11764705882352941, a: 1.0 }
    }

    /// OCHER | #BF9B0C | rgb(0.749, 0.608, 0.047)
    #[classattr]
    pub fn OCHER() -> Color {
        Color { r: 0.7490196078431373, g: 0.6078431372549019, b: 0.047058823529411764, a: 1.0 }
    }

    /// OFF_GREEN | #6BA353 | rgb(0.420, 0.639, 0.325)
    #[classattr]
    pub fn OFF_GREEN() -> Color {
        Color { r: 0.4196078431372549, g: 0.6392156862745098, b: 0.3254901960784314, a: 1.0 }
    }

    /// PURPLY_PINK | #F075E6 | rgb(0.941, 0.459, 0.902)
    #[classattr]
    pub fn PURPLY_PINK() -> Color {
        Color { r: 0.9411764705882353, g: 0.4588235294117647, b: 0.9019607843137255, a: 1.0 }
    }

    /// LIGHTBLUE | #7BC8F6 | rgb(0.482, 0.784, 0.965)
    #[classattr]
    pub fn LIGHTBLUE() -> Color {
        Color { r: 0.4823529411764706, g: 0.7843137254901961, b: 0.9647058823529412, a: 1.0 }
    }

    /// DUSKY_BLUE | #475F94 | rgb(0.278, 0.373, 0.580)
    #[classattr]
    pub fn DUSKY_BLUE() -> Color {
        Color { r: 0.2784313725490196, g: 0.37254901960784315, b: 0.5803921568627451, a: 1.0 }
    }

    /// GOLDEN | #F5BF03 | rgb(0.961, 0.749, 0.012)
    #[classattr]
    pub fn GOLDEN() -> Color {
        Color { r: 0.9607843137254902, g: 0.7490196078431373, b: 0.011764705882352941, a: 1.0 }
    }

    /// LIGHT_BEIGE | #FFFEB6 | rgb(1.000, 0.996, 0.714)
    #[classattr]
    pub fn LIGHT_BEIGE() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.7137254901960784, a: 1.0 }
    }

    /// BUTTER_YELLOW | #FFFD74 | rgb(1.000, 0.992, 0.455)
    #[classattr]
    pub fn BUTTER_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.4549019607843137, a: 1.0 }
    }

    /// DUSKY_PURPLE | #895B7B | rgb(0.537, 0.357, 0.482)
    #[classattr]
    pub fn DUSKY_PURPLE() -> Color {
        Color { r: 0.5372549019607843, g: 0.3568627450980392, b: 0.4823529411764706, a: 1.0 }
    }

    /// FRENCH_BLUE | #436BAD | rgb(0.263, 0.420, 0.678)
    #[classattr]
    pub fn FRENCH_BLUE() -> Color {
        Color { r: 0.2627450980392157, g: 0.4196078431372549, b: 0.6784313725490196, a: 1.0 }
    }

    /// UGLY_YELLOW | #D0C101 | rgb(0.816, 0.757, 0.004)
    #[classattr]
    pub fn UGLY_YELLOW() -> Color {
        Color { r: 0.8156862745098039, g: 0.7568627450980392, b: 0.00392156862745098, a: 1.0 }
    }

    /// GREENY_YELLOW | #C6F808 | rgb(0.776, 0.973, 0.031)
    #[classattr]
    pub fn GREENY_YELLOW() -> Color {
        Color { r: 0.7764705882352941, g: 0.9725490196078431, b: 0.03137254901960784, a: 1.0 }
    }

    /// ORANGISH_RED | #F43605 | rgb(0.957, 0.212, 0.020)
    #[classattr]
    pub fn ORANGISH_RED() -> Color {
        Color { r: 0.9568627450980393, g: 0.21176470588235294, b: 0.0196078431372549, a: 1.0 }
    }

    /// SHAMROCK_GREEN | #02C14D | rgb(0.008, 0.757, 0.302)
    #[classattr]
    pub fn SHAMROCK_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.7568627450980392, b: 0.30196078431372547, a: 1.0 }
    }

    /// ORANGISH_BROWN | #B25F03 | rgb(0.698, 0.373, 0.012)
    #[classattr]
    pub fn ORANGISH_BROWN() -> Color {
        Color { r: 0.6980392156862745, g: 0.37254901960784315, b: 0.011764705882352941, a: 1.0 }
    }

    /// TREE_GREEN | #2A7E19 | rgb(0.165, 0.494, 0.098)
    #[classattr]
    pub fn TREE_GREEN() -> Color {
        Color { r: 0.16470588235294117, g: 0.49411764705882355, b: 0.09803921568627451, a: 1.0 }
    }

    /// DEEP_VIOLET | #490648 | rgb(0.286, 0.024, 0.282)
    #[classattr]
    pub fn DEEP_VIOLET() -> Color {
        Color { r: 0.28627450980392155, g: 0.023529411764705882, b: 0.2823529411764706, a: 1.0 }
    }

    /// GUNMETAL | #536267 | rgb(0.325, 0.384, 0.404)
    #[classattr]
    pub fn GUNMETAL() -> Color {
        Color { r: 0.3254901960784314, g: 0.3843137254901961, b: 0.403921568627451, a: 1.0 }
    }

    /// BLUE_PURPLE | #5A06EF | rgb(0.353, 0.024, 0.937)
    #[classattr]
    pub fn BLUE_PURPLE() -> Color {
        Color { r: 0.35294117647058826, g: 0.023529411764705882, b: 0.9372549019607843, a: 1.0 }
    }

    /// CHERRY | #CF0234 | rgb(0.812, 0.008, 0.204)
    #[classattr]
    pub fn CHERRY() -> Color {
        Color { r: 0.8117647058823529, g: 0.00784313725490196, b: 0.20392156862745098, a: 1.0 }
    }

    /// SANDY_BROWN | #C4A661 | rgb(0.769, 0.651, 0.380)
    #[classattr]
    pub fn SANDY_BROWN() -> Color {
        Color { r: 0.7686274509803922, g: 0.6509803921568628, b: 0.3803921568627451, a: 1.0 }
    }

    /// WARM_GREY | #978A84 | rgb(0.592, 0.541, 0.518)
    #[classattr]
    pub fn WARM_GREY() -> Color {
        Color { r: 0.592156862745098, g: 0.5411764705882353, b: 0.5176470588235295, a: 1.0 }
    }

    /// DARK_INDIGO | #1F0954 | rgb(0.122, 0.035, 0.329)
    #[classattr]
    pub fn DARK_INDIGO() -> Color {
        Color { r: 0.12156862745098039, g: 0.03529411764705882, b: 0.32941176470588235, a: 1.0 }
    }

    /// MIDNIGHT | #03012D | rgb(0.012, 0.004, 0.176)
    #[classattr]
    pub fn MIDNIGHT() -> Color {
        Color { r: 0.011764705882352941, g: 0.00392156862745098, b: 0.17647058823529413, a: 1.0 }
    }

    /// BLUEY_GREEN | #2BB179 | rgb(0.169, 0.694, 0.475)
    #[classattr]
    pub fn BLUEY_GREEN() -> Color {
        Color { r: 0.16862745098039217, g: 0.6941176470588235, b: 0.4745098039215686, a: 1.0 }
    }

    /// GREY_PINK | #C3909B | rgb(0.765, 0.565, 0.608)
    #[classattr]
    pub fn GREY_PINK() -> Color {
        Color { r: 0.7647058823529411, g: 0.5647058823529412, b: 0.6078431372549019, a: 1.0 }
    }

    /// SOFT_PURPLE | #A66FB5 | rgb(0.651, 0.435, 0.710)
    #[classattr]
    pub fn SOFT_PURPLE() -> Color {
        Color { r: 0.6509803921568628, g: 0.43529411764705883, b: 0.7098039215686275, a: 1.0 }
    }

    /// BLOOD | #770001 | rgb(0.467, 0.000, 0.004)
    #[classattr]
    pub fn BLOOD() -> Color {
        Color { r: 0.4666666666666667, g: 0.0, b: 0.00392156862745098, a: 1.0 }
    }

    /// BROWN_RED | #922B05 | rgb(0.573, 0.169, 0.020)
    #[classattr]
    pub fn BROWN_RED() -> Color {
        Color { r: 0.5725490196078431, g: 0.16862745098039217, b: 0.0196078431372549, a: 1.0 }
    }

    /// MEDIUM_GREY | #7D7F7C | rgb(0.490, 0.498, 0.486)
    #[classattr]
    pub fn MEDIUM_GREY() -> Color {
        Color { r: 0.49019607843137253, g: 0.4980392156862745, b: 0.48627450980392156, a: 1.0 }
    }

    /// BERRY | #990F4B | rgb(0.600, 0.059, 0.294)
    #[classattr]
    pub fn BERRY() -> Color {
        Color { r: 0.6, g: 0.058823529411764705, b: 0.29411764705882354, a: 1.0 }
    }

    /// POO | #8F7303 | rgb(0.561, 0.451, 0.012)
    #[classattr]
    pub fn POO() -> Color {
        Color { r: 0.5607843137254902, g: 0.45098039215686275, b: 0.011764705882352941, a: 1.0 }
    }

    /// PURPLEY_PINK | #C83CB9 | rgb(0.784, 0.235, 0.725)
    #[classattr]
    pub fn PURPLEY_PINK() -> Color {
        Color { r: 0.7843137254901961, g: 0.23529411764705882, b: 0.7254901960784313, a: 1.0 }
    }

    /// LIGHT_SALMON | #FEA993 | rgb(0.996, 0.663, 0.576)
    #[classattr]
    pub fn LIGHT_SALMON() -> Color {
        Color { r: 0.996078431372549, g: 0.6627450980392157, b: 0.5764705882352941, a: 1.0 }
    }

    /// SNOT | #ACBB0D | rgb(0.675, 0.733, 0.051)
    #[classattr]
    pub fn SNOT() -> Color {
        Color { r: 0.6745098039215687, g: 0.7333333333333333, b: 0.050980392156862744, a: 1.0 }
    }

    /// EASTER_PURPLE | #C071FE | rgb(0.753, 0.443, 0.996)
    #[classattr]
    pub fn EASTER_PURPLE() -> Color {
        Color { r: 0.7529411764705882, g: 0.44313725490196076, b: 0.996078431372549, a: 1.0 }
    }

    /// LIGHT_YELLOW_GREEN | #CCFD7F | rgb(0.800, 0.992, 0.498)
    #[classattr]
    pub fn LIGHT_YELLOW_GREEN() -> Color {
        Color { r: 0.8, g: 0.9921568627450981, b: 0.4980392156862745, a: 1.0 }
    }

    /// DARK_NAVY_BLUE | #00022E | rgb(0.000, 0.008, 0.180)
    #[classattr]
    pub fn DARK_NAVY_BLUE() -> Color {
        Color { r: 0.0, g: 0.00784313725490196, b: 0.1803921568627451, a: 1.0 }
    }

    /// DRAB | #828344 | rgb(0.510, 0.514, 0.267)
    #[classattr]
    pub fn DRAB() -> Color {
        Color { r: 0.5098039215686274, g: 0.5137254901960784, b: 0.26666666666666666, a: 1.0 }
    }

    /// LIGHT_ROSE | #FFC5CB | rgb(1.000, 0.773, 0.796)
    #[classattr]
    pub fn LIGHT_ROSE() -> Color {
        Color { r: 1.0, g: 0.7725490196078432, b: 0.796078431372549, a: 1.0 }
    }

    /// ROUGE | #AB1239 | rgb(0.671, 0.071, 0.224)
    #[classattr]
    pub fn ROUGE() -> Color {
        Color { r: 0.6705882352941176, g: 0.07058823529411765, b: 0.2235294117647059, a: 1.0 }
    }

    /// PURPLISH_RED | #B0054B | rgb(0.690, 0.020, 0.294)
    #[classattr]
    pub fn PURPLISH_RED() -> Color {
        Color { r: 0.6901960784313725, g: 0.0196078431372549, b: 0.29411764705882354, a: 1.0 }
    }

    /// SLIME_GREEN | #99CC04 | rgb(0.600, 0.800, 0.016)
    #[classattr]
    pub fn SLIME_GREEN() -> Color {
        Color { r: 0.6, g: 0.8, b: 0.01568627450980392, a: 1.0 }
    }

    /// BABY_POOP | #937C00 | rgb(0.576, 0.486, 0.000)
    #[classattr]
    pub fn BABY_POOP() -> Color {
        Color { r: 0.5764705882352941, g: 0.48627450980392156, b: 0.0, a: 1.0 }
    }

    /// IRISH_GREEN | #019529 | rgb(0.004, 0.584, 0.161)
    #[classattr]
    pub fn IRISH_GREEN() -> Color {
        Color { r: 0.00392156862745098, g: 0.5843137254901961, b: 0.1607843137254902, a: 1.0 }
    }

    /// PINK_PURPLE | #EF1DE7 | rgb(0.937, 0.114, 0.906)
    #[classattr]
    pub fn PINK_PURPLE() -> Color {
        Color { r: 0.9372549019607843, g: 0.11372549019607843, b: 0.9058823529411765, a: 1.0 }
    }

    /// DARK_NAVY | #000435 | rgb(0.000, 0.016, 0.208)
    #[classattr]
    pub fn DARK_NAVY() -> Color {
        Color { r: 0.0, g: 0.01568627450980392, b: 0.20784313725490197, a: 1.0 }
    }

    /// GREENY_BLUE | #42B395 | rgb(0.259, 0.702, 0.584)
    #[classattr]
    pub fn GREENY_BLUE() -> Color {
        Color { r: 0.25882352941176473, g: 0.7019607843137254, b: 0.5843137254901961, a: 1.0 }
    }

    /// LIGHT_PLUM | #9D5783 | rgb(0.616, 0.341, 0.514)
    #[classattr]
    pub fn LIGHT_PLUM() -> Color {
        Color { r: 0.615686274509804, g: 0.3411764705882353, b: 0.5137254901960784, a: 1.0 }
    }

    /// PINKISH_GREY | #C8ACA9 | rgb(0.784, 0.675, 0.663)
    #[classattr]
    pub fn PINKISH_GREY() -> Color {
        Color { r: 0.7843137254901961, g: 0.6745098039215687, b: 0.6627450980392157, a: 1.0 }
    }

    /// DIRTY_ORANGE | #C87606 | rgb(0.784, 0.463, 0.024)
    #[classattr]
    pub fn DIRTY_ORANGE() -> Color {
        Color { r: 0.7843137254901961, g: 0.4627450980392157, b: 0.023529411764705882, a: 1.0 }
    }

    /// RUST_RED | #AA2704 | rgb(0.667, 0.153, 0.016)
    #[classattr]
    pub fn RUST_RED() -> Color {
        Color { r: 0.6666666666666666, g: 0.15294117647058825, b: 0.01568627450980392, a: 1.0 }
    }

    /// PALE_LILAC | #E4CBFF | rgb(0.894, 0.796, 1.000)
    #[classattr]
    pub fn PALE_LILAC() -> Color {
        Color { r: 0.8941176470588236, g: 0.796078431372549, b: 1.0, a: 1.0 }
    }

    /// ORANGEY_RED | #FA4224 | rgb(0.980, 0.259, 0.141)
    #[classattr]
    pub fn ORANGEY_RED() -> Color {
        Color { r: 0.9803921568627451, g: 0.25882352941176473, b: 0.1411764705882353, a: 1.0 }
    }

    /// PRIMARY_BLUE | #0804F9 | rgb(0.031, 0.016, 0.976)
    #[classattr]
    pub fn PRIMARY_BLUE() -> Color {
        Color { r: 0.03137254901960784, g: 0.01568627450980392, b: 0.9764705882352941, a: 1.0 }
    }

    /// KERMIT_GREEN | #5CB200 | rgb(0.361, 0.698, 0.000)
    #[classattr]
    pub fn KERMIT_GREEN() -> Color {
        Color { r: 0.3607843137254902, g: 0.6980392156862745, b: 0.0, a: 1.0 }
    }

    /// BROWNISH_PURPLE | #76424E | rgb(0.463, 0.259, 0.306)
    #[classattr]
    pub fn BROWNISH_PURPLE() -> Color {
        Color { r: 0.4627450980392157, g: 0.25882352941176473, b: 0.3058823529411765, a: 1.0 }
    }

    /// MURKY_GREEN | #6C7A0E | rgb(0.424, 0.478, 0.055)
    #[classattr]
    pub fn MURKY_GREEN() -> Color {
        Color { r: 0.4235294117647059, g: 0.47843137254901963, b: 0.054901960784313725, a: 1.0 }
    }

    /// WHEAT | #FBDD7E | rgb(0.984, 0.867, 0.494)
    #[classattr]
    pub fn WHEAT() -> Color {
        Color { r: 0.984313725490196, g: 0.8666666666666667, b: 0.49411764705882355, a: 1.0 }
    }

    /// VERY_DARK_PURPLE | #2A0134 | rgb(0.165, 0.004, 0.204)
    #[classattr]
    pub fn VERY_DARK_PURPLE() -> Color {
        Color { r: 0.16470588235294117, g: 0.00392156862745098, b: 0.20392156862745098, a: 1.0 }
    }

    /// BOTTLE_GREEN | #044A05 | rgb(0.016, 0.290, 0.020)
    #[classattr]
    pub fn BOTTLE_GREEN() -> Color {
        Color { r: 0.01568627450980392, g: 0.2901960784313726, b: 0.0196078431372549, a: 1.0 }
    }

    /// WATERMELON | #FD4659 | rgb(0.992, 0.275, 0.349)
    #[classattr]
    pub fn WATERMELON() -> Color {
        Color { r: 0.9921568627450981, g: 0.27450980392156865, b: 0.34901960784313724, a: 1.0 }
    }

    /// DEEP_SKY_BLUE | #0D75F8 | rgb(0.051, 0.459, 0.973)
    #[classattr]
    pub fn DEEP_SKY_BLUE() -> Color {
        Color { r: 0.050980392156862744, g: 0.4588235294117647, b: 0.9725490196078431, a: 1.0 }
    }

    /// FIRE_ENGINE_RED | #FE0002 | rgb(0.996, 0.000, 0.008)
    #[classattr]
    pub fn FIRE_ENGINE_RED() -> Color {
        Color { r: 0.996078431372549, g: 0.0, b: 0.00784313725490196, a: 1.0 }
    }

    /// YELLOW_OCHRE | #CB9D06 | rgb(0.796, 0.616, 0.024)
    #[classattr]
    pub fn YELLOW_OCHRE() -> Color {
        Color { r: 0.796078431372549, g: 0.615686274509804, b: 0.023529411764705882, a: 1.0 }
    }

    /// PUMPKIN_ORANGE | #FB7D07 | rgb(0.984, 0.490, 0.027)
    #[classattr]
    pub fn PUMPKIN_ORANGE() -> Color {
        Color { r: 0.984313725490196, g: 0.49019607843137253, b: 0.027450980392156862, a: 1.0 }
    }

    /// PALE_OLIVE | #B9CC81 | rgb(0.725, 0.800, 0.506)
    #[classattr]
    pub fn PALE_OLIVE() -> Color {
        Color { r: 0.7254901960784313, g: 0.8, b: 0.5058823529411764, a: 1.0 }
    }

    /// LIGHT_LILAC | #EDC8FF | rgb(0.929, 0.784, 1.000)
    #[classattr]
    pub fn LIGHT_LILAC() -> Color {
        Color { r: 0.9294117647058824, g: 0.7843137254901961, b: 1.0, a: 1.0 }
    }

    /// LIGHTISH_GREEN | #61E160 | rgb(0.380, 0.882, 0.376)
    #[classattr]
    pub fn LIGHTISH_GREEN() -> Color {
        Color { r: 0.3803921568627451, g: 0.8823529411764706, b: 0.3764705882352941, a: 1.0 }
    }

    /// CAROLINA_BLUE | #8AB8FE | rgb(0.541, 0.722, 0.996)
    #[classattr]
    pub fn CAROLINA_BLUE() -> Color {
        Color { r: 0.5411764705882353, g: 0.7215686274509804, b: 0.996078431372549, a: 1.0 }
    }

    /// MULBERRY | #920A4E | rgb(0.573, 0.039, 0.306)
    #[classattr]
    pub fn MULBERRY() -> Color {
        Color { r: 0.5725490196078431, g: 0.0392156862745098, b: 0.3058823529411765, a: 1.0 }
    }

    /// SHOCKING_PINK | #FE02A2 | rgb(0.996, 0.008, 0.635)
    #[classattr]
    pub fn SHOCKING_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.00784313725490196, b: 0.6352941176470588, a: 1.0 }
    }

    /// AUBURN | #9A3001 | rgb(0.604, 0.188, 0.004)
    #[classattr]
    pub fn AUBURN() -> Color {
        Color { r: 0.6039215686274509, g: 0.18823529411764706, b: 0.00392156862745098, a: 1.0 }
    }

    /// BRIGHT_LIME_GREEN | #65FE08 | rgb(0.396, 0.996, 0.031)
    #[classattr]
    pub fn BRIGHT_LIME_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.996078431372549, b: 0.03137254901960784, a: 1.0 }
    }

    /// CELADON | #BEFDB7 | rgb(0.745, 0.992, 0.718)
    #[classattr]
    pub fn CELADON() -> Color {
        Color { r: 0.7450980392156863, g: 0.9921568627450981, b: 0.7176470588235294, a: 1.0 }
    }

    /// PINKISH_BROWN | #B17261 | rgb(0.694, 0.447, 0.380)
    #[classattr]
    pub fn PINKISH_BROWN() -> Color {
        Color { r: 0.6941176470588235, g: 0.4470588235294118, b: 0.3803921568627451, a: 1.0 }
    }

    /// POO_BROWN | #885F01 | rgb(0.533, 0.373, 0.004)
    #[classattr]
    pub fn POO_BROWN() -> Color {
        Color { r: 0.5333333333333333, g: 0.37254901960784315, b: 0.00392156862745098, a: 1.0 }
    }

    /// BRIGHT_SKY_BLUE | #02CCFE | rgb(0.008, 0.800, 0.996)
    #[classattr]
    pub fn BRIGHT_SKY_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.8, b: 0.996078431372549, a: 1.0 }
    }

    /// CELERY | #C1FD95 | rgb(0.757, 0.992, 0.584)
    #[classattr]
    pub fn CELERY() -> Color {
        Color { r: 0.7568627450980392, g: 0.9921568627450981, b: 0.5843137254901961, a: 1.0 }
    }

    /// DIRT_BROWN | #836539 | rgb(0.514, 0.396, 0.224)
    #[classattr]
    pub fn DIRT_BROWN() -> Color {
        Color { r: 0.5137254901960784, g: 0.396078431372549, b: 0.2235294117647059, a: 1.0 }
    }

    /// STRAWBERRY | #FB2943 | rgb(0.984, 0.161, 0.263)
    #[classattr]
    pub fn STRAWBERRY() -> Color {
        Color { r: 0.984313725490196, g: 0.1607843137254902, b: 0.2627450980392157, a: 1.0 }
    }

    /// DARK_LIME | #84B701 | rgb(0.518, 0.718, 0.004)
    #[classattr]
    pub fn DARK_LIME() -> Color {
        Color { r: 0.5176470588235295, g: 0.7176470588235294, b: 0.00392156862745098, a: 1.0 }
    }

    /// COPPER | #B66325 | rgb(0.714, 0.388, 0.145)
    #[classattr]
    pub fn COPPER() -> Color {
        Color { r: 0.7137254901960784, g: 0.38823529411764707, b: 0.1450980392156863, a: 1.0 }
    }

    /// MEDIUM_BROWN | #7F5112 | rgb(0.498, 0.318, 0.071)
    #[classattr]
    pub fn MEDIUM_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.3176470588235294, b: 0.07058823529411765, a: 1.0 }
    }

    /// MUTED_GREEN | #5FA052 | rgb(0.373, 0.627, 0.322)
    #[classattr]
    pub fn MUTED_GREEN() -> Color {
        Color { r: 0.37254901960784315, g: 0.6274509803921569, b: 0.3215686274509804, a: 1.0 }
    }

    /// ROBINS_EGG | #6DEDFD | rgb(0.427, 0.929, 0.992)
    #[classattr]
    pub fn ROBINS_EGG() -> Color {
        Color { r: 0.42745098039215684, g: 0.9294117647058824, b: 0.9921568627450981, a: 1.0 }
    }

    /// BRIGHT_AQUA | #0BF9EA | rgb(0.043, 0.976, 0.918)
    #[classattr]
    pub fn BRIGHT_AQUA() -> Color {
        Color { r: 0.043137254901960784, g: 0.9764705882352941, b: 0.9176470588235294, a: 1.0 }
    }

    /// BRIGHT_LAVENDER | #C760FF | rgb(0.780, 0.376, 1.000)
    #[classattr]
    pub fn BRIGHT_LAVENDER() -> Color {
        Color { r: 0.7803921568627451, g: 0.3764705882352941, b: 1.0, a: 1.0 }
    }

    /// IVORY | #FFFFCB | rgb(1.000, 1.000, 0.796)
    #[classattr]
    pub fn IVORY() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.796078431372549, a: 1.0 }
    }

    /// VERY_LIGHT_PURPLE | #F6CEFC | rgb(0.965, 0.808, 0.988)
    #[classattr]
    pub fn VERY_LIGHT_PURPLE() -> Color {
        Color { r: 0.9647058823529412, g: 0.807843137254902, b: 0.9882352941176471, a: 1.0 }
    }

    /// LIGHT_NAVY | #155084 | rgb(0.082, 0.314, 0.518)
    #[classattr]
    pub fn LIGHT_NAVY() -> Color {
        Color { r: 0.08235294117647059, g: 0.3137254901960784, b: 0.5176470588235295, a: 1.0 }
    }

    /// PINK_RED | #F5054F | rgb(0.961, 0.020, 0.310)
    #[classattr]
    pub fn PINK_RED() -> Color {
        Color { r: 0.9607843137254902, g: 0.0196078431372549, b: 0.30980392156862746, a: 1.0 }
    }

    /// OLIVE_BROWN | #645403 | rgb(0.392, 0.329, 0.012)
    #[classattr]
    pub fn OLIVE_BROWN() -> Color {
        Color { r: 0.39215686274509803, g: 0.32941176470588235, b: 0.011764705882352941, a: 1.0 }
    }

    /// POOP_BROWN | #7A5901 | rgb(0.478, 0.349, 0.004)
    #[classattr]
    pub fn POOP_BROWN() -> Color {
        Color { r: 0.47843137254901963, g: 0.34901960784313724, b: 0.00392156862745098, a: 1.0 }
    }

    /// MUSTARD_GREEN | #A8B504 | rgb(0.659, 0.710, 0.016)
    #[classattr]
    pub fn MUSTARD_GREEN() -> Color {
        Color { r: 0.6588235294117647, g: 0.7098039215686275, b: 0.01568627450980392, a: 1.0 }
    }

    /// OCEAN_GREEN | #3D9973 | rgb(0.239, 0.600, 0.451)
    #[classattr]
    pub fn OCEAN_GREEN() -> Color {
        Color { r: 0.23921568627450981, g: 0.6, b: 0.45098039215686275, a: 1.0 }
    }

    /// VERY_DARK_BLUE | #000133 | rgb(0.000, 0.004, 0.200)
    #[classattr]
    pub fn VERY_DARK_BLUE() -> Color {
        Color { r: 0.0, g: 0.00392156862745098, b: 0.2, a: 1.0 }
    }

    /// DUSTY_GREEN | #76A973 | rgb(0.463, 0.663, 0.451)
    #[classattr]
    pub fn DUSTY_GREEN() -> Color {
        Color { r: 0.4627450980392157, g: 0.6627450980392157, b: 0.45098039215686275, a: 1.0 }
    }

    /// LIGHT_NAVY_BLUE | #2E5A88 | rgb(0.180, 0.353, 0.533)
    #[classattr]
    pub fn LIGHT_NAVY_BLUE() -> Color {
        Color { r: 0.1803921568627451, g: 0.35294117647058826, b: 0.5333333333333333, a: 1.0 }
    }

    /// MINTY_GREEN | #0BF77D | rgb(0.043, 0.969, 0.490)
    #[classattr]
    pub fn MINTY_GREEN() -> Color {
        Color { r: 0.043137254901960784, g: 0.9686274509803922, b: 0.49019607843137253, a: 1.0 }
    }

    /// ADOBE | #BD6C48 | rgb(0.741, 0.424, 0.282)
    #[classattr]
    pub fn ADOBE() -> Color {
        Color { r: 0.7411764705882353, g: 0.4235294117647059, b: 0.2823529411764706, a: 1.0 }
    }

    /// BARNEY | #AC1DB8 | rgb(0.675, 0.114, 0.722)
    #[classattr]
    pub fn BARNEY() -> Color {
        Color { r: 0.6745098039215687, g: 0.11372549019607843, b: 0.7215686274509804, a: 1.0 }
    }

    /// JADE_GREEN | #2BAF6A | rgb(0.169, 0.686, 0.416)
    #[classattr]
    pub fn JADE_GREEN() -> Color {
        Color { r: 0.16862745098039217, g: 0.6862745098039216, b: 0.41568627450980394, a: 1.0 }
    }

    /// BRIGHT_LIGHT_BLUE | #26F7FD | rgb(0.149, 0.969, 0.992)
    #[classattr]
    pub fn BRIGHT_LIGHT_BLUE() -> Color {
        Color { r: 0.14901960784313725, g: 0.9686274509803922, b: 0.9921568627450981, a: 1.0 }
    }

    /// LIGHT_LIME | #AEFD6C | rgb(0.682, 0.992, 0.424)
    #[classattr]
    pub fn LIGHT_LIME() -> Color {
        Color { r: 0.6823529411764706, g: 0.9921568627450981, b: 0.4235294117647059, a: 1.0 }
    }

    /// DARK_KHAKI | #9B8F55 | rgb(0.608, 0.561, 0.333)
    #[classattr]
    pub fn DARK_KHAKI() -> Color {
        Color { r: 0.6078431372549019, g: 0.5607843137254902, b: 0.3333333333333333, a: 1.0 }
    }

    /// ORANGE_YELLOW | #FFAD01 | rgb(1.000, 0.678, 0.004)
    #[classattr]
    pub fn ORANGE_YELLOW() -> Color {
        Color { r: 1.0, g: 0.6784313725490196, b: 0.00392156862745098, a: 1.0 }
    }

    /// OCRE | #C69C04 | rgb(0.776, 0.612, 0.016)
    #[classattr]
    pub fn OCRE() -> Color {
        Color { r: 0.7764705882352941, g: 0.611764705882353, b: 0.01568627450980392, a: 1.0 }
    }

    /// MAIZE | #F4D054 | rgb(0.957, 0.816, 0.329)
    #[classattr]
    pub fn MAIZE() -> Color {
        Color { r: 0.9568627450980393, g: 0.8156862745098039, b: 0.32941176470588235, a: 1.0 }
    }

    /// FADED_PINK | #DE9DAC | rgb(0.871, 0.616, 0.675)
    #[classattr]
    pub fn FADED_PINK() -> Color {
        Color { r: 0.8705882352941177, g: 0.615686274509804, b: 0.6745098039215687, a: 1.0 }
    }

    /// BRITISH_RACING_GREEN | #05480D | rgb(0.020, 0.282, 0.051)
    #[classattr]
    pub fn BRITISH_RACING_GREEN() -> Color {
        Color { r: 0.0196078431372549, g: 0.2823529411764706, b: 0.050980392156862744, a: 1.0 }
    }

    /// SANDSTONE | #C9AE74 | rgb(0.788, 0.682, 0.455)
    #[classattr]
    pub fn SANDSTONE() -> Color {
        Color { r: 0.788235294117647, g: 0.6823529411764706, b: 0.4549019607843137, a: 1.0 }
    }

    /// MUD_BROWN | #60460F | rgb(0.376, 0.275, 0.059)
    #[classattr]
    pub fn MUD_BROWN() -> Color {
        Color { r: 0.3764705882352941, g: 0.27450980392156865, b: 0.058823529411764705, a: 1.0 }
    }

    /// LIGHT_SEA_GREEN | #98F6B0 | rgb(0.596, 0.965, 0.690)
    #[classattr]
    pub fn LIGHT_SEA_GREEN() -> Color {
        Color { r: 0.596078431372549, g: 0.9647058823529412, b: 0.6901960784313725, a: 1.0 }
    }

    /// ROBIN_EGG_BLUE | #8AF1FE | rgb(0.541, 0.945, 0.996)
    #[classattr]
    pub fn ROBIN_EGG_BLUE() -> Color {
        Color { r: 0.5411764705882353, g: 0.9450980392156862, b: 0.996078431372549, a: 1.0 }
    }

    /// AQUA_MARINE | #2EE8BB | rgb(0.180, 0.910, 0.733)
    #[classattr]
    pub fn AQUA_MARINE() -> Color {
        Color { r: 0.1803921568627451, g: 0.9098039215686274, b: 0.7333333333333333, a: 1.0 }
    }

    /// DARK_SEA_GREEN | #11875D | rgb(0.067, 0.529, 0.365)
    #[classattr]
    pub fn DARK_SEA_GREEN() -> Color {
        Color { r: 0.06666666666666667, g: 0.5294117647058824, b: 0.36470588235294116, a: 1.0 }
    }

    /// SOFT_PINK | #FDB0C0 | rgb(0.992, 0.690, 0.753)
    #[classattr]
    pub fn SOFT_PINK() -> Color {
        Color { r: 0.9921568627450981, g: 0.6901960784313725, b: 0.7529411764705882, a: 1.0 }
    }

    /// ORANGEY_BROWN | #B16002 | rgb(0.694, 0.376, 0.008)
    #[classattr]
    pub fn ORANGEY_BROWN() -> Color {
        Color { r: 0.6941176470588235, g: 0.3764705882352941, b: 0.00784313725490196, a: 1.0 }
    }

    /// CHERRY_RED | #F7022A | rgb(0.969, 0.008, 0.165)
    #[classattr]
    pub fn CHERRY_RED() -> Color {
        Color { r: 0.9686274509803922, g: 0.00784313725490196, b: 0.16470588235294117, a: 1.0 }
    }

    /// BURNT_YELLOW | #D5AB09 | rgb(0.835, 0.671, 0.035)
    #[classattr]
    pub fn BURNT_YELLOW() -> Color {
        Color { r: 0.8352941176470589, g: 0.6705882352941176, b: 0.03529411764705882, a: 1.0 }
    }

    /// BROWNISH_GREY | #86775F | rgb(0.525, 0.467, 0.373)
    #[classattr]
    pub fn BROWNISH_GREY() -> Color {
        Color { r: 0.5254901960784314, g: 0.4666666666666667, b: 0.37254901960784315, a: 1.0 }
    }

    /// CAMEL | #C69F59 | rgb(0.776, 0.624, 0.349)
    #[classattr]
    pub fn CAMEL() -> Color {
        Color { r: 0.7764705882352941, g: 0.6235294117647059, b: 0.34901960784313724, a: 1.0 }
    }

    /// PURPLISH_GREY | #7A687F | rgb(0.478, 0.408, 0.498)
    #[classattr]
    pub fn PURPLISH_GREY() -> Color {
        Color { r: 0.47843137254901963, g: 0.40784313725490196, b: 0.4980392156862745, a: 1.0 }
    }

    /// MARINE | #042E60 | rgb(0.016, 0.180, 0.376)
    #[classattr]
    pub fn MARINE() -> Color {
        Color { r: 0.01568627450980392, g: 0.1803921568627451, b: 0.3764705882352941, a: 1.0 }
    }

    /// GREYISH_PINK | #C88D94 | rgb(0.784, 0.553, 0.580)
    #[classattr]
    pub fn GREYISH_PINK() -> Color {
        Color { r: 0.7843137254901961, g: 0.5529411764705883, b: 0.5803921568627451, a: 1.0 }
    }

    /// PALE_TURQUOISE | #A5FBD5 | rgb(0.647, 0.984, 0.835)
    #[classattr]
    pub fn PALE_TURQUOISE() -> Color {
        Color { r: 0.6470588235294118, g: 0.984313725490196, b: 0.8352941176470589, a: 1.0 }
    }

    /// PASTEL_YELLOW | #FFFE71 | rgb(1.000, 0.996, 0.443)
    #[classattr]
    pub fn PASTEL_YELLOW() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.44313725490196076, a: 1.0 }
    }

    /// BLUEY_PURPLE | #6241C7 | rgb(0.384, 0.255, 0.780)
    #[classattr]
    pub fn BLUEY_PURPLE() -> Color {
        Color { r: 0.3843137254901961, g: 0.2549019607843137, b: 0.7803921568627451, a: 1.0 }
    }

    /// CANARY_YELLOW | #FFFE40 | rgb(1.000, 0.996, 0.251)
    #[classattr]
    pub fn CANARY_YELLOW() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.25098039215686274, a: 1.0 }
    }

    /// FADED_RED | #D3494E | rgb(0.827, 0.286, 0.306)
    #[classattr]
    pub fn FADED_RED() -> Color {
        Color { r: 0.8274509803921568, g: 0.28627450980392155, b: 0.3058823529411765, a: 1.0 }
    }

    /// SEPIA | #985E2B | rgb(0.596, 0.369, 0.169)
    #[classattr]
    pub fn SEPIA() -> Color {
        Color { r: 0.596078431372549, g: 0.3686274509803922, b: 0.16862745098039217, a: 1.0 }
    }

    /// COFFEE | #A6814C | rgb(0.651, 0.506, 0.298)
    #[classattr]
    pub fn COFFEE() -> Color {
        Color { r: 0.6509803921568628, g: 0.5058823529411764, b: 0.2980392156862745, a: 1.0 }
    }

    /// BRIGHT_MAGENTA | #FF08E8 | rgb(1.000, 0.031, 0.910)
    #[classattr]
    pub fn BRIGHT_MAGENTA() -> Color {
        Color { r: 1.0, g: 0.03137254901960784, b: 0.9098039215686274, a: 1.0 }
    }

    /// MOCHA | #9D7651 | rgb(0.616, 0.463, 0.318)
    #[classattr]
    pub fn MOCHA() -> Color {
        Color { r: 0.615686274509804, g: 0.4627450980392157, b: 0.3176470588235294, a: 1.0 }
    }

    /// ECRU | #FEFFCA | rgb(0.996, 1.000, 0.792)
    #[classattr]
    pub fn ECRU() -> Color {
        Color { r: 0.996078431372549, g: 1.0, b: 0.792156862745098, a: 1.0 }
    }

    /// PURPLEISH | #98568D | rgb(0.596, 0.337, 0.553)
    #[classattr]
    pub fn PURPLEISH() -> Color {
        Color { r: 0.596078431372549, g: 0.33725490196078434, b: 0.5529411764705883, a: 1.0 }
    }

    /// CRANBERRY | #9E003A | rgb(0.620, 0.000, 0.227)
    #[classattr]
    pub fn CRANBERRY() -> Color {
        Color { r: 0.6196078431372549, g: 0.0, b: 0.22745098039215686, a: 1.0 }
    }

    /// DARKISH_GREEN | #287C37 | rgb(0.157, 0.486, 0.216)
    #[classattr]
    pub fn DARKISH_GREEN() -> Color {
        Color { r: 0.1568627450980392, g: 0.48627450980392156, b: 0.21568627450980393, a: 1.0 }
    }

    /// BROWN_ORANGE | #B96902 | rgb(0.725, 0.412, 0.008)
    #[classattr]
    pub fn BROWN_ORANGE() -> Color {
        Color { r: 0.7254901960784313, g: 0.4117647058823529, b: 0.00784313725490196, a: 1.0 }
    }

    /// DUSKY_ROSE | #BA6873 | rgb(0.729, 0.408, 0.451)
    #[classattr]
    pub fn DUSKY_ROSE() -> Color {
        Color { r: 0.7294117647058823, g: 0.40784313725490196, b: 0.45098039215686275, a: 1.0 }
    }

    /// MELON | #FF7855 | rgb(1.000, 0.471, 0.333)
    #[classattr]
    pub fn MELON() -> Color {
        Color { r: 1.0, g: 0.47058823529411764, b: 0.3333333333333333, a: 1.0 }
    }

    /// SICKLY_GREEN | #94B21C | rgb(0.580, 0.698, 0.110)
    #[classattr]
    pub fn SICKLY_GREEN() -> Color {
        Color { r: 0.5803921568627451, g: 0.6980392156862745, b: 0.10980392156862745, a: 1.0 }
    }

    /// SILVER | #C5C9C7 | rgb(0.773, 0.788, 0.780)
    #[classattr]
    pub fn SILVER() -> Color {
        Color { r: 0.7725490196078432, g: 0.788235294117647, b: 0.7803921568627451, a: 1.0 }
    }

    /// PURPLY_BLUE | #661AEE | rgb(0.400, 0.102, 0.933)
    #[classattr]
    pub fn PURPLY_BLUE() -> Color {
        Color { r: 0.4, g: 0.10196078431372549, b: 0.9333333333333333, a: 1.0 }
    }

    /// PURPLEISH_BLUE | #6140EF | rgb(0.380, 0.251, 0.937)
    #[classattr]
    pub fn PURPLEISH_BLUE() -> Color {
        Color { r: 0.3803921568627451, g: 0.25098039215686274, b: 0.9372549019607843, a: 1.0 }
    }

    /// HOSPITAL_GREEN | #9BE5AA | rgb(0.608, 0.898, 0.667)
    #[classattr]
    pub fn HOSPITAL_GREEN() -> Color {
        Color { r: 0.6078431372549019, g: 0.8980392156862745, b: 0.6666666666666666, a: 1.0 }
    }

    /// SHIT_BROWN | #7B5804 | rgb(0.482, 0.345, 0.016)
    #[classattr]
    pub fn SHIT_BROWN() -> Color {
        Color { r: 0.4823529411764706, g: 0.34509803921568627, b: 0.01568627450980392, a: 1.0 }
    }

    /// MID_BLUE | #276AB3 | rgb(0.153, 0.416, 0.702)
    #[classattr]
    pub fn MID_BLUE() -> Color {
        Color { r: 0.15294117647058825, g: 0.41568627450980394, b: 0.7019607843137254, a: 1.0 }
    }

    /// AMBER | #FEB308 | rgb(0.996, 0.702, 0.031)
    #[classattr]
    pub fn AMBER() -> Color {
        Color { r: 0.996078431372549, g: 0.7019607843137254, b: 0.03137254901960784, a: 1.0 }
    }

    /// EASTER_GREEN | #8CFD7E | rgb(0.549, 0.992, 0.494)
    #[classattr]
    pub fn EASTER_GREEN() -> Color {
        Color { r: 0.5490196078431373, g: 0.9921568627450981, b: 0.49411764705882355, a: 1.0 }
    }

    /// SOFT_BLUE | #6488EA | rgb(0.392, 0.533, 0.918)
    #[classattr]
    pub fn SOFT_BLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.5333333333333333, b: 0.9176470588235294, a: 1.0 }
    }

    /// CERULEAN_BLUE | #056EEE | rgb(0.020, 0.431, 0.933)
    #[classattr]
    pub fn CERULEAN_BLUE() -> Color {
        Color { r: 0.0196078431372549, g: 0.43137254901960786, b: 0.9333333333333333, a: 1.0 }
    }

    /// GOLDEN_BROWN | #B27A01 | rgb(0.698, 0.478, 0.004)
    #[classattr]
    pub fn GOLDEN_BROWN() -> Color {
        Color { r: 0.6980392156862745, g: 0.47843137254901963, b: 0.00392156862745098, a: 1.0 }
    }

    /// BRIGHT_TURQUOISE | #0FFEF9 | rgb(0.059, 0.996, 0.976)
    #[classattr]
    pub fn BRIGHT_TURQUOISE() -> Color {
        Color { r: 0.058823529411764705, g: 0.996078431372549, b: 0.9764705882352941, a: 1.0 }
    }

    /// RED_PINK | #FA2A55 | rgb(0.980, 0.165, 0.333)
    #[classattr]
    pub fn RED_PINK() -> Color {
        Color { r: 0.9803921568627451, g: 0.16470588235294117, b: 0.3333333333333333, a: 1.0 }
    }

    /// RED_PURPLE | #820747 | rgb(0.510, 0.027, 0.278)
    #[classattr]
    pub fn RED_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.027450980392156862, b: 0.2784313725490196, a: 1.0 }
    }

    /// GREYISH_BROWN | #7A6A4F | rgb(0.478, 0.416, 0.310)
    #[classattr]
    pub fn GREYISH_BROWN() -> Color {
        Color { r: 0.47843137254901963, g: 0.41568627450980394, b: 0.30980392156862746, a: 1.0 }
    }

    /// VERMILLION | #F4320C | rgb(0.957, 0.196, 0.047)
    #[classattr]
    pub fn VERMILLION() -> Color {
        Color { r: 0.9568627450980393, g: 0.19607843137254902, b: 0.047058823529411764, a: 1.0 }
    }

    /// RUSSET | #A13905 | rgb(0.631, 0.224, 0.020)
    #[classattr]
    pub fn RUSSET() -> Color {
        Color { r: 0.6313725490196078, g: 0.2235294117647059, b: 0.0196078431372549, a: 1.0 }
    }

    /// STEEL_GREY | #6F828A | rgb(0.435, 0.510, 0.541)
    #[classattr]
    pub fn STEEL_GREY() -> Color {
        Color { r: 0.43529411764705883, g: 0.5098039215686274, b: 0.5411764705882353, a: 1.0 }
    }

    /// LIGHTER_PURPLE | #A55AF4 | rgb(0.647, 0.353, 0.957)
    #[classattr]
    pub fn LIGHTER_PURPLE() -> Color {
        Color { r: 0.6470588235294118, g: 0.35294117647058826, b: 0.9568627450980393, a: 1.0 }
    }

    /// BRIGHT_VIOLET | #AD0AFD | rgb(0.678, 0.039, 0.992)
    #[classattr]
    pub fn BRIGHT_VIOLET() -> Color {
        Color { r: 0.6784313725490196, g: 0.0392156862745098, b: 0.9921568627450981, a: 1.0 }
    }

    /// PRUSSIAN_BLUE | #004577 | rgb(0.000, 0.271, 0.467)
    #[classattr]
    pub fn PRUSSIAN_BLUE() -> Color {
        Color { r: 0.0, g: 0.27058823529411763, b: 0.4666666666666667, a: 1.0 }
    }

    /// SLATE_GREEN | #658D6D | rgb(0.396, 0.553, 0.427)
    #[classattr]
    pub fn SLATE_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.5529411764705883, b: 0.42745098039215684, a: 1.0 }
    }

    /// DIRTY_PINK | #CA7B80 | rgb(0.792, 0.482, 0.502)
    #[classattr]
    pub fn DIRTY_PINK() -> Color {
        Color { r: 0.792156862745098, g: 0.4823529411764706, b: 0.5019607843137255, a: 1.0 }
    }

    /// DARK_BLUE_GREEN | #005249 | rgb(0.000, 0.322, 0.286)
    #[classattr]
    pub fn DARK_BLUE_GREEN() -> Color {
        Color { r: 0.0, g: 0.3215686274509804, b: 0.28627450980392155, a: 1.0 }
    }

    /// PINE | #2B5D34 | rgb(0.169, 0.365, 0.204)
    #[classattr]
    pub fn PINE() -> Color {
        Color { r: 0.16862745098039217, g: 0.36470588235294116, b: 0.20392156862745098, a: 1.0 }
    }

    /// YELLOWY_GREEN | #BFF128 | rgb(0.749, 0.945, 0.157)
    #[classattr]
    pub fn YELLOWY_GREEN() -> Color {
        Color { r: 0.7490196078431373, g: 0.9450980392156862, b: 0.1568627450980392, a: 1.0 }
    }

    /// DARK_GOLD | #B59410 | rgb(0.710, 0.580, 0.063)
    #[classattr]
    pub fn DARK_GOLD() -> Color {
        Color { r: 0.7098039215686275, g: 0.5803921568627451, b: 0.06274509803921569, a: 1.0 }
    }

    /// BLUISH | #2976BB | rgb(0.161, 0.463, 0.733)
    #[classattr]
    pub fn BLUISH() -> Color {
        Color { r: 0.1607843137254902, g: 0.4627450980392157, b: 0.7333333333333333, a: 1.0 }
    }

    /// DARKISH_BLUE | #014182 | rgb(0.004, 0.255, 0.510)
    #[classattr]
    pub fn DARKISH_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.2549019607843137, b: 0.5098039215686274, a: 1.0 }
    }

    /// DULL_RED | #BB3F3F | rgb(0.733, 0.247, 0.247)
    #[classattr]
    pub fn DULL_RED() -> Color {
        Color { r: 0.7333333333333333, g: 0.24705882352941178, b: 0.24705882352941178, a: 1.0 }
    }

    /// PINKY_RED | #FC2647 | rgb(0.988, 0.149, 0.278)
    #[classattr]
    pub fn PINKY_RED() -> Color {
        Color { r: 0.9882352941176471, g: 0.14901960784313725, b: 0.2784313725490196, a: 1.0 }
    }

    /// BRONZE | #A87900 | rgb(0.659, 0.475, 0.000)
    #[classattr]
    pub fn BRONZE() -> Color {
        Color { r: 0.6588235294117647, g: 0.4745098039215686, b: 0.0, a: 1.0 }
    }

    /// PALE_TEAL | #82CBB2 | rgb(0.510, 0.796, 0.698)
    #[classattr]
    pub fn PALE_TEAL() -> Color {
        Color { r: 0.5098039215686274, g: 0.796078431372549, b: 0.6980392156862745, a: 1.0 }
    }

    /// MILITARY_GREEN | #667C3E | rgb(0.400, 0.486, 0.243)
    #[classattr]
    pub fn MILITARY_GREEN() -> Color {
        Color { r: 0.4, g: 0.48627450980392156, b: 0.24313725490196078, a: 1.0 }
    }

    /// BARBIE_PINK | #FE46A5 | rgb(0.996, 0.275, 0.647)
    #[classattr]
    pub fn BARBIE_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.27450980392156865, b: 0.6470588235294118, a: 1.0 }
    }

    /// BUBBLEGUM_PINK | #FE83CC | rgb(0.996, 0.514, 0.800)
    #[classattr]
    pub fn BUBBLEGUM_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.5137254901960784, b: 0.8, a: 1.0 }
    }

    /// PEA_SOUP_GREEN | #94A617 | rgb(0.580, 0.651, 0.090)
    #[classattr]
    pub fn PEA_SOUP_GREEN() -> Color {
        Color { r: 0.5803921568627451, g: 0.6509803921568628, b: 0.09019607843137255, a: 1.0 }
    }

    /// DARK_MUSTARD | #A88905 | rgb(0.659, 0.537, 0.020)
    #[classattr]
    pub fn DARK_MUSTARD() -> Color {
        Color { r: 0.6588235294117647, g: 0.5372549019607843, b: 0.0196078431372549, a: 1.0 }
    }

    /// SHIT | #7F5F00 | rgb(0.498, 0.373, 0.000)
    #[classattr]
    pub fn SHIT() -> Color {
        Color { r: 0.4980392156862745, g: 0.37254901960784315, b: 0.0, a: 1.0 }
    }

    /// MEDIUM_PURPLE | #9E43A2 | rgb(0.620, 0.263, 0.635)
    #[classattr]
    pub fn MEDIUM_PURPLE() -> Color {
        Color { r: 0.6196078431372549, g: 0.2627450980392157, b: 0.6352941176470588, a: 1.0 }
    }

    /// VERY_DARK_GREEN | #062E03 | rgb(0.024, 0.180, 0.012)
    #[classattr]
    pub fn VERY_DARK_GREEN() -> Color {
        Color { r: 0.023529411764705882, g: 0.1803921568627451, b: 0.011764705882352941, a: 1.0 }
    }

    /// DIRT | #8A6E45 | rgb(0.541, 0.431, 0.271)
    #[classattr]
    pub fn DIRT() -> Color {
        Color { r: 0.5411764705882353, g: 0.43137254901960786, b: 0.27058823529411763, a: 1.0 }
    }

    /// DUSKY_PINK | #CC7A8B | rgb(0.800, 0.478, 0.545)
    #[classattr]
    pub fn DUSKY_PINK() -> Color {
        Color { r: 0.8, g: 0.47843137254901963, b: 0.5450980392156862, a: 1.0 }
    }

    /// RED_VIOLET | #9E0168 | rgb(0.620, 0.004, 0.408)
    #[classattr]
    pub fn RED_VIOLET() -> Color {
        Color { r: 0.6196078431372549, g: 0.00392156862745098, b: 0.40784313725490196, a: 1.0 }
    }

    /// LEMON_YELLOW | #FDFF38 | rgb(0.992, 1.000, 0.220)
    #[classattr]
    pub fn LEMON_YELLOW() -> Color {
        Color { r: 0.9921568627450981, g: 1.0, b: 0.2196078431372549, a: 1.0 }
    }

    /// PISTACHIO | #C0FA8B | rgb(0.753, 0.980, 0.545)
    #[classattr]
    pub fn PISTACHIO() -> Color {
        Color { r: 0.7529411764705882, g: 0.9803921568627451, b: 0.5450980392156862, a: 1.0 }
    }

    /// DULL_YELLOW | #EEDC5B | rgb(0.933, 0.863, 0.357)
    #[classattr]
    pub fn DULL_YELLOW() -> Color {
        Color { r: 0.9333333333333333, g: 0.8627450980392157, b: 0.3568627450980392, a: 1.0 }
    }

    /// DARK_LIME_GREEN | #7EBD01 | rgb(0.494, 0.741, 0.004)
    #[classattr]
    pub fn DARK_LIME_GREEN() -> Color {
        Color { r: 0.49411764705882355, g: 0.7411764705882353, b: 0.00392156862745098, a: 1.0 }
    }

    /// DENIM_BLUE | #3B5B92 | rgb(0.231, 0.357, 0.573)
    #[classattr]
    pub fn DENIM_BLUE() -> Color {
        Color { r: 0.23137254901960785, g: 0.3568627450980392, b: 0.5725490196078431, a: 1.0 }
    }

    /// TEAL_BLUE | #01889F | rgb(0.004, 0.533, 0.624)
    #[classattr]
    pub fn TEAL_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.5333333333333333, b: 0.6235294117647059, a: 1.0 }
    }

    /// LIGHTISH_BLUE | #3D7AFD | rgb(0.239, 0.478, 0.992)
    #[classattr]
    pub fn LIGHTISH_BLUE() -> Color {
        Color { r: 0.23921568627450981, g: 0.47843137254901963, b: 0.9921568627450981, a: 1.0 }
    }

    /// PURPLEY_BLUE | #5F34E7 | rgb(0.373, 0.204, 0.906)
    #[classattr]
    pub fn PURPLEY_BLUE() -> Color {
        Color { r: 0.37254901960784315, g: 0.20392156862745098, b: 0.9058823529411765, a: 1.0 }
    }

    /// LIGHT_INDIGO | #6D5ACF | rgb(0.427, 0.353, 0.812)
    #[classattr]
    pub fn LIGHT_INDIGO() -> Color {
        Color { r: 0.42745098039215684, g: 0.35294117647058826, b: 0.8117647058823529, a: 1.0 }
    }

    /// SWAMP_GREEN | #748500 | rgb(0.455, 0.522, 0.000)
    #[classattr]
    pub fn SWAMP_GREEN() -> Color {
        Color { r: 0.4549019607843137, g: 0.5215686274509804, b: 0.0, a: 1.0 }
    }

    /// BROWN_GREEN | #706C11 | rgb(0.439, 0.424, 0.067)
    #[classattr]
    pub fn BROWN_GREEN() -> Color {
        Color { r: 0.4392156862745098, g: 0.4235294117647059, b: 0.06666666666666667, a: 1.0 }
    }

    /// DARK_MAROON | #3C0008 | rgb(0.235, 0.000, 0.031)
    #[classattr]
    pub fn DARK_MAROON() -> Color {
        Color { r: 0.23529411764705882, g: 0.0, b: 0.03137254901960784, a: 1.0 }
    }

    /// HOT_PURPLE | #CB00F5 | rgb(0.796, 0.000, 0.961)
    #[classattr]
    pub fn HOT_PURPLE() -> Color {
        Color { r: 0.796078431372549, g: 0.0, b: 0.9607843137254902, a: 1.0 }
    }

    /// DARK_FOREST_GREEN | #002D04 | rgb(0.000, 0.176, 0.016)
    #[classattr]
    pub fn DARK_FOREST_GREEN() -> Color {
        Color { r: 0.0, g: 0.17647058823529413, b: 0.01568627450980392, a: 1.0 }
    }

    /// FADED_BLUE | #658CBB | rgb(0.396, 0.549, 0.733)
    #[classattr]
    pub fn FADED_BLUE() -> Color {
        Color { r: 0.396078431372549, g: 0.5490196078431373, b: 0.7333333333333333, a: 1.0 }
    }

    /// DRAB_GREEN | #749551 | rgb(0.455, 0.584, 0.318)
    #[classattr]
    pub fn DRAB_GREEN() -> Color {
        Color { r: 0.4549019607843137, g: 0.5843137254901961, b: 0.3176470588235294, a: 1.0 }
    }

    /// LIGHT_LIME_GREEN | #B9FF66 | rgb(0.725, 1.000, 0.400)
    #[classattr]
    pub fn LIGHT_LIME_GREEN() -> Color {
        Color { r: 0.7254901960784313, g: 1.0, b: 0.4, a: 1.0 }
    }

    /// SNOT_GREEN | #9DC100 | rgb(0.616, 0.757, 0.000)
    #[classattr]
    pub fn SNOT_GREEN() -> Color {
        Color { r: 0.615686274509804, g: 0.7568627450980392, b: 0.0, a: 1.0 }
    }

    /// YELLOWISH | #FAEE66 | rgb(0.980, 0.933, 0.400)
    #[classattr]
    pub fn YELLOWISH() -> Color {
        Color { r: 0.9803921568627451, g: 0.9333333333333333, b: 0.4, a: 1.0 }
    }

    /// LIGHT_BLUE_GREEN | #7EFBB3 | rgb(0.494, 0.984, 0.702)
    #[classattr]
    pub fn LIGHT_BLUE_GREEN() -> Color {
        Color { r: 0.49411764705882355, g: 0.984313725490196, b: 0.7019607843137254, a: 1.0 }
    }

    /// BORDEAUX | #7B002C | rgb(0.482, 0.000, 0.173)
    #[classattr]
    pub fn BORDEAUX() -> Color {
        Color { r: 0.4823529411764706, g: 0.0, b: 0.17254901960784313, a: 1.0 }
    }

    /// LIGHT_MAUVE | #C292A1 | rgb(0.761, 0.573, 0.631)
    #[classattr]
    pub fn LIGHT_MAUVE() -> Color {
        Color { r: 0.7607843137254902, g: 0.5725490196078431, b: 0.6313725490196078, a: 1.0 }
    }

    /// OCEAN | #017B92 | rgb(0.004, 0.482, 0.573)
    #[classattr]
    pub fn OCEAN() -> Color {
        Color { r: 0.00392156862745098, g: 0.4823529411764706, b: 0.5725490196078431, a: 1.0 }
    }

    /// MARIGOLD | #FCC006 | rgb(0.988, 0.753, 0.024)
    #[classattr]
    pub fn MARIGOLD() -> Color {
        Color { r: 0.9882352941176471, g: 0.7529411764705882, b: 0.023529411764705882, a: 1.0 }
    }

    /// MUDDY_GREEN | #657432 | rgb(0.396, 0.455, 0.196)
    #[classattr]
    pub fn MUDDY_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.4549019607843137, b: 0.19607843137254902, a: 1.0 }
    }

    /// DULL_ORANGE | #D8863B | rgb(0.847, 0.525, 0.231)
    #[classattr]
    pub fn DULL_ORANGE() -> Color {
        Color { r: 0.8470588235294118, g: 0.5254901960784314, b: 0.23137254901960785, a: 1.0 }
    }

    /// STEEL | #738595 | rgb(0.451, 0.522, 0.584)
    #[classattr]
    pub fn STEEL() -> Color {
        Color { r: 0.45098039215686275, g: 0.5215686274509804, b: 0.5843137254901961, a: 1.0 }
    }

    /// ELECTRIC_PURPLE | #AA23FF | rgb(0.667, 0.137, 1.000)
    #[classattr]
    pub fn ELECTRIC_PURPLE() -> Color {
        Color { r: 0.6666666666666666, g: 0.13725490196078433, b: 1.0, a: 1.0 }
    }

    /// FLUORESCENT_GREEN | #08FF08 | rgb(0.031, 1.000, 0.031)
    #[classattr]
    pub fn FLUORESCENT_GREEN() -> Color {
        Color { r: 0.03137254901960784, g: 1.0, b: 0.03137254901960784, a: 1.0 }
    }

    /// YELLOWISH_BROWN | #9B7A01 | rgb(0.608, 0.478, 0.004)
    #[classattr]
    pub fn YELLOWISH_BROWN() -> Color {
        Color { r: 0.6078431372549019, g: 0.47843137254901963, b: 0.00392156862745098, a: 1.0 }
    }

    /// BLUSH | #F29E8E | rgb(0.949, 0.620, 0.557)
    #[classattr]
    pub fn BLUSH() -> Color {
        Color { r: 0.9490196078431372, g: 0.6196078431372549, b: 0.5568627450980392, a: 1.0 }
    }

    /// SOFT_GREEN | #6FC276 | rgb(0.435, 0.761, 0.463)
    #[classattr]
    pub fn SOFT_GREEN() -> Color {
        Color { r: 0.43529411764705883, g: 0.7607843137254902, b: 0.4627450980392157, a: 1.0 }
    }

    /// BRIGHT_ORANGE | #FF5B00 | rgb(1.000, 0.357, 0.000)
    #[classattr]
    pub fn BRIGHT_ORANGE() -> Color {
        Color { r: 1.0, g: 0.3568627450980392, b: 0.0, a: 1.0 }
    }

    /// LEMON | #FDFF52 | rgb(0.992, 1.000, 0.322)
    #[classattr]
    pub fn LEMON() -> Color {
        Color { r: 0.9921568627450981, g: 1.0, b: 0.3215686274509804, a: 1.0 }
    }

    /// PURPLE_GREY | #866F85 | rgb(0.525, 0.435, 0.522)
    #[classattr]
    pub fn PURPLE_GREY() -> Color {
        Color { r: 0.5254901960784314, g: 0.43529411764705883, b: 0.5215686274509804, a: 1.0 }
    }

    /// ACID_GREEN | #8FFE09 | rgb(0.561, 0.996, 0.035)
    #[classattr]
    pub fn ACID_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.996078431372549, b: 0.03529411764705882, a: 1.0 }
    }

    /// PALE_LAVENDER | #EECFFE | rgb(0.933, 0.812, 0.996)
    #[classattr]
    pub fn PALE_LAVENDER() -> Color {
        Color { r: 0.9333333333333333, g: 0.8117647058823529, b: 0.996078431372549, a: 1.0 }
    }

    /// VIOLET_BLUE | #510AC9 | rgb(0.318, 0.039, 0.788)
    #[classattr]
    pub fn VIOLET_BLUE() -> Color {
        Color { r: 0.3176470588235294, g: 0.0392156862745098, b: 0.788235294117647, a: 1.0 }
    }

    /// LIGHT_FOREST_GREEN | #4F9153 | rgb(0.310, 0.569, 0.325)
    #[classattr]
    pub fn LIGHT_FOREST_GREEN() -> Color {
        Color { r: 0.30980392156862746, g: 0.5686274509803921, b: 0.3254901960784314, a: 1.0 }
    }

    /// BURNT_RED | #9F2305 | rgb(0.624, 0.137, 0.020)
    #[classattr]
    pub fn BURNT_RED() -> Color {
        Color { r: 0.6235294117647059, g: 0.13725490196078433, b: 0.0196078431372549, a: 1.0 }
    }

    /// KHAKI_GREEN | #728639 | rgb(0.447, 0.525, 0.224)
    #[classattr]
    pub fn KHAKI_GREEN() -> Color {
        Color { r: 0.4470588235294118, g: 0.5254901960784314, b: 0.2235294117647059, a: 1.0 }
    }

    /// CERISE | #DE0C62 | rgb(0.871, 0.047, 0.384)
    #[classattr]
    pub fn CERISE() -> Color {
        Color { r: 0.8705882352941177, g: 0.047058823529411764, b: 0.3843137254901961, a: 1.0 }
    }

    /// FADED_PURPLE | #916E99 | rgb(0.569, 0.431, 0.600)
    #[classattr]
    pub fn FADED_PURPLE() -> Color {
        Color { r: 0.5686274509803921, g: 0.43137254901960786, b: 0.6, a: 1.0 }
    }

    /// APRICOT | #FFB16D | rgb(1.000, 0.694, 0.427)
    #[classattr]
    pub fn APRICOT() -> Color {
        Color { r: 1.0, g: 0.6941176470588235, b: 0.42745098039215684, a: 1.0 }
    }

    /// DARK_OLIVE_GREEN | #3C4D03 | rgb(0.235, 0.302, 0.012)
    #[classattr]
    pub fn DARK_OLIVE_GREEN() -> Color {
        Color { r: 0.23529411764705882, g: 0.30196078431372547, b: 0.011764705882352941, a: 1.0 }
    }

    /// GREY_BROWN | #7F7053 | rgb(0.498, 0.439, 0.325)
    #[classattr]
    pub fn GREY_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.4392156862745098, b: 0.3254901960784314, a: 1.0 }
    }

    /// GREEN_GREY | #77926F | rgb(0.467, 0.573, 0.435)
    #[classattr]
    pub fn GREEN_GREY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5725490196078431, b: 0.43529411764705883, a: 1.0 }
    }

    /// TRUE_BLUE | #010FCC | rgb(0.004, 0.059, 0.800)
    #[classattr]
    pub fn TRUE_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.058823529411764705, b: 0.8, a: 1.0 }
    }

    /// PALE_VIOLET | #CEAEFA | rgb(0.808, 0.682, 0.980)
    #[classattr]
    pub fn PALE_VIOLET() -> Color {
        Color { r: 0.807843137254902, g: 0.6823529411764706, b: 0.9803921568627451, a: 1.0 }
    }

    /// PERIWINKLE_BLUE | #8F99FB | rgb(0.561, 0.600, 0.984)
    #[classattr]
    pub fn PERIWINKLE_BLUE() -> Color {
        Color { r: 0.5607843137254902, g: 0.6, b: 0.984313725490196, a: 1.0 }
    }

    /// LIGHT_SKY_BLUE | #C6FCFF | rgb(0.776, 0.988, 1.000)
    #[classattr]
    pub fn LIGHT_SKY_BLUE() -> Color {
        Color { r: 0.7764705882352941, g: 0.9882352941176471, b: 1.0, a: 1.0 }
    }

    /// BLURPLE | #5539CC | rgb(0.333, 0.224, 0.800)
    #[classattr]
    pub fn BLURPLE() -> Color {
        Color { r: 0.3333333333333333, g: 0.2235294117647059, b: 0.8, a: 1.0 }
    }

    /// GREEN_BROWN | #544E03 | rgb(0.329, 0.306, 0.012)
    #[classattr]
    pub fn GREEN_BROWN() -> Color {
        Color { r: 0.32941176470588235, g: 0.3058823529411765, b: 0.011764705882352941, a: 1.0 }
    }

    /// BLUEGREEN | #017A79 | rgb(0.004, 0.478, 0.475)
    #[classattr]
    pub fn BLUEGREEN() -> Color {
        Color { r: 0.00392156862745098, g: 0.47843137254901963, b: 0.4745098039215686, a: 1.0 }
    }

    /// BRIGHT_TEAL | #01F9C6 | rgb(0.004, 0.976, 0.776)
    #[classattr]
    pub fn BRIGHT_TEAL() -> Color {
        Color { r: 0.00392156862745098, g: 0.9764705882352941, b: 0.7764705882352941, a: 1.0 }
    }

    /// BROWNISH_YELLOW | #C9B003 | rgb(0.788, 0.690, 0.012)
    #[classattr]
    pub fn BROWNISH_YELLOW() -> Color {
        Color { r: 0.788235294117647, g: 0.6901960784313725, b: 0.011764705882352941, a: 1.0 }
    }

    /// PEA_SOUP | #929901 | rgb(0.573, 0.600, 0.004)
    #[classattr]
    pub fn PEA_SOUP() -> Color {
        Color { r: 0.5725490196078431, g: 0.6, b: 0.00392156862745098, a: 1.0 }
    }

    /// FOREST | #0B5509 | rgb(0.043, 0.333, 0.035)
    #[classattr]
    pub fn FOREST() -> Color {
        Color { r: 0.043137254901960784, g: 0.3333333333333333, b: 0.03529411764705882, a: 1.0 }
    }

    /// BARNEY_PURPLE | #A00498 | rgb(0.627, 0.016, 0.596)
    #[classattr]
    pub fn BARNEY_PURPLE() -> Color {
        Color { r: 0.6274509803921569, g: 0.01568627450980392, b: 0.596078431372549, a: 1.0 }
    }

    /// ULTRAMARINE | #2000B1 | rgb(0.125, 0.000, 0.694)
    #[classattr]
    pub fn ULTRAMARINE() -> Color {
        Color { r: 0.12549019607843137, g: 0.0, b: 0.6941176470588235, a: 1.0 }
    }

    /// PURPLISH | #94568C | rgb(0.580, 0.337, 0.549)
    #[classattr]
    pub fn PURPLISH() -> Color {
        Color { r: 0.5803921568627451, g: 0.33725490196078434, b: 0.5490196078431373, a: 1.0 }
    }

    /// PUKE_YELLOW | #C2BE0E | rgb(0.761, 0.745, 0.055)
    #[classattr]
    pub fn PUKE_YELLOW() -> Color {
        Color { r: 0.7607843137254902, g: 0.7450980392156863, b: 0.054901960784313725, a: 1.0 }
    }

    /// BLUISH_GREY | #748B97 | rgb(0.455, 0.545, 0.592)
    #[classattr]
    pub fn BLUISH_GREY() -> Color {
        Color { r: 0.4549019607843137, g: 0.5450980392156862, b: 0.592156862745098, a: 1.0 }
    }

    /// DARK_PERIWINKLE | #665FD1 | rgb(0.400, 0.373, 0.820)
    #[classattr]
    pub fn DARK_PERIWINKLE() -> Color {
        Color { r: 0.4, g: 0.37254901960784315, b: 0.8196078431372549, a: 1.0 }
    }

    /// DARK_LILAC | #9C6DA5 | rgb(0.612, 0.427, 0.647)
    #[classattr]
    pub fn DARK_LILAC() -> Color {
        Color { r: 0.611764705882353, g: 0.42745098039215684, b: 0.6470588235294118, a: 1.0 }
    }

    /// REDDISH | #C44240 | rgb(0.769, 0.259, 0.251)
    #[classattr]
    pub fn REDDISH() -> Color {
        Color { r: 0.7686274509803922, g: 0.25882352941176473, b: 0.25098039215686274, a: 1.0 }
    }

    /// LIGHT_MAROON | #A24857 | rgb(0.635, 0.282, 0.341)
    #[classattr]
    pub fn LIGHT_MAROON() -> Color {
        Color { r: 0.6352941176470588, g: 0.2823529411764706, b: 0.3411764705882353, a: 1.0 }
    }

    /// DUSTY_PURPLE | #825F87 | rgb(0.510, 0.373, 0.529)
    #[classattr]
    pub fn DUSTY_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.37254901960784315, b: 0.5294117647058824, a: 1.0 }
    }

    /// TERRA_COTTA | #C9643B | rgb(0.788, 0.392, 0.231)
    #[classattr]
    pub fn TERRA_COTTA() -> Color {
        Color { r: 0.788235294117647, g: 0.39215686274509803, b: 0.23137254901960785, a: 1.0 }
    }

    /// AVOCADO | #90B134 | rgb(0.565, 0.694, 0.204)
    #[classattr]
    pub fn AVOCADO() -> Color {
        Color { r: 0.5647058823529412, g: 0.6941176470588235, b: 0.20392156862745098, a: 1.0 }
    }

    /// MARINE_BLUE | #01386A | rgb(0.004, 0.220, 0.416)
    #[classattr]
    pub fn MARINE_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.2196078431372549, b: 0.41568627450980394, a: 1.0 }
    }

    /// TEAL_GREEN | #25A36F | rgb(0.145, 0.639, 0.435)
    #[classattr]
    pub fn TEAL_GREEN() -> Color {
        Color { r: 0.1450980392156863, g: 0.6392156862745098, b: 0.43529411764705883, a: 1.0 }
    }

    /// SLATE_GREY | #59656D | rgb(0.349, 0.396, 0.427)
    #[classattr]
    pub fn SLATE_GREY() -> Color {
        Color { r: 0.34901960784313724, g: 0.396078431372549, b: 0.42745098039215684, a: 1.0 }
    }

    /// LIGHTER_GREEN | #75FD63 | rgb(0.459, 0.992, 0.388)
    #[classattr]
    pub fn LIGHTER_GREEN() -> Color {
        Color { r: 0.4588235294117647, g: 0.9921568627450981, b: 0.38823529411764707, a: 1.0 }
    }

    /// ELECTRIC_GREEN | #21FC0D | rgb(0.129, 0.988, 0.051)
    #[classattr]
    pub fn ELECTRIC_GREEN() -> Color {
        Color { r: 0.12941176470588237, g: 0.9882352941176471, b: 0.050980392156862744, a: 1.0 }
    }

    /// DUSTY_BLUE | #5A86AD | rgb(0.353, 0.525, 0.678)
    #[classattr]
    pub fn DUSTY_BLUE() -> Color {
        Color { r: 0.35294117647058826, g: 0.5254901960784314, b: 0.6784313725490196, a: 1.0 }
    }

    /// GOLDEN_YELLOW | #FEC615 | rgb(0.996, 0.776, 0.082)
    #[classattr]
    pub fn GOLDEN_YELLOW() -> Color {
        Color { r: 0.996078431372549, g: 0.7764705882352941, b: 0.08235294117647059, a: 1.0 }
    }

    /// BRIGHT_YELLOW | #FFFD01 | rgb(1.000, 0.992, 0.004)
    #[classattr]
    pub fn BRIGHT_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.00392156862745098, a: 1.0 }
    }

    /// LIGHT_LAVENDER | #DFC5FE | rgb(0.875, 0.773, 0.996)
    #[classattr]
    pub fn LIGHT_LAVENDER() -> Color {
        Color { r: 0.8745098039215686, g: 0.7725490196078432, b: 0.996078431372549, a: 1.0 }
    }

    /// UMBER | #B26400 | rgb(0.698, 0.392, 0.000)
    #[classattr]
    pub fn UMBER() -> Color {
        Color { r: 0.6980392156862745, g: 0.39215686274509803, b: 0.0, a: 1.0 }
    }

    /// POOP | #7F5E00 | rgb(0.498, 0.369, 0.000)
    #[classattr]
    pub fn POOP() -> Color {
        Color { r: 0.4980392156862745, g: 0.3686274509803922, b: 0.0, a: 1.0 }
    }

    /// DARK_PEACH | #DE7E5D | rgb(0.871, 0.494, 0.365)
    #[classattr]
    pub fn DARK_PEACH() -> Color {
        Color { r: 0.8705882352941177, g: 0.49411764705882355, b: 0.36470588235294116, a: 1.0 }
    }

    /// JUNGLE_GREEN | #048243 | rgb(0.016, 0.510, 0.263)
    #[classattr]
    pub fn JUNGLE_GREEN() -> Color {
        Color { r: 0.01568627450980392, g: 0.5098039215686274, b: 0.2627450980392157, a: 1.0 }
    }

    /// EGGSHELL | #FFFFD4 | rgb(1.000, 1.000, 0.831)
    #[classattr]
    pub fn EGGSHELL() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.8313725490196079, a: 1.0 }
    }

    /// DENIM | #3B638C | rgb(0.231, 0.388, 0.549)
    #[classattr]
    pub fn DENIM() -> Color {
        Color { r: 0.23137254901960785, g: 0.38823529411764707, b: 0.5490196078431373, a: 1.0 }
    }

    /// YELLOW_BROWN | #B79400 | rgb(0.718, 0.580, 0.000)
    #[classattr]
    pub fn YELLOW_BROWN() -> Color {
        Color { r: 0.7176470588235294, g: 0.5803921568627451, b: 0.0, a: 1.0 }
    }

    /// DULL_PURPLE | #84597E | rgb(0.518, 0.349, 0.494)
    #[classattr]
    pub fn DULL_PURPLE() -> Color {
        Color { r: 0.5176470588235295, g: 0.34901960784313724, b: 0.49411764705882355, a: 1.0 }
    }

    /// CHOCOLATE_BROWN | #411900 | rgb(0.255, 0.098, 0.000)
    #[classattr]
    pub fn CHOCOLATE_BROWN() -> Color {
        Color { r: 0.2549019607843137, g: 0.09803921568627451, b: 0.0, a: 1.0 }
    }

    /// WINE_RED | #7B0323 | rgb(0.482, 0.012, 0.137)
    #[classattr]
    pub fn WINE_RED() -> Color {
        Color { r: 0.4823529411764706, g: 0.011764705882352941, b: 0.13725490196078433, a: 1.0 }
    }

    /// NEON_BLUE | #04D9FF | rgb(0.016, 0.851, 1.000)
    #[classattr]
    pub fn NEON_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.8509803921568627, b: 1.0, a: 1.0 }
    }

    /// DIRTY_GREEN | #667E2C | rgb(0.400, 0.494, 0.173)
    #[classattr]
    pub fn DIRTY_GREEN() -> Color {
        Color { r: 0.4, g: 0.49411764705882355, b: 0.17254901960784313, a: 1.0 }
    }

    /// LIGHT_TAN | #FBEEAC | rgb(0.984, 0.933, 0.675)
    #[classattr]
    pub fn LIGHT_TAN() -> Color {
        Color { r: 0.984313725490196, g: 0.9333333333333333, b: 0.6745098039215687, a: 1.0 }
    }

    /// ICE_BLUE | #D7FFFE | rgb(0.843, 1.000, 0.996)
    #[classattr]
    pub fn ICE_BLUE() -> Color {
        Color { r: 0.8431372549019608, g: 1.0, b: 0.996078431372549, a: 1.0 }
    }

    /// CADET_BLUE | #4E7496 | rgb(0.306, 0.455, 0.588)
    #[classattr]
    pub fn CADET_BLUE() -> Color {
        Color { r: 0.3058823529411765, g: 0.4549019607843137, b: 0.5882352941176471, a: 1.0 }
    }

    /// DARK_MAUVE | #874C62 | rgb(0.529, 0.298, 0.384)
    #[classattr]
    pub fn DARK_MAUVE() -> Color {
        Color { r: 0.5294117647058824, g: 0.2980392156862745, b: 0.3843137254901961, a: 1.0 }
    }

    /// VERY_LIGHT_BLUE | #D5FFFF | rgb(0.835, 1.000, 1.000)
    #[classattr]
    pub fn VERY_LIGHT_BLUE() -> Color {
        Color { r: 0.8352941176470589, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// GREY_PURPLE | #826D8C | rgb(0.510, 0.427, 0.549)
    #[classattr]
    pub fn GREY_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.42745098039215684, b: 0.5490196078431373, a: 1.0 }
    }

    /// PASTEL_PINK | #FFBACD | rgb(1.000, 0.729, 0.804)
    #[classattr]
    pub fn PASTEL_PINK() -> Color {
        Color { r: 1.0, g: 0.7294117647058823, b: 0.803921568627451, a: 1.0 }
    }

    /// VERY_LIGHT_GREEN | #D1FFBD | rgb(0.820, 1.000, 0.741)
    #[classattr]
    pub fn VERY_LIGHT_GREEN() -> Color {
        Color { r: 0.8196078431372549, g: 1.0, b: 0.7411764705882353, a: 1.0 }
    }

    /// DARK_SKY_BLUE | #448EE4 | rgb(0.267, 0.557, 0.894)
    #[classattr]
    pub fn DARK_SKY_BLUE() -> Color {
        Color { r: 0.26666666666666666, g: 0.5568627450980392, b: 0.8941176470588236, a: 1.0 }
    }

    /// EVERGREEN | #05472A | rgb(0.020, 0.278, 0.165)
    #[classattr]
    pub fn EVERGREEN() -> Color {
        Color { r: 0.0196078431372549, g: 0.2784313725490196, b: 0.16470588235294117, a: 1.0 }
    }

    /// DULL_PINK | #D5869D | rgb(0.835, 0.525, 0.616)
    #[classattr]
    pub fn DULL_PINK() -> Color {
        Color { r: 0.8352941176470589, g: 0.5254901960784314, b: 0.615686274509804, a: 1.0 }
    }

    /// AUBERGINE | #3D0734 | rgb(0.239, 0.027, 0.204)
    #[classattr]
    pub fn AUBERGINE() -> Color {
        Color { r: 0.23921568627450981, g: 0.027450980392156862, b: 0.20392156862745098, a: 1.0 }
    }

    /// MAHOGANY | #4A0100 | rgb(0.290, 0.004, 0.000)
    #[classattr]
    pub fn MAHOGANY() -> Color {
        Color { r: 0.2901960784313726, g: 0.00392156862745098, b: 0.0, a: 1.0 }
    }

    /// REDDISH_ORANGE | #F8481C | rgb(0.973, 0.282, 0.110)
    #[classattr]
    pub fn REDDISH_ORANGE() -> Color {
        Color { r: 0.9725490196078431, g: 0.2823529411764706, b: 0.10980392156862745, a: 1.0 }
    }

    /// DEEP_GREEN | #02590F | rgb(0.008, 0.349, 0.059)
    #[classattr]
    pub fn DEEP_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.34901960784313724, b: 0.058823529411764705, a: 1.0 }
    }

    /// VOMIT_GREEN | #89A203 | rgb(0.537, 0.635, 0.012)
    #[classattr]
    pub fn VOMIT_GREEN() -> Color {
        Color { r: 0.5372549019607843, g: 0.6352941176470588, b: 0.011764705882352941, a: 1.0 }
    }

    /// DUSTY_PINK | #D58A94 | rgb(0.835, 0.541, 0.580)
    #[classattr]
    pub fn DUSTY_PINK() -> Color {
        Color { r: 0.8352941176470589, g: 0.5411764705882353, b: 0.5803921568627451, a: 1.0 }
    }

    /// FADED_GREEN | #7BB274 | rgb(0.482, 0.698, 0.455)
    #[classattr]
    pub fn FADED_GREEN() -> Color {
        Color { r: 0.4823529411764706, g: 0.6980392156862745, b: 0.4549019607843137, a: 1.0 }
    }

    /// CAMO_GREEN | #526525 | rgb(0.322, 0.396, 0.145)
    #[classattr]
    pub fn CAMO_GREEN() -> Color {
        Color { r: 0.3215686274509804, g: 0.396078431372549, b: 0.1450980392156863, a: 1.0 }
    }

    /// PINKY_PURPLE | #C94CBE | rgb(0.788, 0.298, 0.745)
    #[classattr]
    pub fn PINKY_PURPLE() -> Color {
        Color { r: 0.788235294117647, g: 0.2980392156862745, b: 0.7450980392156863, a: 1.0 }
    }

    /// BROWNISH_RED | #9E3623 | rgb(0.620, 0.212, 0.137)
    #[classattr]
    pub fn BROWNISH_RED() -> Color {
        Color { r: 0.6196078431372549, g: 0.21176470588235294, b: 0.13725490196078433, a: 1.0 }
    }

    /// DARK_ROSE | #B5485D | rgb(0.710, 0.282, 0.365)
    #[classattr]
    pub fn DARK_ROSE() -> Color {
        Color { r: 0.7098039215686275, g: 0.2823529411764706, b: 0.36470588235294116, a: 1.0 }
    }

    /// MUD | #735C12 | rgb(0.451, 0.361, 0.071)
    #[classattr]
    pub fn MUD() -> Color {
        Color { r: 0.45098039215686275, g: 0.3607843137254902, b: 0.07058823529411765, a: 1.0 }
    }

    /// BROWNISH | #9C6D57 | rgb(0.612, 0.427, 0.341)
    #[classattr]
    pub fn BROWNISH() -> Color {
        Color { r: 0.611764705882353, g: 0.42745098039215684, b: 0.3411764705882353, a: 1.0 }
    }

    /// EMERALD_GREEN | #028F1E | rgb(0.008, 0.561, 0.118)
    #[classattr]
    pub fn EMERALD_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.5607843137254902, b: 0.11764705882352941, a: 1.0 }
    }

    /// PALE_BROWN | #B1916E | rgb(0.694, 0.569, 0.431)
    #[classattr]
    pub fn PALE_BROWN() -> Color {
        Color { r: 0.6941176470588235, g: 0.5686274509803921, b: 0.43137254901960786, a: 1.0 }
    }

    /// DULL_BLUE | #49759C | rgb(0.286, 0.459, 0.612)
    #[classattr]
    pub fn DULL_BLUE() -> Color {
        Color { r: 0.28627450980392155, g: 0.4588235294117647, b: 0.611764705882353, a: 1.0 }
    }

    /// BURNT_UMBER | #A0450E | rgb(0.627, 0.271, 0.055)
    #[classattr]
    pub fn BURNT_UMBER() -> Color {
        Color { r: 0.6274509803921569, g: 0.27058823529411763, b: 0.054901960784313725, a: 1.0 }
    }

    /// MEDIUM_GREEN | #39AD48 | rgb(0.224, 0.678, 0.282)
    #[classattr]
    pub fn MEDIUM_GREEN() -> Color {
        Color { r: 0.2235294117647059, g: 0.6784313725490196, b: 0.2823529411764706, a: 1.0 }
    }

    /// CLAY | #B66A50 | rgb(0.714, 0.416, 0.314)
    #[classattr]
    pub fn CLAY() -> Color {
        Color { r: 0.7137254901960784, g: 0.41568627450980394, b: 0.3137254901960784, a: 1.0 }
    }

    /// LIGHT_AQUA | #8CFFDB | rgb(0.549, 1.000, 0.859)
    #[classattr]
    pub fn LIGHT_AQUA() -> Color {
        Color { r: 0.5490196078431373, g: 1.0, b: 0.8588235294117647, a: 1.0 }
    }

    /// LIGHT_OLIVE_GREEN | #A4BE5C | rgb(0.643, 0.745, 0.361)
    #[classattr]
    pub fn LIGHT_OLIVE_GREEN() -> Color {
        Color { r: 0.6431372549019608, g: 0.7450980392156863, b: 0.3607843137254902, a: 1.0 }
    }

    /// BROWNISH_ORANGE | #CB7723 | rgb(0.796, 0.467, 0.137)
    #[classattr]
    pub fn BROWNISH_ORANGE() -> Color {
        Color { r: 0.796078431372549, g: 0.4666666666666667, b: 0.13725490196078433, a: 1.0 }
    }

    /// DARK_AQUA | #05696B | rgb(0.020, 0.412, 0.420)
    #[classattr]
    pub fn DARK_AQUA() -> Color {
        Color { r: 0.0196078431372549, g: 0.4117647058823529, b: 0.4196078431372549, a: 1.0 }
    }

    /// PURPLISH_PINK | #CE5DAE | rgb(0.808, 0.365, 0.682)
    #[classattr]
    pub fn PURPLISH_PINK() -> Color {
        Color { r: 0.807843137254902, g: 0.36470588235294116, b: 0.6823529411764706, a: 1.0 }
    }

    /// DARK_SALMON | #C85A53 | rgb(0.784, 0.353, 0.325)
    #[classattr]
    pub fn DARK_SALMON() -> Color {
        Color { r: 0.7843137254901961, g: 0.35294117647058826, b: 0.3254901960784314, a: 1.0 }
    }

    /// GREENISH_GREY | #96AE8D | rgb(0.588, 0.682, 0.553)
    #[classattr]
    pub fn GREENISH_GREY() -> Color {
        Color { r: 0.5882352941176471, g: 0.6823529411764706, b: 0.5529411764705883, a: 1.0 }
    }

    /// JADE | #1FA774 | rgb(0.122, 0.655, 0.455)
    #[classattr]
    pub fn JADE() -> Color {
        Color { r: 0.12156862745098039, g: 0.6549019607843137, b: 0.4549019607843137, a: 1.0 }
    }

    /// UGLY_GREEN | #7A9703 | rgb(0.478, 0.592, 0.012)
    #[classattr]
    pub fn UGLY_GREEN() -> Color {
        Color { r: 0.47843137254901963, g: 0.592156862745098, b: 0.011764705882352941, a: 1.0 }
    }

    /// DARK_BEIGE | #AC9362 | rgb(0.675, 0.576, 0.384)
    #[classattr]
    pub fn DARK_BEIGE() -> Color {
        Color { r: 0.6745098039215687, g: 0.5764705882352941, b: 0.3843137254901961, a: 1.0 }
    }

    /// EMERALD | #01A049 | rgb(0.004, 0.627, 0.286)
    #[classattr]
    pub fn EMERALD() -> Color {
        Color { r: 0.00392156862745098, g: 0.6274509803921569, b: 0.28627450980392155, a: 1.0 }
    }

    /// PALE_RED | #D9544D | rgb(0.851, 0.329, 0.302)
    #[classattr]
    pub fn PALE_RED() -> Color {
        Color { r: 0.8509803921568627, g: 0.32941176470588235, b: 0.30196078431372547, a: 1.0 }
    }

    /// LIGHT_MAGENTA | #FA5FF7 | rgb(0.980, 0.373, 0.969)
    #[classattr]
    pub fn LIGHT_MAGENTA() -> Color {
        Color { r: 0.9803921568627451, g: 0.37254901960784315, b: 0.9686274509803922, a: 1.0 }
    }

    /// SKY | #82CAFC | rgb(0.510, 0.792, 0.988)
    #[classattr]
    pub fn SKY() -> Color {
        Color { r: 0.5098039215686274, g: 0.792156862745098, b: 0.9882352941176471, a: 1.0 }
    }

    /// LIGHT_CYAN | #ACFFFC | rgb(0.675, 1.000, 0.988)
    #[classattr]
    pub fn LIGHT_CYAN() -> Color {
        Color { r: 0.6745098039215687, g: 1.0, b: 0.9882352941176471, a: 1.0 }
    }

    /// YELLOW_ORANGE | #FCB001 | rgb(0.988, 0.690, 0.004)
    #[classattr]
    pub fn YELLOW_ORANGE() -> Color {
        Color { r: 0.9882352941176471, g: 0.6901960784313725, b: 0.00392156862745098, a: 1.0 }
    }

    /// REDDISH_PURPLE | #910951 | rgb(0.569, 0.035, 0.318)
    #[classattr]
    pub fn REDDISH_PURPLE() -> Color {
        Color { r: 0.5686274509803921, g: 0.03529411764705882, b: 0.3176470588235294, a: 1.0 }
    }

    /// REDDISH_PINK | #FE2C54 | rgb(0.996, 0.173, 0.329)
    #[classattr]
    pub fn REDDISH_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.17254901960784313, b: 0.32941176470588235, a: 1.0 }
    }

    /// ORCHID | #C875C4 | rgb(0.784, 0.459, 0.769)
    #[classattr]
    pub fn ORCHID() -> Color {
        Color { r: 0.7843137254901961, g: 0.4588235294117647, b: 0.7686274509803922, a: 1.0 }
    }

    /// DIRTY_YELLOW | #CDC50A | rgb(0.804, 0.773, 0.039)
    #[classattr]
    pub fn DIRTY_YELLOW() -> Color {
        Color { r: 0.803921568627451, g: 0.7725490196078432, b: 0.0392156862745098, a: 1.0 }
    }

    /// ORANGE_RED | #FD411E | rgb(0.992, 0.255, 0.118)
    #[classattr]
    pub fn ORANGE_RED() -> Color {
        Color { r: 0.9921568627450981, g: 0.2549019607843137, b: 0.11764705882352941, a: 1.0 }
    }

    /// DEEP_RED | #9A0200 | rgb(0.604, 0.008, 0.000)
    #[classattr]
    pub fn DEEP_RED() -> Color {
        Color { r: 0.6039215686274509, g: 0.00784313725490196, b: 0.0, a: 1.0 }
    }

    /// ORANGE_BROWN | #BE6400 | rgb(0.745, 0.392, 0.000)
    #[classattr]
    pub fn ORANGE_BROWN() -> Color {
        Color { r: 0.7450980392156863, g: 0.39215686274509803, b: 0.0, a: 1.0 }
    }

    /// COBALT_BLUE | #030AA7 | rgb(0.012, 0.039, 0.655)
    #[classattr]
    pub fn COBALT_BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.0392156862745098, b: 0.6549019607843137, a: 1.0 }
    }

    /// NEON_PINK | #FE019A | rgb(0.996, 0.004, 0.604)
    #[classattr]
    pub fn NEON_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.00392156862745098, b: 0.6039215686274509, a: 1.0 }
    }

    /// ROSE_PINK | #F7879A | rgb(0.969, 0.529, 0.604)
    #[classattr]
    pub fn ROSE_PINK() -> Color {
        Color { r: 0.9686274509803922, g: 0.5294117647058824, b: 0.6039215686274509, a: 1.0 }
    }

    /// GREYISH_PURPLE | #887191 | rgb(0.533, 0.443, 0.569)
    #[classattr]
    pub fn GREYISH_PURPLE() -> Color {
        Color { r: 0.5333333333333333, g: 0.44313725490196076, b: 0.5686274509803921, a: 1.0 }
    }

    /// RASPBERRY | #B00149 | rgb(0.690, 0.004, 0.286)
    #[classattr]
    pub fn RASPBERRY() -> Color {
        Color { r: 0.6901960784313725, g: 0.00392156862745098, b: 0.28627450980392155, a: 1.0 }
    }

    /// AQUA_GREEN | #12E193 | rgb(0.071, 0.882, 0.576)
    #[classattr]
    pub fn AQUA_GREEN() -> Color {
        Color { r: 0.07058823529411765, g: 0.8823529411764706, b: 0.5764705882352941, a: 1.0 }
    }

    /// SALMON_PINK | #FE7B7C | rgb(0.996, 0.482, 0.486)
    #[classattr]
    pub fn SALMON_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.4823529411764706, b: 0.48627450980392156, a: 1.0 }
    }

    /// TANGERINE | #FF9408 | rgb(1.000, 0.580, 0.031)
    #[classattr]
    pub fn TANGERINE() -> Color {
        Color { r: 1.0, g: 0.5803921568627451, b: 0.03137254901960784, a: 1.0 }
    }

    /// BROWNISH_GREEN | #6A6E09 | rgb(0.416, 0.431, 0.035)
    #[classattr]
    pub fn BROWNISH_GREEN() -> Color {
        Color { r: 0.41568627450980394, g: 0.43137254901960786, b: 0.03529411764705882, a: 1.0 }
    }

    /// RED_BROWN | #8B2E16 | rgb(0.545, 0.180, 0.086)
    #[classattr]
    pub fn RED_BROWN() -> Color {
        Color { r: 0.5450980392156862, g: 0.1803921568627451, b: 0.08627450980392157, a: 1.0 }
    }

    /// GREENISH_BROWN | #696112 | rgb(0.412, 0.380, 0.071)
    #[classattr]
    pub fn GREENISH_BROWN() -> Color {
        Color { r: 0.4117647058823529, g: 0.3803921568627451, b: 0.07058823529411765, a: 1.0 }
    }

    /// PUMPKIN | #E17701 | rgb(0.882, 0.467, 0.004)
    #[classattr]
    pub fn PUMPKIN() -> Color {
        Color { r: 0.8823529411764706, g: 0.4666666666666667, b: 0.00392156862745098, a: 1.0 }
    }

    /// PINE_GREEN | #0A481E | rgb(0.039, 0.282, 0.118)
    #[classattr]
    pub fn PINE_GREEN() -> Color {
        Color { r: 0.0392156862745098, g: 0.2823529411764706, b: 0.11764705882352941, a: 1.0 }
    }

    /// CHARCOAL | #343837 | rgb(0.204, 0.220, 0.216)
    #[classattr]
    pub fn CHARCOAL() -> Color {
        Color { r: 0.20392156862745098, g: 0.2196078431372549, b: 0.21568627450980393, a: 1.0 }
    }

    /// BABY_PINK | #FFB7CE | rgb(1.000, 0.718, 0.808)
    #[classattr]
    pub fn BABY_PINK() -> Color {
        Color { r: 1.0, g: 0.7176470588235294, b: 0.807843137254902, a: 1.0 }
    }

    /// CORNFLOWER | #6A79F7 | rgb(0.416, 0.475, 0.969)
    #[classattr]
    pub fn CORNFLOWER() -> Color {
        Color { r: 0.41568627450980394, g: 0.4745098039215686, b: 0.9686274509803922, a: 1.0 }
    }

    /// BLUE_VIOLET | #5D06E9 | rgb(0.365, 0.024, 0.914)
    #[classattr]
    pub fn BLUE_VIOLET() -> Color {
        Color { r: 0.36470588235294116, g: 0.023529411764705882, b: 0.9137254901960784, a: 1.0 }
    }

    /// CHOCOLATE | #3D1C02 | rgb(0.239, 0.110, 0.008)
    #[classattr]
    pub fn CHOCOLATE() -> Color {
        Color { r: 0.23921568627450981, g: 0.10980392156862745, b: 0.00784313725490196, a: 1.0 }
    }

    /// GREYISH_GREEN | #82A67D | rgb(0.510, 0.651, 0.490)
    #[classattr]
    pub fn GREYISH_GREEN() -> Color {
        Color { r: 0.5098039215686274, g: 0.6509803921568628, b: 0.49019607843137253, a: 1.0 }
    }

    /// SCARLET | #BE0119 | rgb(0.745, 0.004, 0.098)
    #[classattr]
    pub fn SCARLET() -> Color {
        Color { r: 0.7450980392156863, g: 0.00392156862745098, b: 0.09803921568627451, a: 1.0 }
    }

    /// DARK_OLIVE | #373E02 | rgb(0.216, 0.243, 0.008)
    #[classattr]
    pub fn DARK_OLIVE() -> Color {
        Color { r: 0.21568627450980393, g: 0.24313725490196078, b: 0.00784313725490196, a: 1.0 }
    }

    /// SIENNA | #A9561E | rgb(0.663, 0.337, 0.118)
    #[classattr]
    pub fn SIENNA() -> Color {
        Color { r: 0.6627450980392157, g: 0.33725490196078434, b: 0.11764705882352941, a: 1.0 }
    }

    /// PASTEL_PURPLE | #CAA0FF | rgb(0.792, 0.627, 1.000)
    #[classattr]
    pub fn PASTEL_PURPLE() -> Color {
        Color { r: 0.792156862745098, g: 0.6274509803921569, b: 1.0, a: 1.0 }
    }

    /// TERRACOTTA | #CA6641 | rgb(0.792, 0.400, 0.255)
    #[classattr]
    pub fn TERRACOTTA() -> Color {
        Color { r: 0.792156862745098, g: 0.4, b: 0.2549019607843137, a: 1.0 }
    }

    /// AQUA_BLUE | #02D8E9 | rgb(0.008, 0.847, 0.914)
    #[classattr]
    pub fn AQUA_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.8470588235294118, b: 0.9137254901960784, a: 1.0 }
    }

    /// SAGE_GREEN | #88B378 | rgb(0.533, 0.702, 0.471)
    #[classattr]
    pub fn SAGE_GREEN() -> Color {
        Color { r: 0.5333333333333333, g: 0.7019607843137254, b: 0.47058823529411764, a: 1.0 }
    }

    /// BLOOD_RED | #980002 | rgb(0.596, 0.000, 0.008)
    #[classattr]
    pub fn BLOOD_RED() -> Color {
        Color { r: 0.596078431372549, g: 0.0, b: 0.00784313725490196, a: 1.0 }
    }

    /// DEEP_PINK | #CB0162 | rgb(0.796, 0.004, 0.384)
    #[classattr]
    pub fn DEEP_PINK() -> Color {
        Color { r: 0.796078431372549, g: 0.00392156862745098, b: 0.3843137254901961, a: 1.0 }
    }

    /// GRASS | #5CAC2D | rgb(0.361, 0.675, 0.176)
    #[classattr]
    pub fn GRASS() -> Color {
        Color { r: 0.3607843137254902, g: 0.6745098039215687, b: 0.17647058823529413, a: 1.0 }
    }

    /// MOSS | #769958 | rgb(0.463, 0.600, 0.345)
    #[classattr]
    pub fn MOSS() -> Color {
        Color { r: 0.4627450980392157, g: 0.6, b: 0.34509803921568627, a: 1.0 }
    }

    /// PASTEL_BLUE | #A2BFFE | rgb(0.635, 0.749, 0.996)
    #[classattr]
    pub fn PASTEL_BLUE() -> Color {
        Color { r: 0.6352941176470588, g: 0.7490196078431373, b: 0.996078431372549, a: 1.0 }
    }

    /// BLUISH_GREEN | #10A674 | rgb(0.063, 0.651, 0.455)
    #[classattr]
    pub fn BLUISH_GREEN() -> Color {
        Color { r: 0.06274509803921569, g: 0.6509803921568628, b: 0.4549019607843137, a: 1.0 }
    }

    /// DARK_TAN | #AF884A | rgb(0.686, 0.533, 0.290)
    #[classattr]
    pub fn DARK_TAN() -> Color {
        Color { r: 0.6862745098039216, g: 0.5333333333333333, b: 0.2901960784313726, a: 1.0 }
    }

    /// GREENISH_BLUE | #0B8B87 | rgb(0.043, 0.545, 0.529)
    #[classattr]
    pub fn GREENISH_BLUE() -> Color {
        Color { r: 0.043137254901960784, g: 0.5450980392156862, b: 0.5294117647058824, a: 1.0 }
    }

    /// PALE_ORANGE | #FFA756 | rgb(1.000, 0.655, 0.337)
    #[classattr]
    pub fn PALE_ORANGE() -> Color {
        Color { r: 1.0, g: 0.6549019607843137, b: 0.33725490196078434, a: 1.0 }
    }

    /// VOMIT | #A2A415 | rgb(0.635, 0.643, 0.082)
    #[classattr]
    pub fn VOMIT() -> Color {
        Color { r: 0.6352941176470588, g: 0.6431372549019608, b: 0.08235294117647059, a: 1.0 }
    }

    /// FORREST_GREEN | #154406 | rgb(0.082, 0.267, 0.024)
    #[classattr]
    pub fn FORREST_GREEN() -> Color {
        Color { r: 0.08235294117647059, g: 0.26666666666666666, b: 0.023529411764705882, a: 1.0 }
    }

    /// DARK_LAVENDER | #856798 | rgb(0.522, 0.404, 0.596)
    #[classattr]
    pub fn DARK_LAVENDER() -> Color {
        Color { r: 0.5215686274509804, g: 0.403921568627451, b: 0.596078431372549, a: 1.0 }
    }

    /// DARK_VIOLET | #34013F | rgb(0.204, 0.004, 0.247)
    #[classattr]
    pub fn DARK_VIOLET() -> Color {
        Color { r: 0.20392156862745098, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 }
    }

    /// DARK_CYAN | #0A888A | rgb(0.039, 0.533, 0.541)
    #[classattr]
    pub fn DARK_CYAN() -> Color {
        Color { r: 0.0392156862745098, g: 0.5333333333333333, b: 0.5411764705882353, a: 1.0 }
    }

    /// OLIVE_DRAB | #6F7632 | rgb(0.435, 0.463, 0.196)
    #[classattr]
    pub fn OLIVE_DRAB() -> Color {
        Color { r: 0.43529411764705883, g: 0.4627450980392157, b: 0.19607843137254902, a: 1.0 }
    }

    /// PINKISH | #D46A7E | rgb(0.831, 0.416, 0.494)
    #[classattr]
    pub fn PINKISH() -> Color {
        Color { r: 0.8313725490196079, g: 0.41568627450980394, b: 0.49411764705882355, a: 1.0 }
    }

    /// COBALT | #1E488F | rgb(0.118, 0.282, 0.561)
    #[classattr]
    pub fn COBALT() -> Color {
        Color { r: 0.11764705882352941, g: 0.2823529411764706, b: 0.5607843137254902, a: 1.0 }
    }

    /// NEON_PURPLE | #BC13FE | rgb(0.737, 0.075, 0.996)
    #[classattr]
    pub fn NEON_PURPLE() -> Color {
        Color { r: 0.7372549019607844, g: 0.07450980392156863, b: 0.996078431372549, a: 1.0 }
    }

    /// LIGHT_TURQUOISE | #7EF4CC | rgb(0.494, 0.957, 0.800)
    #[classattr]
    pub fn LIGHT_TURQUOISE() -> Color {
        Color { r: 0.49411764705882355, g: 0.9568627450980393, b: 0.8, a: 1.0 }
    }

    /// APPLE_GREEN | #76CD26 | rgb(0.463, 0.804, 0.149)
    #[classattr]
    pub fn APPLE_GREEN() -> Color {
        Color { r: 0.4627450980392157, g: 0.803921568627451, b: 0.14901960784313725, a: 1.0 }
    }

    /// DULL_GREEN | #74A662 | rgb(0.455, 0.651, 0.384)
    #[classattr]
    pub fn DULL_GREEN() -> Color {
        Color { r: 0.4549019607843137, g: 0.6509803921568628, b: 0.3843137254901961, a: 1.0 }
    }

    /// WINE | #80013F | rgb(0.502, 0.004, 0.247)
    #[classattr]
    pub fn WINE() -> Color {
        Color { r: 0.5019607843137255, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 }
    }

    /// POWDER_BLUE | #B1D1FC | rgb(0.694, 0.820, 0.988)
    #[classattr]
    pub fn POWDER_BLUE() -> Color {
        Color { r: 0.6941176470588235, g: 0.8196078431372549, b: 0.9882352941176471, a: 1.0 }
    }

    /// OFF_WHITE | #FFFFE4 | rgb(1.000, 1.000, 0.894)
    #[classattr]
    pub fn OFF_WHITE() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.8941176470588236, a: 1.0 }
    }

    /// ELECTRIC_BLUE | #0652FF | rgb(0.024, 0.322, 1.000)
    #[classattr]
    pub fn ELECTRIC_BLUE() -> Color {
        Color { r: 0.023529411764705882, g: 0.3215686274509804, b: 1.0, a: 1.0 }
    }

    /// DARK_TURQUOISE | #045C5A | rgb(0.016, 0.361, 0.353)
    #[classattr]
    pub fn DARK_TURQUOISE() -> Color {
        Color { r: 0.01568627450980392, g: 0.3607843137254902, b: 0.35294117647058826, a: 1.0 }
    }

    /// AZURE | #069AF3 | rgb(0.024, 0.604, 0.953)
    #[classattr]
    pub fn AZURE() -> Color {
        Color { r: 0.023529411764705882, g: 0.6039215686274509, b: 0.9529411764705882, a: 1.0 }
    }

    /// BRIGHT_RED | #FF000D | rgb(1.000, 0.000, 0.051)
    #[classattr]
    pub fn BRIGHT_RED() -> Color {
        Color { r: 1.0, g: 0.0, b: 0.050980392156862744, a: 1.0 }
    }

    /// PINKISH_RED | #F10C45 | rgb(0.945, 0.047, 0.271)
    #[classattr]
    pub fn PINKISH_RED() -> Color {
        Color { r: 0.9450980392156862, g: 0.047058823529411764, b: 0.27058823529411763, a: 1.0 }
    }

    /// CORNFLOWER_BLUE | #5170D7 | rgb(0.318, 0.439, 0.843)
    #[classattr]
    pub fn CORNFLOWER_BLUE() -> Color {
        Color { r: 0.3176470588235294, g: 0.4392156862745098, b: 0.8431372549019608, a: 1.0 }
    }

    /// LIGHT_OLIVE | #ACBF69 | rgb(0.675, 0.749, 0.412)
    #[classattr]
    pub fn LIGHT_OLIVE() -> Color {
        Color { r: 0.6745098039215687, g: 0.7490196078431373, b: 0.4117647058823529, a: 1.0 }
    }

    /// GRAPE | #6C3461 | rgb(0.424, 0.204, 0.380)
    #[classattr]
    pub fn GRAPE() -> Color {
        Color { r: 0.4235294117647059, g: 0.20392156862745098, b: 0.3803921568627451, a: 1.0 }
    }

    /// GREYISH_BLUE | #5E819D | rgb(0.369, 0.506, 0.616)
    #[classattr]
    pub fn GREYISH_BLUE() -> Color {
        Color { r: 0.3686274509803922, g: 0.5058823529411764, b: 0.615686274509804, a: 1.0 }
    }

    /// PURPLISH_BLUE | #601EF9 | rgb(0.376, 0.118, 0.976)
    #[classattr]
    pub fn PURPLISH_BLUE() -> Color {
        Color { r: 0.3764705882352941, g: 0.11764705882352941, b: 0.9764705882352941, a: 1.0 }
    }

    /// YELLOWISH_GREEN | #B0DD16 | rgb(0.690, 0.867, 0.086)
    #[classattr]
    pub fn YELLOWISH_GREEN() -> Color {
        Color { r: 0.6901960784313725, g: 0.8666666666666667, b: 0.08627450980392157, a: 1.0 }
    }

    /// GREENISH_YELLOW | #CDFD02 | rgb(0.804, 0.992, 0.008)
    #[classattr]
    pub fn GREENISH_YELLOW() -> Color {
        Color { r: 0.803921568627451, g: 0.9921568627450981, b: 0.00784313725490196, a: 1.0 }
    }

    /// MEDIUM_BLUE | #2C6FBB | rgb(0.173, 0.435, 0.733)
    #[classattr]
    pub fn MEDIUM_BLUE() -> Color {
        Color { r: 0.17254901960784313, g: 0.43529411764705883, b: 0.7333333333333333, a: 1.0 }
    }

    /// DUSTY_ROSE | #C0737A | rgb(0.753, 0.451, 0.478)
    #[classattr]
    pub fn DUSTY_ROSE() -> Color {
        Color { r: 0.7529411764705882, g: 0.45098039215686275, b: 0.47843137254901963, a: 1.0 }
    }

    /// LIGHT_VIOLET | #D6B4FC | rgb(0.839, 0.706, 0.988)
    #[classattr]
    pub fn LIGHT_VIOLET() -> Color {
        Color { r: 0.8392156862745098, g: 0.7058823529411765, b: 0.9882352941176471, a: 1.0 }
    }

    /// MIDNIGHT_BLUE | #020035 | rgb(0.008, 0.000, 0.208)
    #[classattr]
    pub fn MIDNIGHT_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.0, b: 0.20784313725490197, a: 1.0 }
    }

    /// BLUISH_PURPLE | #703BE7 | rgb(0.439, 0.231, 0.906)
    #[classattr]
    pub fn BLUISH_PURPLE() -> Color {
        Color { r: 0.4392156862745098, g: 0.23137254901960785, b: 0.9058823529411765, a: 1.0 }
    }

    /// RED_ORANGE | #FD3C06 | rgb(0.992, 0.235, 0.024)
    #[classattr]
    pub fn RED_ORANGE() -> Color {
        Color { r: 0.9921568627450981, g: 0.23529411764705882, b: 0.023529411764705882, a: 1.0 }
    }

    /// DARK_MAGENTA | #960056 | rgb(0.588, 0.000, 0.337)
    #[classattr]
    pub fn DARK_MAGENTA() -> Color {
        Color { r: 0.5882352941176471, g: 0.0, b: 0.33725490196078434, a: 1.0 }
    }

    /// GREENISH | #40A368 | rgb(0.251, 0.639, 0.408)
    #[classattr]
    pub fn GREENISH() -> Color {
        Color { r: 0.25098039215686274, g: 0.6392156862745098, b: 0.40784313725490196, a: 1.0 }
    }

    /// OCEAN_BLUE | #03719C | rgb(0.012, 0.443, 0.612)
    #[classattr]
    pub fn OCEAN_BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.44313725490196076, b: 0.611764705882353, a: 1.0 }
    }

    /// CORAL | #FC5A50 | rgb(0.988, 0.353, 0.314)
    #[classattr]
    pub fn CORAL() -> Color {
        Color { r: 0.9882352941176471, g: 0.35294117647058826, b: 0.3137254901960784, a: 1.0 }
    }

    /// CREAM | #FFFFC2 | rgb(1.000, 1.000, 0.761)
    #[classattr]
    pub fn CREAM() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.7607843137254902, a: 1.0 }
    }

    /// REDDISH_BROWN | #7F2B0A | rgb(0.498, 0.169, 0.039)
    #[classattr]
    pub fn REDDISH_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.16862745098039217, b: 0.0392156862745098, a: 1.0 }
    }

    /// BURNT_SIENNA | #B04E0F | rgb(0.690, 0.306, 0.059)
    #[classattr]
    pub fn BURNT_SIENNA() -> Color {
        Color { r: 0.6901960784313725, g: 0.3058823529411765, b: 0.058823529411764705, a: 1.0 }
    }

    /// BRICK | #A03623 | rgb(0.627, 0.212, 0.137)
    #[classattr]
    pub fn BRICK() -> Color {
        Color { r: 0.6274509803921569, g: 0.21176470588235294, b: 0.13725490196078433, a: 1.0 }
    }

    /// SAGE | #87AE73 | rgb(0.529, 0.682, 0.451)
    #[classattr]
    pub fn SAGE() -> Color {
        Color { r: 0.5294117647058824, g: 0.6823529411764706, b: 0.45098039215686275, a: 1.0 }
    }

    /// WHITE | #FFFFFF | rgb(1.000, 1.000, 1.000)
    #[classattr]
    pub fn WHITE() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// ROBINS_EGG_BLUE | #98EFF9 | rgb(0.596, 0.937, 0.976)
    #[classattr]
    pub fn ROBINS_EGG_BLUE() -> Color {
        Color { r: 0.596078431372549, g: 0.9372549019607843, b: 0.9764705882352941, a: 1.0 }
    }

    /// MOSS_GREEN | #658B38 | rgb(0.396, 0.545, 0.220)
    #[classattr]
    pub fn MOSS_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.5450980392156862, b: 0.2196078431372549, a: 1.0 }
    }

    /// STEEL_BLUE | #5A7D9A | rgb(0.353, 0.490, 0.604)
    #[classattr]
    pub fn STEEL_BLUE() -> Color {
        Color { r: 0.35294117647058826, g: 0.49019607843137253, b: 0.6039215686274509, a: 1.0 }
    }

    /// EGGPLANT | #380835 | rgb(0.220, 0.031, 0.208)
    #[classattr]
    pub fn EGGPLANT() -> Color {
        Color { r: 0.2196078431372549, g: 0.03137254901960784, b: 0.20784313725490197, a: 1.0 }
    }

    /// LIGHT_YELLOW | #FFFE7A | rgb(1.000, 0.996, 0.478)
    #[classattr]
    pub fn LIGHT_YELLOW() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.47843137254901963, a: 1.0 }
    }

    /// LEAF_GREEN | #5CA904 | rgb(0.361, 0.663, 0.016)
    #[classattr]
    pub fn LEAF_GREEN() -> Color {
        Color { r: 0.3607843137254902, g: 0.6627450980392157, b: 0.01568627450980392, a: 1.0 }
    }

    /// LIGHT_GREY | #D8DCD6 | rgb(0.847, 0.863, 0.839)
    #[classattr]
    pub fn LIGHT_GREY() -> Color {
        Color { r: 0.8470588235294118, g: 0.8627450980392157, b: 0.8392156862745098, a: 1.0 }
    }

    /// PUKE | #A5A502 | rgb(0.647, 0.647, 0.008)
    #[classattr]
    pub fn PUKE() -> Color {
        Color { r: 0.6470588235294118, g: 0.6470588235294118, b: 0.00784313725490196, a: 1.0 }
    }

    /// PINKISH_PURPLE | #D648D7 | rgb(0.839, 0.282, 0.843)
    #[classattr]
    pub fn PINKISH_PURPLE() -> Color {
        Color { r: 0.8392156862745098, g: 0.2823529411764706, b: 0.8431372549019608, a: 1.0 }
    }

    /// SEA_BLUE | #047495 | rgb(0.016, 0.455, 0.584)
    #[classattr]
    pub fn SEA_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.4549019607843137, b: 0.5843137254901961, a: 1.0 }
    }

    /// PALE_PURPLE | #B790D4 | rgb(0.718, 0.565, 0.831)
    #[classattr]
    pub fn PALE_PURPLE() -> Color {
        Color { r: 0.7176470588235294, g: 0.5647058823529412, b: 0.8313725490196079, a: 1.0 }
    }

    /// SLATE_BLUE | #5B7C99 | rgb(0.357, 0.486, 0.600)
    #[classattr]
    pub fn SLATE_BLUE() -> Color {
        Color { r: 0.3568627450980392, g: 0.48627450980392156, b: 0.6, a: 1.0 }
    }

    /// HUNTER_GREEN | #0B4008 | rgb(0.043, 0.251, 0.031)
    #[classattr]
    pub fn HUNTER_GREEN() -> Color {
        Color { r: 0.043137254901960784, g: 0.25098039215686274, b: 0.03137254901960784, a: 1.0 }
    }

    /// FUCHSIA | #ED0DD9 | rgb(0.929, 0.051, 0.851)
    #[classattr]
    pub fn FUCHSIA() -> Color {
        Color { r: 0.9294117647058824, g: 0.050980392156862744, b: 0.8509803921568627, a: 1.0 }
    }

    /// CRIMSON | #8C000F | rgb(0.549, 0.000, 0.059)
    #[classattr]
    pub fn CRIMSON() -> Color {
        Color { r: 0.5490196078431373, g: 0.0, b: 0.058823529411764705, a: 1.0 }
    }

    /// PALE_YELLOW | #FFFF84 | rgb(1.000, 1.000, 0.518)
    #[classattr]
    pub fn PALE_YELLOW() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.5176470588235295, a: 1.0 }
    }

    /// OCHRE | #BF9005 | rgb(0.749, 0.565, 0.020)
    #[classattr]
    pub fn OCHRE() -> Color {
        Color { r: 0.7490196078431373, g: 0.5647058823529412, b: 0.0196078431372549, a: 1.0 }
    }

    /// MUSTARD_YELLOW | #D2BD0A | rgb(0.824, 0.741, 0.039)
    #[classattr]
    pub fn MUSTARD_YELLOW() -> Color {
        Color { r: 0.8235294117647058, g: 0.7411764705882353, b: 0.0392156862745098, a: 1.0 }
    }

    /// LIGHT_RED | #FF474C | rgb(1.000, 0.278, 0.298)
    #[classattr]
    pub fn LIGHT_RED() -> Color {
        Color { r: 1.0, g: 0.2784313725490196, b: 0.2980392156862745, a: 1.0 }
    }

    /// CERULEAN | #0485D1 | rgb(0.016, 0.522, 0.820)
    #[classattr]
    pub fn CERULEAN() -> Color {
        Color { r: 0.01568627450980392, g: 0.5215686274509804, b: 0.8196078431372549, a: 1.0 }
    }

    /// PALE_PINK | #FFCFDC | rgb(1.000, 0.812, 0.863)
    #[classattr]
    pub fn PALE_PINK() -> Color {
        Color { r: 1.0, g: 0.8117647058823529, b: 0.8627450980392157, a: 1.0 }
    }

    /// DEEP_BLUE | #040273 | rgb(0.016, 0.008, 0.451)
    #[classattr]
    pub fn DEEP_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.00784313725490196, b: 0.45098039215686275, a: 1.0 }
    }

    /// RUST | #A83C09 | rgb(0.659, 0.235, 0.035)
    #[classattr]
    pub fn RUST() -> Color {
        Color { r: 0.6588235294117647, g: 0.23529411764705882, b: 0.03529411764705882, a: 1.0 }
    }

    /// LIGHT_TEAL | #90E4C1 | rgb(0.565, 0.894, 0.757)
    #[classattr]
    pub fn LIGHT_TEAL() -> Color {
        Color { r: 0.5647058823529412, g: 0.8941176470588236, b: 0.7568627450980392, a: 1.0 }
    }

    /// SLATE | #516572 | rgb(0.318, 0.396, 0.447)
    #[classattr]
    pub fn SLATE() -> Color {
        Color { r: 0.3176470588235294, g: 0.396078431372549, b: 0.4470588235294118, a: 1.0 }
    }

    /// GOLDENROD | #FAC205 | rgb(0.980, 0.761, 0.020)
    #[classattr]
    pub fn GOLDENROD() -> Color {
        Color { r: 0.9803921568627451, g: 0.7607843137254902, b: 0.0196078431372549, a: 1.0 }
    }

    /// DARK_YELLOW | #D5B60A | rgb(0.835, 0.714, 0.039)
    #[classattr]
    pub fn DARK_YELLOW() -> Color {
        Color { r: 0.8352941176470589, g: 0.7137254901960784, b: 0.0392156862745098, a: 1.0 }
    }

    /// DARK_GREY | #363737 | rgb(0.212, 0.216, 0.216)
    #[classattr]
    pub fn DARK_GREY() -> Color {
        Color { r: 0.21176470588235294, g: 0.21568627450980393, b: 0.21568627450980393, a: 1.0 }
    }

    /// ARMY_GREEN | #4B5D16 | rgb(0.294, 0.365, 0.086)
    #[classattr]
    pub fn ARMY_GREEN() -> Color {
        Color { r: 0.29411764705882354, g: 0.36470588235294116, b: 0.08627450980392157, a: 1.0 }
    }

    /// SEAFOAM | #80F9AD | rgb(0.502, 0.976, 0.678)
    #[classattr]
    pub fn SEAFOAM() -> Color {
        Color { r: 0.5019607843137255, g: 0.9764705882352941, b: 0.6784313725490196, a: 1.0 }
    }

    /// PUCE | #A57E52 | rgb(0.647, 0.494, 0.322)
    #[classattr]
    pub fn PUCE() -> Color {
        Color { r: 0.6470588235294118, g: 0.49411764705882355, b: 0.3215686274509804, a: 1.0 }
    }

    /// SPRING_GREEN | #A9F971 | rgb(0.663, 0.976, 0.443)
    #[classattr]
    pub fn SPRING_GREEN() -> Color {
        Color { r: 0.6627450980392157, g: 0.9764705882352941, b: 0.44313725490196076, a: 1.0 }
    }

    /// DARK_ORANGE | #C65102 | rgb(0.776, 0.318, 0.008)
    #[classattr]
    pub fn DARK_ORANGE() -> Color {
        Color { r: 0.7764705882352941, g: 0.3176470588235294, b: 0.00784313725490196, a: 1.0 }
    }

    /// SAND | #E2CA76 | rgb(0.886, 0.792, 0.463)
    #[classattr]
    pub fn SAND() -> Color {
        Color { r: 0.8862745098039215, g: 0.792156862745098, b: 0.4627450980392157, a: 1.0 }
    }

    /// PASTEL_GREEN | #B0FF9D | rgb(0.690, 1.000, 0.616)
    #[classattr]
    pub fn PASTEL_GREEN() -> Color {
        Color { r: 0.6901960784313725, g: 1.0, b: 0.615686274509804, a: 1.0 }
    }

    /// MINT | #9FFEB0 | rgb(0.624, 0.996, 0.690)
    #[classattr]
    pub fn MINT() -> Color {
        Color { r: 0.6235294117647059, g: 0.996078431372549, b: 0.6901960784313725, a: 1.0 }
    }

    /// LIGHT_ORANGE | #FDAA48 | rgb(0.992, 0.667, 0.282)
    #[classattr]
    pub fn LIGHT_ORANGE() -> Color {
        Color { r: 0.9921568627450981, g: 0.6666666666666666, b: 0.2823529411764706, a: 1.0 }
    }

    /// BRIGHT_PINK | #FE01B1 | rgb(0.996, 0.004, 0.694)
    #[classattr]
    pub fn BRIGHT_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.00392156862745098, b: 0.6941176470588235, a: 1.0 }
    }

    /// CHARTREUSE | #C1F80A | rgb(0.757, 0.973, 0.039)
    #[classattr]
    pub fn CHARTREUSE() -> Color {
        Color { r: 0.7568627450980392, g: 0.9725490196078431, b: 0.0392156862745098, a: 1.0 }
    }

    /// DEEP_PURPLE | #36013F | rgb(0.212, 0.004, 0.247)
    #[classattr]
    pub fn DEEP_PURPLE() -> Color {
        Color { r: 0.21176470588235294, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 }
    }

    /// DARK_BROWN | #341C02 | rgb(0.204, 0.110, 0.008)
    #[classattr]
    pub fn DARK_BROWN() -> Color {
        Color { r: 0.20392156862745098, g: 0.10980392156862745, b: 0.00784313725490196, a: 1.0 }
    }

    /// TAUPE | #B9A281 | rgb(0.725, 0.635, 0.506)
    #[classattr]
    pub fn TAUPE() -> Color {
        Color { r: 0.7254901960784313, g: 0.6352941176470588, b: 0.5058823529411764, a: 1.0 }
    }

    /// PEA_GREEN | #8EAB12 | rgb(0.557, 0.671, 0.071)
    #[classattr]
    pub fn PEA_GREEN() -> Color {
        Color { r: 0.5568627450980392, g: 0.6705882352941176, b: 0.07058823529411765, a: 1.0 }
    }

    /// PUKE_GREEN | #9AAE07 | rgb(0.604, 0.682, 0.027)
    #[classattr]
    pub fn PUKE_GREEN() -> Color {
        Color { r: 0.6039215686274509, g: 0.6823529411764706, b: 0.027450980392156862, a: 1.0 }
    }

    /// KELLY_GREEN | #02AB2E | rgb(0.008, 0.671, 0.180)
    #[classattr]
    pub fn KELLY_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.6705882352941176, b: 0.1803921568627451, a: 1.0 }
    }

    /// SEAFOAM_GREEN | #7AF9AB | rgb(0.478, 0.976, 0.671)
    #[classattr]
    pub fn SEAFOAM_GREEN() -> Color {
        Color { r: 0.47843137254901963, g: 0.9764705882352941, b: 0.6705882352941176, a: 1.0 }
    }

    /// KHAKI | #AAA662 | rgb(0.667, 0.651, 0.384)
    #[classattr]
    pub fn KHAKI() -> Color {
        Color { r: 0.6666666666666666, g: 0.6509803921568628, b: 0.3843137254901961, a: 1.0 }
    }

    /// BURGUNDY | #610023 | rgb(0.380, 0.000, 0.137)
    #[classattr]
    pub fn BURGUNDY() -> Color {
        Color { r: 0.3803921568627451, g: 0.0, b: 0.13725490196078433, a: 1.0 }
    }

    /// DARK_TEAL | #014D4E | rgb(0.004, 0.302, 0.306)
    #[classattr]
    pub fn DARK_TEAL() -> Color {
        Color { r: 0.00392156862745098, g: 0.30196078431372547, b: 0.3058823529411765, a: 1.0 }
    }

    /// BRICK_RED | #8F1402 | rgb(0.561, 0.078, 0.008)
    #[classattr]
    pub fn BRICK_RED() -> Color {
        Color { r: 0.5607843137254902, g: 0.0784313725490196, b: 0.00784313725490196, a: 1.0 }
    }

    /// ROYAL_PURPLE | #4B006E | rgb(0.294, 0.000, 0.431)
    #[classattr]
    pub fn ROYAL_PURPLE() -> Color {
        Color { r: 0.29411764705882354, g: 0.0, b: 0.43137254901960786, a: 1.0 }
    }

    /// PLUM | #580F41 | rgb(0.345, 0.059, 0.255)
    #[classattr]
    pub fn PLUM() -> Color {
        Color { r: 0.34509803921568627, g: 0.058823529411764705, b: 0.2549019607843137, a: 1.0 }
    }

    /// MINT_GREEN | #8FFF9F | rgb(0.561, 1.000, 0.624)
    #[classattr]
    pub fn MINT_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 1.0, b: 0.6235294117647059, a: 1.0 }
    }

    /// GOLD | #DBB40C | rgb(0.859, 0.706, 0.047)
    #[classattr]
    pub fn GOLD() -> Color {
        Color { r: 0.8588235294117647, g: 0.7058823529411765, b: 0.047058823529411764, a: 1.0 }
    }

    /// BABY_BLUE | #A2CFFE | rgb(0.635, 0.812, 0.996)
    #[classattr]
    pub fn BABY_BLUE() -> Color {
        Color { r: 0.6352941176470588, g: 0.8117647058823529, b: 0.996078431372549, a: 1.0 }
    }

    /// BRIGHT_PURPLE | #BE03FD | rgb(0.745, 0.012, 0.992)
    #[classattr]
    pub fn BRIGHT_PURPLE() -> Color {
        Color { r: 0.7450980392156863, g: 0.011764705882352941, b: 0.9921568627450981, a: 1.0 }
    }

    /// DARK_RED | #840000 | rgb(0.518, 0.000, 0.000)
    #[classattr]
    pub fn DARK_RED() -> Color {
        Color { r: 0.5176470588235295, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// PALE_BLUE | #D0FEFE | rgb(0.816, 0.996, 0.996)
    #[classattr]
    pub fn PALE_BLUE() -> Color {
        Color { r: 0.8156862745098039, g: 0.996078431372549, b: 0.996078431372549, a: 1.0 }
    }

    /// GRASS_GREEN | #3F9B0B | rgb(0.247, 0.608, 0.043)
    #[classattr]
    pub fn GRASS_GREEN() -> Color {
        Color { r: 0.24705882352941178, g: 0.6078431372549019, b: 0.043137254901960784, a: 1.0 }
    }

    /// NAVY | #01153E | rgb(0.004, 0.082, 0.243)
    #[classattr]
    pub fn NAVY() -> Color {
        Color { r: 0.00392156862745098, g: 0.08235294117647059, b: 0.24313725490196078, a: 1.0 }
    }

    /// AQUAMARINE | #04D8B2 | rgb(0.016, 0.847, 0.698)
    #[classattr]
    pub fn AQUAMARINE() -> Color {
        Color { r: 0.01568627450980392, g: 0.8470588235294118, b: 0.6980392156862745, a: 1.0 }
    }

    /// BURNT_ORANGE | #C04E01 | rgb(0.753, 0.306, 0.004)
    #[classattr]
    pub fn BURNT_ORANGE() -> Color {
        Color { r: 0.7529411764705882, g: 0.3058823529411765, b: 0.00392156862745098, a: 1.0 }
    }

    /// NEON_GREEN | #0CFF0C | rgb(0.047, 1.000, 0.047)
    #[classattr]
    pub fn NEON_GREEN() -> Color {
        Color { r: 0.047058823529411764, g: 1.0, b: 0.047058823529411764, a: 1.0 }
    }

    /// BRIGHT_BLUE | #0165FC | rgb(0.004, 0.396, 0.988)
    #[classattr]
    pub fn BRIGHT_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.396078431372549, b: 0.9882352941176471, a: 1.0 }
    }

    /// ROSE | #CF6275 | rgb(0.812, 0.384, 0.459)
    #[classattr]
    pub fn ROSE() -> Color {
        Color { r: 0.8117647058823529, g: 0.3843137254901961, b: 0.4588235294117647, a: 1.0 }
    }

    /// LIGHT_PINK | #FFD1DF | rgb(1.000, 0.820, 0.875)
    #[classattr]
    pub fn LIGHT_PINK() -> Color {
        Color { r: 1.0, g: 0.8196078431372549, b: 0.8745098039215686, a: 1.0 }
    }

    /// MUSTARD | #CEB301 | rgb(0.808, 0.702, 0.004)
    #[classattr]
    pub fn MUSTARD() -> Color {
        Color { r: 0.807843137254902, g: 0.7019607843137254, b: 0.00392156862745098, a: 1.0 }
    }

    /// INDIGO | #380282 | rgb(0.220, 0.008, 0.510)
    #[classattr]
    pub fn INDIGO() -> Color {
        Color { r: 0.2196078431372549, g: 0.00784313725490196, b: 0.5098039215686274, a: 1.0 }
    }

    /// LIME | #AAFF32 | rgb(0.667, 1.000, 0.196)
    #[classattr]
    pub fn LIME() -> Color {
        Color { r: 0.6666666666666666, g: 1.0, b: 0.19607843137254902, a: 1.0 }
    }

    /// SEA_GREEN | #53FCA1 | rgb(0.325, 0.988, 0.631)
    #[classattr]
    pub fn SEA_GREEN() -> Color {
        Color { r: 0.3254901960784314, g: 0.9882352941176471, b: 0.6313725490196078, a: 1.0 }
    }

    /// PERIWINKLE | #8E82FE | rgb(0.557, 0.510, 0.996)
    #[classattr]
    pub fn PERIWINKLE() -> Color {
        Color { r: 0.5568627450980392, g: 0.5098039215686274, b: 0.996078431372549, a: 1.0 }
    }

    /// DARK_PINK | #CB416B | rgb(0.796, 0.255, 0.420)
    #[classattr]
    pub fn DARK_PINK() -> Color {
        Color { r: 0.796078431372549, g: 0.2549019607843137, b: 0.4196078431372549, a: 1.0 }
    }

    /// OLIVE_GREEN | #677A04 | rgb(0.404, 0.478, 0.016)
    #[classattr]
    pub fn OLIVE_GREEN() -> Color {
        Color { r: 0.403921568627451, g: 0.47843137254901963, b: 0.01568627450980392, a: 1.0 }
    }

    /// PEACH | #FFB07C | rgb(1.000, 0.690, 0.486)
    #[classattr]
    pub fn PEACH() -> Color {
        Color { r: 1.0, g: 0.6901960784313725, b: 0.48627450980392156, a: 1.0 }
    }

    /// PALE_GREEN | #C7FDB5 | rgb(0.780, 0.992, 0.710)
    #[classattr]
    pub fn PALE_GREEN() -> Color {
        Color { r: 0.7803921568627451, g: 0.9921568627450981, b: 0.7098039215686275, a: 1.0 }
    }

    /// LIGHT_BROWN | #AD8150 | rgb(0.678, 0.506, 0.314)
    #[classattr]
    pub fn LIGHT_BROWN() -> Color {
        Color { r: 0.6784313725490196, g: 0.5058823529411764, b: 0.3137254901960784, a: 1.0 }
    }

    /// HOT_PINK | #FF028D | rgb(1.000, 0.008, 0.553)
    #[classattr]
    pub fn HOT_PINK() -> Color {
        Color { r: 1.0, g: 0.00784313725490196, b: 0.5529411764705883, a: 1.0 }
    }

    /// BLACK | #000000 | rgb(0.000, 0.000, 0.000)
    #[classattr]
    pub fn BLACK() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// LILAC | #CEA2FD | rgb(0.808, 0.635, 0.992)
    #[classattr]
    pub fn LILAC() -> Color {
        Color { r: 0.807843137254902, g: 0.6352941176470588, b: 0.9921568627450981, a: 1.0 }
    }

    /// NAVY_BLUE | #001146 | rgb(0.000, 0.067, 0.275)
    #[classattr]
    pub fn NAVY_BLUE() -> Color {
        Color { r: 0.0, g: 0.06666666666666667, b: 0.27450980392156865, a: 1.0 }
    }

    /// ROYAL_BLUE | #0504AA | rgb(0.020, 0.016, 0.667)
    #[classattr]
    pub fn ROYAL_BLUE() -> Color {
        Color { r: 0.0196078431372549, g: 0.01568627450980392, b: 0.6666666666666666, a: 1.0 }
    }

    /// BEIGE | #E6DAA6 | rgb(0.902, 0.855, 0.651)
    #[classattr]
    pub fn BEIGE() -> Color {
        Color { r: 0.9019607843137255, g: 0.8549019607843137, b: 0.6509803921568628, a: 1.0 }
    }

    /// SALMON | #FF796C | rgb(1.000, 0.475, 0.424)
    #[classattr]
    pub fn SALMON() -> Color {
        Color { r: 1.0, g: 0.4745098039215686, b: 0.4235294117647059, a: 1.0 }
    }

    /// OLIVE | #6E750E | rgb(0.431, 0.459, 0.055)
    #[classattr]
    pub fn OLIVE() -> Color {
        Color { r: 0.43137254901960786, g: 0.4588235294117647, b: 0.054901960784313725, a: 1.0 }
    }

    /// MAROON | #650021 | rgb(0.396, 0.000, 0.129)
    #[classattr]
    pub fn MAROON() -> Color {
        Color { r: 0.396078431372549, g: 0.0, b: 0.12941176470588237, a: 1.0 }
    }

    /// BRIGHT_GREEN | #01FF07 | rgb(0.004, 1.000, 0.027)
    #[classattr]
    pub fn BRIGHT_GREEN() -> Color {
        Color { r: 0.00392156862745098, g: 1.0, b: 0.027450980392156862, a: 1.0 }
    }

    /// DARK_PURPLE | #35063E | rgb(0.208, 0.024, 0.243)
    #[classattr]
    pub fn DARK_PURPLE() -> Color {
        Color { r: 0.20784313725490197, g: 0.023529411764705882, b: 0.24313725490196078, a: 1.0 }
    }

    /// MAUVE | #AE7181 | rgb(0.682, 0.443, 0.506)
    #[classattr]
    pub fn MAUVE() -> Color {
        Color { r: 0.6823529411764706, g: 0.44313725490196076, b: 0.5058823529411764, a: 1.0 }
    }

    /// FOREST_GREEN | #06470C | rgb(0.024, 0.278, 0.047)
    #[classattr]
    pub fn FOREST_GREEN() -> Color {
        Color { r: 0.023529411764705882, g: 0.2784313725490196, b: 0.047058823529411764, a: 1.0 }
    }

    /// AQUA | #13EAC9 | rgb(0.075, 0.918, 0.788)
    #[classattr]
    pub fn AQUA() -> Color {
        Color { r: 0.07450980392156863, g: 0.9176470588235294, b: 0.788235294117647, a: 1.0 }
    }

    /// CYAN | #00FFFF | rgb(0.000, 1.000, 1.000)
    #[classattr]
    pub fn CYAN() -> Color {
        Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// TAN | #D1B26F | rgb(0.820, 0.698, 0.435)
    #[classattr]
    pub fn TAN() -> Color {
        Color { r: 0.8196078431372549, g: 0.6980392156862745, b: 0.43529411764705883, a: 1.0 }
    }

    /// DARK_BLUE | #00035B | rgb(0.000, 0.012, 0.357)
    #[classattr]
    pub fn DARK_BLUE() -> Color {
        Color { r: 0.0, g: 0.011764705882352941, b: 0.3568627450980392, a: 1.0 }
    }

    /// LAVENDER | #C79FEF | rgb(0.780, 0.624, 0.937)
    #[classattr]
    pub fn LAVENDER() -> Color {
        Color { r: 0.7803921568627451, g: 0.6235294117647059, b: 0.9372549019607843, a: 1.0 }
    }

    /// TURQUOISE | #06C2AC | rgb(0.024, 0.761, 0.675)
    #[classattr]
    pub fn TURQUOISE() -> Color {
        Color { r: 0.023529411764705882, g: 0.7607843137254902, b: 0.6745098039215687, a: 1.0 }
    }

    /// DARK_GREEN | #033500 | rgb(0.012, 0.208, 0.000)
    #[classattr]
    pub fn DARK_GREEN() -> Color {
        Color { r: 0.011764705882352941, g: 0.20784313725490197, b: 0.0, a: 1.0 }
    }

    /// VIOLET | #9A0EEA | rgb(0.604, 0.055, 0.918)
    #[classattr]
    pub fn VIOLET() -> Color {
        Color { r: 0.6039215686274509, g: 0.054901960784313725, b: 0.9176470588235294, a: 1.0 }
    }

    /// LIGHT_PURPLE | #BF77F6 | rgb(0.749, 0.467, 0.965)
    #[classattr]
    pub fn LIGHT_PURPLE() -> Color {
        Color { r: 0.7490196078431373, g: 0.4666666666666667, b: 0.9647058823529412, a: 1.0 }
    }

    /// LIME_GREEN | #89FE05 | rgb(0.537, 0.996, 0.020)
    #[classattr]
    pub fn LIME_GREEN() -> Color {
        Color { r: 0.5372549019607843, g: 0.996078431372549, b: 0.0196078431372549, a: 1.0 }
    }

    /// GREY | #929591 | rgb(0.573, 0.584, 0.569)
    #[classattr]
    pub fn GREY() -> Color {
        Color { r: 0.5725490196078431, g: 0.5843137254901961, b: 0.5686274509803921, a: 1.0 }
    }

    /// SKY_BLUE | #75BBFD | rgb(0.459, 0.733, 0.992)
    #[classattr]
    pub fn SKY_BLUE() -> Color {
        Color { r: 0.4588235294117647, g: 0.7333333333333333, b: 0.9921568627450981, a: 1.0 }
    }

    /// YELLOW | #FFFF14 | rgb(1.000, 1.000, 0.078)
    #[classattr]
    pub fn YELLOW() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.0784313725490196, a: 1.0 }
    }

    /// MAGENTA | #C20078 | rgb(0.761, 0.000, 0.471)
    #[classattr]
    pub fn MAGENTA() -> Color {
        Color { r: 0.7607843137254902, g: 0.0, b: 0.47058823529411764, a: 1.0 }
    }

    /// LIGHT_GREEN | #96F97B | rgb(0.588, 0.976, 0.482)
    #[classattr]
    pub fn LIGHT_GREEN() -> Color {
        Color { r: 0.5882352941176471, g: 0.9764705882352941, b: 0.4823529411764706, a: 1.0 }
    }

    /// ORANGE | #F97306 | rgb(0.976, 0.451, 0.024)
    #[classattr]
    pub fn ORANGE() -> Color {
        Color { r: 0.9764705882352941, g: 0.45098039215686275, b: 0.023529411764705882, a: 1.0 }
    }

    /// TEAL | #029386 | rgb(0.008, 0.576, 0.525)
    #[classattr]
    pub fn TEAL() -> Color {
        Color { r: 0.00784313725490196, g: 0.5764705882352941, b: 0.5254901960784314, a: 1.0 }
    }

    /// LIGHT_BLUE | #95D0FC | rgb(0.584, 0.816, 0.988)
    #[classattr]
    pub fn LIGHT_BLUE() -> Color {
        Color { r: 0.5843137254901961, g: 0.8156862745098039, b: 0.9882352941176471, a: 1.0 }
    }

    /// RED | #E50000 | rgb(0.898, 0.000, 0.000)
    #[classattr]
    pub fn RED() -> Color {
        Color { r: 0.8980392156862745, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// BROWN | #653700 | rgb(0.396, 0.216, 0.000)
    #[classattr]
    pub fn BROWN() -> Color {
        Color { r: 0.396078431372549, g: 0.21568627450980393, b: 0.0, a: 1.0 }
    }

    /// PINK | #FF81C0 | rgb(1.000, 0.506, 0.753)
    #[classattr]
    pub fn PINK() -> Color {
        Color { r: 1.0, g: 0.5058823529411764, b: 0.7529411764705882, a: 1.0 }
    }

    /// BLUE | #0343DF | rgb(0.012, 0.263, 0.875)
    #[classattr]
    pub fn BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.2627450980392157, b: 0.8745098039215686, a: 1.0 }
    }

    /// GREEN | #15B01A | rgb(0.082, 0.690, 0.102)
    #[classattr]
    pub fn GREEN() -> Color {
        Color { r: 0.08235294117647059, g: 0.6901960784313725, b: 0.10196078431372549, a: 1.0 }
    }

    /// PURPLE | #7E1E9C | rgb(0.494, 0.118, 0.612)
    #[classattr]
    pub fn PURPLE() -> Color {
        Color { r: 0.49411764705882355, g: 0.11764705882352941, b: 0.611764705882353, a: 1.0 }
    }

    /// GRAY_TEAL | #5E9B8A | rgb(0.369, 0.608, 0.541)
    #[classattr]
    pub fn GRAY_TEAL() -> Color {
        Color { r: 0.3686274509803922, g: 0.6078431372549019, b: 0.5411764705882353, a: 1.0 }
    }

    /// PURPLEY_GRAY | #947E94 | rgb(0.580, 0.494, 0.580)
    #[classattr]
    pub fn PURPLEY_GRAY() -> Color {
        Color { r: 0.5803921568627451, g: 0.49411764705882355, b: 0.5803921568627451, a: 1.0 }
    }

    /// LIGHT_GRAY_GREEN | #B7E1A1 | rgb(0.718, 0.882, 0.631)
    #[classattr]
    pub fn LIGHT_GRAY_GREEN() -> Color {
        Color { r: 0.7176470588235294, g: 0.8823529411764706, b: 0.6313725490196078, a: 1.0 }
    }

    /// REDDISH_GRAY | #997570 | rgb(0.600, 0.459, 0.439)
    #[classattr]
    pub fn REDDISH_GRAY() -> Color {
        Color { r: 0.6, g: 0.4588235294117647, b: 0.4392156862745098, a: 1.0 }
    }

    /// BATTLESHIP_GRAY | #6B7C85 | rgb(0.420, 0.486, 0.522)
    #[classattr]
    pub fn BATTLESHIP_GRAY() -> Color {
        Color { r: 0.4196078431372549, g: 0.48627450980392156, b: 0.5215686274509804, a: 1.0 }
    }

    /// CHARCOAL_GRAY | #3C4142 | rgb(0.235, 0.255, 0.259)
    #[classattr]
    pub fn CHARCOAL_GRAY() -> Color {
        Color { r: 0.23529411764705882, g: 0.2549019607843137, b: 0.25882352941176473, a: 1.0 }
    }

    /// GRAYISH_TEAL | #719F91 | rgb(0.443, 0.624, 0.569)
    #[classattr]
    pub fn GRAYISH_TEAL() -> Color {
        Color { r: 0.44313725490196076, g: 0.6235294117647059, b: 0.5686274509803921, a: 1.0 }
    }

    /// GRAY_GREEN | #86A17D | rgb(0.525, 0.631, 0.490)
    #[classattr]
    pub fn GRAY_GREEN() -> Color {
        Color { r: 0.5254901960784314, g: 0.6313725490196078, b: 0.49019607843137253, a: 1.0 }
    }

    /// COOL_GRAY | #95A3A6 | rgb(0.584, 0.639, 0.651)
    #[classattr]
    pub fn COOL_GRAY() -> Color {
        Color { r: 0.5843137254901961, g: 0.6392156862745098, b: 0.6509803921568628, a: 1.0 }
    }

    /// DARK_BLUE_GRAY | #1F3B4D | rgb(0.122, 0.231, 0.302)
    #[classattr]
    pub fn DARK_BLUE_GRAY() -> Color {
        Color { r: 0.12156862745098039, g: 0.23137254901960785, b: 0.30196078431372547, a: 1.0 }
    }

    /// BLUEY_GRAY | #89A0B0 | rgb(0.537, 0.627, 0.690)
    #[classattr]
    pub fn BLUEY_GRAY() -> Color {
        Color { r: 0.5372549019607843, g: 0.6274509803921569, b: 0.6901960784313725, a: 1.0 }
    }

    /// GREENY_GRAY | #7EA07A | rgb(0.494, 0.627, 0.478)
    #[classattr]
    pub fn GREENY_GRAY() -> Color {
        Color { r: 0.49411764705882355, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 }
    }

    /// BLUEGRAY | #85A3B2 | rgb(0.522, 0.639, 0.698)
    #[classattr]
    pub fn BLUEGRAY() -> Color {
        Color { r: 0.5215686274509804, g: 0.6392156862745098, b: 0.6980392156862745, a: 1.0 }
    }

    /// LIGHT_BLUE_GRAY | #B7C9E2 | rgb(0.718, 0.788, 0.886)
    #[classattr]
    pub fn LIGHT_BLUE_GRAY() -> Color {
        Color { r: 0.7176470588235294, g: 0.788235294117647, b: 0.8862745098039215, a: 1.0 }
    }

    /// GRAY_BLUE | #647D8E | rgb(0.392, 0.490, 0.557)
    #[classattr]
    pub fn GRAY_BLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.49019607843137253, b: 0.5568627450980392, a: 1.0 }
    }

    /// BROWN_GRAY | #8D8468 | rgb(0.553, 0.518, 0.408)
    #[classattr]
    pub fn BROWN_GRAY() -> Color {
        Color { r: 0.5529411764705883, g: 0.5176470588235295, b: 0.40784313725490196, a: 1.0 }
    }

    /// BLUE_GRAY | #758DA3 | rgb(0.459, 0.553, 0.639)
    #[classattr]
    pub fn BLUE_GRAY() -> Color {
        Color { r: 0.4588235294117647, g: 0.5529411764705883, b: 0.6392156862745098, a: 1.0 }
    }

    /// GRAYBLUE | #77A1B5 | rgb(0.467, 0.631, 0.710)
    #[classattr]
    pub fn GRAYBLUE() -> Color {
        Color { r: 0.4666666666666667, g: 0.6313725490196078, b: 0.7098039215686275, a: 1.0 }
    }

    /// DARK_GRAY_BLUE | #29465B | rgb(0.161, 0.275, 0.357)
    #[classattr]
    pub fn DARK_GRAY_BLUE() -> Color {
        Color { r: 0.1607843137254902, g: 0.27450980392156865, b: 0.3568627450980392, a: 1.0 }
    }

    /// GRAYISH | #A8A495 | rgb(0.659, 0.643, 0.584)
    #[classattr]
    pub fn GRAYISH() -> Color {
        Color { r: 0.6588235294117647, g: 0.6431372549019608, b: 0.5843137254901961, a: 1.0 }
    }

    /// LIGHT_GRAY_BLUE | #9DBCD4 | rgb(0.616, 0.737, 0.831)
    #[classattr]
    pub fn LIGHT_GRAY_BLUE() -> Color {
        Color { r: 0.615686274509804, g: 0.7372549019607844, b: 0.8313725490196079, a: 1.0 }
    }

    /// PALE_GRAY | #FDFDFE | rgb(0.992, 0.992, 0.996)
    #[classattr]
    pub fn PALE_GRAY() -> Color {
        Color { r: 0.9921568627450981, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 }
    }

    /// WARM_GRAY | #978A84 | rgb(0.592, 0.541, 0.518)
    #[classattr]
    pub fn WARM_GRAY() -> Color {
        Color { r: 0.592156862745098, g: 0.5411764705882353, b: 0.5176470588235295, a: 1.0 }
    }

    /// GRAY_PINK | #C3909B | rgb(0.765, 0.565, 0.608)
    #[classattr]
    pub fn GRAY_PINK() -> Color {
        Color { r: 0.7647058823529411, g: 0.5647058823529412, b: 0.6078431372549019, a: 1.0 }
    }

    /// MEDIUM_GRAY | #7D7F7C | rgb(0.490, 0.498, 0.486)
    #[classattr]
    pub fn MEDIUM_GRAY() -> Color {
        Color { r: 0.49019607843137253, g: 0.4980392156862745, b: 0.48627450980392156, a: 1.0 }
    }

    /// PINKISH_GRAY | #C8ACA9 | rgb(0.784, 0.675, 0.663)
    #[classattr]
    pub fn PINKISH_GRAY() -> Color {
        Color { r: 0.7843137254901961, g: 0.6745098039215687, b: 0.6627450980392157, a: 1.0 }
    }

    /// BROWNISH_GRAY | #86775F | rgb(0.525, 0.467, 0.373)
    #[classattr]
    pub fn BROWNISH_GRAY() -> Color {
        Color { r: 0.5254901960784314, g: 0.4666666666666667, b: 0.37254901960784315, a: 1.0 }
    }

    /// PURPLISH_GRAY | #7A687F | rgb(0.478, 0.408, 0.498)
    #[classattr]
    pub fn PURPLISH_GRAY() -> Color {
        Color { r: 0.47843137254901963, g: 0.40784313725490196, b: 0.4980392156862745, a: 1.0 }
    }

    /// GRAYISH_PINK | #C88D94 | rgb(0.784, 0.553, 0.580)
    #[classattr]
    pub fn GRAYISH_PINK() -> Color {
        Color { r: 0.7843137254901961, g: 0.5529411764705883, b: 0.5803921568627451, a: 1.0 }
    }

    /// GRAYISH_BROWN | #7A6A4F | rgb(0.478, 0.416, 0.310)
    #[classattr]
    pub fn GRAYISH_BROWN() -> Color {
        Color { r: 0.47843137254901963, g: 0.41568627450980394, b: 0.30980392156862746, a: 1.0 }
    }

    /// STEEL_GRAY | #6F828A | rgb(0.435, 0.510, 0.541)
    #[classattr]
    pub fn STEEL_GRAY() -> Color {
        Color { r: 0.43529411764705883, g: 0.5098039215686274, b: 0.5411764705882353, a: 1.0 }
    }

    /// PURPLE_GRAY | #866F85 | rgb(0.525, 0.435, 0.522)
    #[classattr]
    pub fn PURPLE_GRAY() -> Color {
        Color { r: 0.5254901960784314, g: 0.43529411764705883, b: 0.5215686274509804, a: 1.0 }
    }

    /// GRAY_BROWN | #7F7053 | rgb(0.498, 0.439, 0.325)
    #[classattr]
    pub fn GRAY_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.4392156862745098, b: 0.3254901960784314, a: 1.0 }
    }

    /// GREEN_GRAY | #77926F | rgb(0.467, 0.573, 0.435)
    #[classattr]
    pub fn GREEN_GRAY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5725490196078431, b: 0.43529411764705883, a: 1.0 }
    }

    /// BLUISH_GRAY | #748B97 | rgb(0.455, 0.545, 0.592)
    #[classattr]
    pub fn BLUISH_GRAY() -> Color {
        Color { r: 0.4549019607843137, g: 0.5450980392156862, b: 0.592156862745098, a: 1.0 }
    }

    /// SLATE_GRAY | #59656D | rgb(0.349, 0.396, 0.427)
    #[classattr]
    pub fn SLATE_GRAY() -> Color {
        Color { r: 0.34901960784313724, g: 0.396078431372549, b: 0.42745098039215684, a: 1.0 }
    }

    /// GRAY_PURPLE | #826D8C | rgb(0.510, 0.427, 0.549)
    #[classattr]
    pub fn GRAY_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.42745098039215684, b: 0.5490196078431373, a: 1.0 }
    }

    /// GREENISH_GRAY | #96AE8D | rgb(0.588, 0.682, 0.553)
    #[classattr]
    pub fn GREENISH_GRAY() -> Color {
        Color { r: 0.5882352941176471, g: 0.6823529411764706, b: 0.5529411764705883, a: 1.0 }
    }

    /// GRAYISH_PURPLE | #887191 | rgb(0.533, 0.443, 0.569)
    #[classattr]
    pub fn GRAYISH_PURPLE() -> Color {
        Color { r: 0.5333333333333333, g: 0.44313725490196076, b: 0.5686274509803921, a: 1.0 }
    }

    /// GRAYISH_GREEN | #82A67D | rgb(0.510, 0.651, 0.490)
    #[classattr]
    pub fn GRAYISH_GREEN() -> Color {
        Color { r: 0.5098039215686274, g: 0.6509803921568628, b: 0.49019607843137253, a: 1.0 }
    }

    /// GRAYISH_BLUE | #5E819D | rgb(0.369, 0.506, 0.616)
    #[classattr]
    pub fn GRAYISH_BLUE() -> Color {
        Color { r: 0.3686274509803922, g: 0.5058823529411764, b: 0.615686274509804, a: 1.0 }
    }

    /// LIGHT_GRAY | #D8DCD6 | rgb(0.847, 0.863, 0.839)
    #[classattr]
    pub fn LIGHT_GRAY() -> Color {
        Color { r: 0.8470588235294118, g: 0.8627450980392157, b: 0.8392156862745098, a: 1.0 }
    }

    /// DARK_GRAY | #363737 | rgb(0.212, 0.216, 0.216)
    #[classattr]
    pub fn DARK_GRAY() -> Color {
        Color { r: 0.21176470588235294, g: 0.21568627450980393, b: 0.21568627450980393, a: 1.0 }
    }

    /// GRAY | #929591 | rgb(0.573, 0.584, 0.569)
    #[classattr]
    pub fn GRAY() -> Color {
        Color { r: 0.5725490196078431, g: 0.5843137254901961, b: 0.5686274509803921, a: 1.0 }
    }

    /// ALICEBLUE | #F0F8FF | rgb(0.941, 0.973, 1.000)
    #[classattr]
    pub fn ALICEBLUE() -> Color {
        Color { r: 0.9411764705882353, g: 0.9725490196078431, b: 1.0, a: 1.0 }
    }

    /// ANTIQUEWHITE | #FAEBD7 | rgb(0.980, 0.922, 0.843)
    #[classattr]
    pub fn ANTIQUEWHITE() -> Color {
        Color { r: 0.9803921568627451, g: 0.9215686274509803, b: 0.8431372549019608, a: 1.0 }
    }

    /// BISQUE | #FFE4C4 | rgb(1.000, 0.894, 0.769)
    #[classattr]
    pub fn BISQUE() -> Color {
        Color { r: 1.0, g: 0.8941176470588236, b: 0.7686274509803922, a: 1.0 }
    }

    /// BLANCHEDALMOND | #FFEBCD | rgb(1.000, 0.922, 0.804)
    #[classattr]
    pub fn BLANCHEDALMOND() -> Color {
        Color { r: 1.0, g: 0.9215686274509803, b: 0.803921568627451, a: 1.0 }
    }

    /// BLUEVIOLET | #8A2BE2 | rgb(0.541, 0.169, 0.886)
    #[classattr]
    pub fn BLUEVIOLET() -> Color {
        Color { r: 0.5411764705882353, g: 0.16862745098039217, b: 0.8862745098039215, a: 1.0 }
    }

    /// BURLYWOOD | #DEB887 | rgb(0.871, 0.722, 0.529)
    #[classattr]
    pub fn BURLYWOOD() -> Color {
        Color { r: 0.8705882352941177, g: 0.7215686274509804, b: 0.5294117647058824, a: 1.0 }
    }

    /// CADETBLUE | #5F9EA0 | rgb(0.373, 0.620, 0.627)
    #[classattr]
    pub fn CADETBLUE() -> Color {
        Color { r: 0.37254901960784315, g: 0.6196078431372549, b: 0.6274509803921569, a: 1.0 }
    }

    /// CORNFLOWERBLUE | #6495ED | rgb(0.392, 0.584, 0.929)
    #[classattr]
    pub fn CORNFLOWERBLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.5843137254901961, b: 0.9294117647058824, a: 1.0 }
    }

    /// CORNSILK | #FFF8DC | rgb(1.000, 0.973, 0.863)
    #[classattr]
    pub fn CORNSILK() -> Color {
        Color { r: 1.0, g: 0.9725490196078431, b: 0.8627450980392157, a: 1.0 }
    }

    /// DARKCYAN | #008B8B | rgb(0.000, 0.545, 0.545)
    #[classattr]
    pub fn DARKCYAN() -> Color {
        Color { r: 0.0, g: 0.5450980392156862, b: 0.5450980392156862, a: 1.0 }
    }

    /// DARKGOLDENROD | #B8860B | rgb(0.722, 0.525, 0.043)
    #[classattr]
    pub fn DARKGOLDENROD() -> Color {
        Color { r: 0.7215686274509804, g: 0.5254901960784314, b: 0.043137254901960784, a: 1.0 }
    }

    /// DARKGRAY | #A9A9A9 | rgb(0.663, 0.663, 0.663)
    #[classattr]
    pub fn DARKGRAY() -> Color {
        Color { r: 0.6627450980392157, g: 0.6627450980392157, b: 0.6627450980392157, a: 1.0 }
    }

    /// DARKGREY | #A9A9A9 | rgb(0.663, 0.663, 0.663)
    #[classattr]
    pub fn DARKGREY() -> Color {
        Color { r: 0.6627450980392157, g: 0.6627450980392157, b: 0.6627450980392157, a: 1.0 }
    }

    /// DARKKHAKI | #BDB76B | rgb(0.741, 0.718, 0.420)
    #[classattr]
    pub fn DARKKHAKI() -> Color {
        Color { r: 0.7411764705882353, g: 0.7176470588235294, b: 0.4196078431372549, a: 1.0 }
    }

    /// DARKMAGENTA | #8B008B | rgb(0.545, 0.000, 0.545)
    #[classattr]
    pub fn DARKMAGENTA() -> Color {
        Color { r: 0.5450980392156862, g: 0.0, b: 0.5450980392156862, a: 1.0 }
    }

    /// DARKOLIVEGREEN | #556B2F | rgb(0.333, 0.420, 0.184)
    #[classattr]
    pub fn DARKOLIVEGREEN() -> Color {
        Color { r: 0.3333333333333333, g: 0.4196078431372549, b: 0.1843137254901961, a: 1.0 }
    }

    /// DARKORANGE | #FF8C00 | rgb(1.000, 0.549, 0.000)
    #[classattr]
    pub fn DARKORANGE() -> Color {
        Color { r: 1.0, g: 0.5490196078431373, b: 0.0, a: 1.0 }
    }

    /// DARKORCHID | #9932CC | rgb(0.600, 0.196, 0.800)
    #[classattr]
    pub fn DARKORCHID() -> Color {
        Color { r: 0.6, g: 0.19607843137254902, b: 0.8, a: 1.0 }
    }

    /// DARKRED | #8B0000 | rgb(0.545, 0.000, 0.000)
    #[classattr]
    pub fn DARKRED() -> Color {
        Color { r: 0.5450980392156862, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// DARKSALMON | #E9967A | rgb(0.914, 0.588, 0.478)
    #[classattr]
    pub fn DARKSALMON() -> Color {
        Color { r: 0.9137254901960784, g: 0.5882352941176471, b: 0.47843137254901963, a: 1.0 }
    }

    /// DARKSEAGREEN | #8FBC8F | rgb(0.561, 0.737, 0.561)
    #[classattr]
    pub fn DARKSEAGREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.7372549019607844, b: 0.5607843137254902, a: 1.0 }
    }

    /// DARKSLATEBLUE | #483D8B | rgb(0.282, 0.239, 0.545)
    #[classattr]
    pub fn DARKSLATEBLUE() -> Color {
        Color { r: 0.2823529411764706, g: 0.23921568627450981, b: 0.5450980392156862, a: 1.0 }
    }

    /// DARKSLATEGRAY | #2F4F4F | rgb(0.184, 0.310, 0.310)
    #[classattr]
    pub fn DARKSLATEGRAY() -> Color {
        Color { r: 0.1843137254901961, g: 0.30980392156862746, b: 0.30980392156862746, a: 1.0 }
    }

    /// DARKSLATEGREY | #2F4F4F | rgb(0.184, 0.310, 0.310)
    #[classattr]
    pub fn DARKSLATEGREY() -> Color {
        Color { r: 0.1843137254901961, g: 0.30980392156862746, b: 0.30980392156862746, a: 1.0 }
    }

    /// DARKTURQUOISE | #00CED1 | rgb(0.000, 0.808, 0.820)
    #[classattr]
    pub fn DARKTURQUOISE() -> Color {
        Color { r: 0.0, g: 0.807843137254902, b: 0.8196078431372549, a: 1.0 }
    }

    /// DARKVIOLET | #9400D3 | rgb(0.580, 0.000, 0.827)
    #[classattr]
    pub fn DARKVIOLET() -> Color {
        Color { r: 0.5803921568627451, g: 0.0, b: 0.8274509803921568, a: 1.0 }
    }

    /// DEEPPINK | #FF1493 | rgb(1.000, 0.078, 0.576)
    #[classattr]
    pub fn DEEPPINK() -> Color {
        Color { r: 1.0, g: 0.0784313725490196, b: 0.5764705882352941, a: 1.0 }
    }

    /// DEEPSKYBLUE | #00BFFF | rgb(0.000, 0.749, 1.000)
    #[classattr]
    pub fn DEEPSKYBLUE() -> Color {
        Color { r: 0.0, g: 0.7490196078431373, b: 1.0, a: 1.0 }
    }

    /// DIMGRAY | #696969 | rgb(0.412, 0.412, 0.412)
    #[classattr]
    pub fn DIMGRAY() -> Color {
        Color { r: 0.4117647058823529, g: 0.4117647058823529, b: 0.4117647058823529, a: 1.0 }
    }

    /// DIMGREY | #696969 | rgb(0.412, 0.412, 0.412)
    #[classattr]
    pub fn DIMGREY() -> Color {
        Color { r: 0.4117647058823529, g: 0.4117647058823529, b: 0.4117647058823529, a: 1.0 }
    }

    /// DODGERBLUE | #1E90FF | rgb(0.118, 0.565, 1.000)
    #[classattr]
    pub fn DODGERBLUE() -> Color {
        Color { r: 0.11764705882352941, g: 0.5647058823529412, b: 1.0, a: 1.0 }
    }

    /// FIREBRICK | #B22222 | rgb(0.698, 0.133, 0.133)
    #[classattr]
    pub fn FIREBRICK() -> Color {
        Color { r: 0.6980392156862745, g: 0.13333333333333333, b: 0.13333333333333333, a: 1.0 }
    }

    /// FLORALWHITE | #FFFAF0 | rgb(1.000, 0.980, 0.941)
    #[classattr]
    pub fn FLORALWHITE() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.9411764705882353, a: 1.0 }
    }

    /// FORESTGREEN | #228B22 | rgb(0.133, 0.545, 0.133)
    #[classattr]
    pub fn FORESTGREEN() -> Color {
        Color { r: 0.13333333333333333, g: 0.5450980392156862, b: 0.13333333333333333, a: 1.0 }
    }

    /// GAINSBORO | #DCDCDC | rgb(0.863, 0.863, 0.863)
    #[classattr]
    pub fn GAINSBORO() -> Color {
        Color { r: 0.8627450980392157, g: 0.8627450980392157, b: 0.8627450980392157, a: 1.0 }
    }

    /// GHOSTWHITE | #F8F8FF | rgb(0.973, 0.973, 1.000)
    #[classattr]
    pub fn GHOSTWHITE() -> Color {
        Color { r: 0.9725490196078431, g: 0.9725490196078431, b: 1.0, a: 1.0 }
    }

    /// GREENYELLOW | #ADFF2F | rgb(0.678, 1.000, 0.184)
    #[classattr]
    pub fn GREENYELLOW() -> Color {
        Color { r: 0.6784313725490196, g: 1.0, b: 0.1843137254901961, a: 1.0 }
    }

    /// HONEYDEW | #F0FFF0 | rgb(0.941, 1.000, 0.941)
    #[classattr]
    pub fn HONEYDEW() -> Color {
        Color { r: 0.9411764705882353, g: 1.0, b: 0.9411764705882353, a: 1.0 }
    }

    /// HOTPINK | #FF69B4 | rgb(1.000, 0.412, 0.706)
    #[classattr]
    pub fn HOTPINK() -> Color {
        Color { r: 1.0, g: 0.4117647058823529, b: 0.7058823529411765, a: 1.0 }
    }

    /// INDIANRED | #CD5C5C | rgb(0.804, 0.361, 0.361)
    #[classattr]
    pub fn INDIANRED() -> Color {
        Color { r: 0.803921568627451, g: 0.3607843137254902, b: 0.3607843137254902, a: 1.0 }
    }

    /// LAVENDERBLUSH | #FFF0F5 | rgb(1.000, 0.941, 0.961)
    #[classattr]
    pub fn LAVENDERBLUSH() -> Color {
        Color { r: 1.0, g: 0.9411764705882353, b: 0.9607843137254902, a: 1.0 }
    }

    /// LAWNGREEN | #7CFC00 | rgb(0.486, 0.988, 0.000)
    #[classattr]
    pub fn LAWNGREEN() -> Color {
        Color { r: 0.48627450980392156, g: 0.9882352941176471, b: 0.0, a: 1.0 }
    }

    /// LEMONCHIFFON | #FFFACD | rgb(1.000, 0.980, 0.804)
    #[classattr]
    pub fn LEMONCHIFFON() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.803921568627451, a: 1.0 }
    }

    /// LIGHTCORAL | #F08080 | rgb(0.941, 0.502, 0.502)
    #[classattr]
    pub fn LIGHTCORAL() -> Color {
        Color { r: 0.9411764705882353, g: 0.5019607843137255, b: 0.5019607843137255, a: 1.0 }
    }

    /// LIGHTCYAN | #E0FFFF | rgb(0.878, 1.000, 1.000)
    #[classattr]
    pub fn LIGHTCYAN() -> Color {
        Color { r: 0.8784313725490196, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// LIGHTGOLDENRODYELLOW | #FAFAD2 | rgb(0.980, 0.980, 0.824)
    #[classattr]
    pub fn LIGHTGOLDENRODYELLOW() -> Color {
        Color { r: 0.9803921568627451, g: 0.9803921568627451, b: 0.8235294117647058, a: 1.0 }
    }

    /// LIGHTGRAY | #D3D3D3 | rgb(0.827, 0.827, 0.827)
    #[classattr]
    pub fn LIGHTGRAY() -> Color {
        Color { r: 0.8274509803921568, g: 0.8274509803921568, b: 0.8274509803921568, a: 1.0 }
    }

    /// LIGHTGREY | #D3D3D3 | rgb(0.827, 0.827, 0.827)
    #[classattr]
    pub fn LIGHTGREY() -> Color {
        Color { r: 0.8274509803921568, g: 0.8274509803921568, b: 0.8274509803921568, a: 1.0 }
    }

    /// LIGHTPINK | #FFB6C1 | rgb(1.000, 0.714, 0.757)
    #[classattr]
    pub fn LIGHTPINK() -> Color {
        Color { r: 1.0, g: 0.7137254901960784, b: 0.7568627450980392, a: 1.0 }
    }

    /// LIGHTSALMON | #FFA07A | rgb(1.000, 0.627, 0.478)
    #[classattr]
    pub fn LIGHTSALMON() -> Color {
        Color { r: 1.0, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 }
    }

    /// LIGHTSEAGREEN | #20B2AA | rgb(0.125, 0.698, 0.667)
    #[classattr]
    pub fn LIGHTSEAGREEN() -> Color {
        Color { r: 0.12549019607843137, g: 0.6980392156862745, b: 0.6666666666666666, a: 1.0 }
    }

    /// LIGHTSKYBLUE | #87CEFA | rgb(0.529, 0.808, 0.980)
    #[classattr]
    pub fn LIGHTSKYBLUE() -> Color {
        Color { r: 0.5294117647058824, g: 0.807843137254902, b: 0.9803921568627451, a: 1.0 }
    }

    /// LIGHTSLATEGRAY | #778899 | rgb(0.467, 0.533, 0.600)
    #[classattr]
    pub fn LIGHTSLATEGRAY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5333333333333333, b: 0.6, a: 1.0 }
    }

    /// LIGHTSLATEGREY | #778899 | rgb(0.467, 0.533, 0.600)
    #[classattr]
    pub fn LIGHTSLATEGREY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5333333333333333, b: 0.6, a: 1.0 }
    }

    /// LIGHTSTEELBLUE | #B0C4DE | rgb(0.690, 0.769, 0.871)
    #[classattr]
    pub fn LIGHTSTEELBLUE() -> Color {
        Color { r: 0.6901960784313725, g: 0.7686274509803922, b: 0.8705882352941177, a: 1.0 }
    }

    /// LIGHTYELLOW | #FFFFE0 | rgb(1.000, 1.000, 0.878)
    #[classattr]
    pub fn LIGHTYELLOW() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.8784313725490196, a: 1.0 }
    }

    /// LIMEGREEN | #32CD32 | rgb(0.196, 0.804, 0.196)
    #[classattr]
    pub fn LIMEGREEN() -> Color {
        Color { r: 0.19607843137254902, g: 0.803921568627451, b: 0.19607843137254902, a: 1.0 }
    }

    /// LINEN | #FAF0E6 | rgb(0.980, 0.941, 0.902)
    #[classattr]
    pub fn LINEN() -> Color {
        Color { r: 0.9803921568627451, g: 0.9411764705882353, b: 0.9019607843137255, a: 1.0 }
    }

    /// MEDIUMAQUAMARINE | #66CDAA | rgb(0.400, 0.804, 0.667)
    #[classattr]
    pub fn MEDIUMAQUAMARINE() -> Color {
        Color { r: 0.4, g: 0.803921568627451, b: 0.6666666666666666, a: 1.0 }
    }

    /// MEDIUMBLUE | #0000CD | rgb(0.000, 0.000, 0.804)
    #[classattr]
    pub fn MEDIUMBLUE() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.803921568627451, a: 1.0 }
    }

    /// MEDIUMORCHID | #BA55D3 | rgb(0.729, 0.333, 0.827)
    #[classattr]
    pub fn MEDIUMORCHID() -> Color {
        Color { r: 0.7294117647058823, g: 0.3333333333333333, b: 0.8274509803921568, a: 1.0 }
    }

    /// MEDIUMPURPLE | #9370DB | rgb(0.576, 0.439, 0.859)
    #[classattr]
    pub fn MEDIUMPURPLE() -> Color {
        Color { r: 0.5764705882352941, g: 0.4392156862745098, b: 0.8588235294117647, a: 1.0 }
    }

    /// MEDIUMSEAGREEN | #3CB371 | rgb(0.235, 0.702, 0.443)
    #[classattr]
    pub fn MEDIUMSEAGREEN() -> Color {
        Color { r: 0.23529411764705882, g: 0.7019607843137254, b: 0.44313725490196076, a: 1.0 }
    }

    /// MEDIUMSLATEBLUE | #7B68EE | rgb(0.482, 0.408, 0.933)
    #[classattr]
    pub fn MEDIUMSLATEBLUE() -> Color {
        Color { r: 0.4823529411764706, g: 0.40784313725490196, b: 0.9333333333333333, a: 1.0 }
    }

    /// MEDIUMSPRINGGREEN | #00FA9A | rgb(0.000, 0.980, 0.604)
    #[classattr]
    pub fn MEDIUMSPRINGGREEN() -> Color {
        Color { r: 0.0, g: 0.9803921568627451, b: 0.6039215686274509, a: 1.0 }
    }

    /// MEDIUMTURQUOISE | #48D1CC | rgb(0.282, 0.820, 0.800)
    #[classattr]
    pub fn MEDIUMTURQUOISE() -> Color {
        Color { r: 0.2823529411764706, g: 0.8196078431372549, b: 0.8, a: 1.0 }
    }

    /// MEDIUMVIOLETRED | #C71585 | rgb(0.780, 0.082, 0.522)
    #[classattr]
    pub fn MEDIUMVIOLETRED() -> Color {
        Color { r: 0.7803921568627451, g: 0.08235294117647059, b: 0.5215686274509804, a: 1.0 }
    }

    /// MIDNIGHTBLUE | #191970 | rgb(0.098, 0.098, 0.439)
    #[classattr]
    pub fn MIDNIGHTBLUE() -> Color {
        Color { r: 0.09803921568627451, g: 0.09803921568627451, b: 0.4392156862745098, a: 1.0 }
    }

    /// MINTCREAM | #F5FFFA | rgb(0.961, 1.000, 0.980)
    #[classattr]
    pub fn MINTCREAM() -> Color {
        Color { r: 0.9607843137254902, g: 1.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// MISTYROSE | #FFE4E1 | rgb(1.000, 0.894, 0.882)
    #[classattr]
    pub fn MISTYROSE() -> Color {
        Color { r: 1.0, g: 0.8941176470588236, b: 0.8823529411764706, a: 1.0 }
    }

    /// MOCCASIN | #FFE4B5 | rgb(1.000, 0.894, 0.710)
    #[classattr]
    pub fn MOCCASIN() -> Color {
        Color { r: 1.0, g: 0.8941176470588236, b: 0.7098039215686275, a: 1.0 }
    }

    /// NAVAJOWHITE | #FFDEAD | rgb(1.000, 0.871, 0.678)
    #[classattr]
    pub fn NAVAJOWHITE() -> Color {
        Color { r: 1.0, g: 0.8705882352941177, b: 0.6784313725490196, a: 1.0 }
    }

    /// OLDLACE | #FDF5E6 | rgb(0.992, 0.961, 0.902)
    #[classattr]
    pub fn OLDLACE() -> Color {
        Color { r: 0.9921568627450981, g: 0.9607843137254902, b: 0.9019607843137255, a: 1.0 }
    }

    /// OLIVEDRAB | #6B8E23 | rgb(0.420, 0.557, 0.137)
    #[classattr]
    pub fn OLIVEDRAB() -> Color {
        Color { r: 0.4196078431372549, g: 0.5568627450980392, b: 0.13725490196078433, a: 1.0 }
    }

    /// PALEGOLDENROD | #EEE8AA | rgb(0.933, 0.910, 0.667)
    #[classattr]
    pub fn PALEGOLDENROD() -> Color {
        Color { r: 0.9333333333333333, g: 0.9098039215686274, b: 0.6666666666666666, a: 1.0 }
    }

    /// PALEGREEN | #98FB98 | rgb(0.596, 0.984, 0.596)
    #[classattr]
    pub fn PALEGREEN() -> Color {
        Color { r: 0.596078431372549, g: 0.984313725490196, b: 0.596078431372549, a: 1.0 }
    }

    /// PALETURQUOISE | #AFEEEE | rgb(0.686, 0.933, 0.933)
    #[classattr]
    pub fn PALETURQUOISE() -> Color {
        Color { r: 0.6862745098039216, g: 0.9333333333333333, b: 0.9333333333333333, a: 1.0 }
    }

    /// PALEVIOLETRED | #DB7093 | rgb(0.859, 0.439, 0.576)
    #[classattr]
    pub fn PALEVIOLETRED() -> Color {
        Color { r: 0.8588235294117647, g: 0.4392156862745098, b: 0.5764705882352941, a: 1.0 }
    }

    /// PAPAYAWHIP | #FFEFD5 | rgb(1.000, 0.937, 0.835)
    #[classattr]
    pub fn PAPAYAWHIP() -> Color {
        Color { r: 1.0, g: 0.9372549019607843, b: 0.8352941176470589, a: 1.0 }
    }

    /// PEACHPUFF | #FFDAB9 | rgb(1.000, 0.855, 0.725)
    #[classattr]
    pub fn PEACHPUFF() -> Color {
        Color { r: 1.0, g: 0.8549019607843137, b: 0.7254901960784313, a: 1.0 }
    }

    /// PERU | #CD853F | rgb(0.804, 0.522, 0.247)
    #[classattr]
    pub fn PERU() -> Color {
        Color { r: 0.803921568627451, g: 0.5215686274509804, b: 0.24705882352941178, a: 1.0 }
    }

    /// POWDERBLUE | #B0E0E6 | rgb(0.690, 0.878, 0.902)
    #[classattr]
    pub fn POWDERBLUE() -> Color {
        Color { r: 0.6901960784313725, g: 0.8784313725490196, b: 0.9019607843137255, a: 1.0 }
    }

    /// REBECCAPURPLE | #663399 | rgb(0.400, 0.200, 0.600)
    #[classattr]
    pub fn REBECCAPURPLE() -> Color {
        Color { r: 0.4, g: 0.2, b: 0.6, a: 1.0 }
    }

    /// ROSYBROWN | #BC8F8F | rgb(0.737, 0.561, 0.561)
    #[classattr]
    pub fn ROSYBROWN() -> Color {
        Color { r: 0.7372549019607844, g: 0.5607843137254902, b: 0.5607843137254902, a: 1.0 }
    }

    /// ROYALBLUE | #4169E1 | rgb(0.255, 0.412, 0.882)
    #[classattr]
    pub fn ROYALBLUE() -> Color {
        Color { r: 0.2549019607843137, g: 0.4117647058823529, b: 0.8823529411764706, a: 1.0 }
    }

    /// SADDLEBROWN | #8B4513 | rgb(0.545, 0.271, 0.075)
    #[classattr]
    pub fn SADDLEBROWN() -> Color {
        Color { r: 0.5450980392156862, g: 0.27058823529411763, b: 0.07450980392156863, a: 1.0 }
    }

    /// SANDYBROWN | #F4A460 | rgb(0.957, 0.643, 0.376)
    #[classattr]
    pub fn SANDYBROWN() -> Color {
        Color { r: 0.9568627450980393, g: 0.6431372549019608, b: 0.3764705882352941, a: 1.0 }
    }

    /// SEAGREEN | #2E8B57 | rgb(0.180, 0.545, 0.341)
    #[classattr]
    pub fn SEAGREEN() -> Color {
        Color { r: 0.1803921568627451, g: 0.5450980392156862, b: 0.3411764705882353, a: 1.0 }
    }

    /// SEASHELL | #FFF5EE | rgb(1.000, 0.961, 0.933)
    #[classattr]
    pub fn SEASHELL() -> Color {
        Color { r: 1.0, g: 0.9607843137254902, b: 0.9333333333333333, a: 1.0 }
    }

    /// SKYBLUE | #87CEEB | rgb(0.529, 0.808, 0.922)
    #[classattr]
    pub fn SKYBLUE() -> Color {
        Color { r: 0.5294117647058824, g: 0.807843137254902, b: 0.9215686274509803, a: 1.0 }
    }

    /// SLATEBLUE | #6A5ACD | rgb(0.416, 0.353, 0.804)
    #[classattr]
    pub fn SLATEBLUE() -> Color {
        Color { r: 0.41568627450980394, g: 0.35294117647058826, b: 0.803921568627451, a: 1.0 }
    }

    /// SLATEGRAY | #708090 | rgb(0.439, 0.502, 0.565)
    #[classattr]
    pub fn SLATEGRAY() -> Color {
        Color { r: 0.4392156862745098, g: 0.5019607843137255, b: 0.5647058823529412, a: 1.0 }
    }

    /// SLATEGREY | #708090 | rgb(0.439, 0.502, 0.565)
    #[classattr]
    pub fn SLATEGREY() -> Color {
        Color { r: 0.4392156862745098, g: 0.5019607843137255, b: 0.5647058823529412, a: 1.0 }
    }

    /// SNOW | #FFFAFA | rgb(1.000, 0.980, 0.980)
    #[classattr]
    pub fn SNOW() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.9803921568627451, a: 1.0 }
    }

    /// SPRINGGREEN | #00FF7F | rgb(0.000, 1.000, 0.498)
    #[classattr]
    pub fn SPRINGGREEN() -> Color {
        Color { r: 0.0, g: 1.0, b: 0.4980392156862745, a: 1.0 }
    }

    /// STEELBLUE | #4682B4 | rgb(0.275, 0.510, 0.706)
    #[classattr]
    pub fn STEELBLUE() -> Color {
        Color { r: 0.27450980392156865, g: 0.5098039215686274, b: 0.7058823529411765, a: 1.0 }
    }

    /// THISTLE | #D8BFD8 | rgb(0.847, 0.749, 0.847)
    #[classattr]
    pub fn THISTLE() -> Color {
        Color { r: 0.8470588235294118, g: 0.7490196078431373, b: 0.8470588235294118, a: 1.0 }
    }

    /// WHITESMOKE | #F5F5F5 | rgb(0.961, 0.961, 0.961)
    #[classattr]
    pub fn WHITESMOKE() -> Color {
        Color { r: 0.9607843137254902, g: 0.9607843137254902, b: 0.9607843137254902, a: 1.0 }
    }

    /// INVISIBLE | #000000 | rgba(0.000, 0.000, 0.000, 0.000)
    #[classattr]
    pub fn INVISIBLE() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }

    /// HALF_TRANSPARENT | #FFFFFF | rgba(1.000, 1.000, 1.000, 0.5)
    #[classattr]
    pub fn HALF_TRANSPARENT() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 }
    }
}


impl From<macroquad::prelude::Color> for Color {
    fn from(t: macroquad::prelude::Color) -> Self {
        Color { r: t.r, g: t.g, b: t.b, a: t.a }
    }
}

impl From<Color> for macroquad::prelude::Color {
    fn from(t: Color) -> Self {
        macroquad::prelude::Color {  r: t.r, g: t.g, b: t.b, a: t.a  }
    }
}
