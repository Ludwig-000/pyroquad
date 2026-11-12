//! Python versions of miniquad's pipeline settings: everything a `Material` fixes at creation
//! besides its shaders, uniforms and textures.

use macroquad::miniquad as mi;
use pyo3::prelude::*;

use crate::py_abstractions::UniformType::Comparison;


/// Which side of a triangle is skipped when drawing.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CullFace {
    /// draw both sides (default)
    Nothing,
    /// skip triangles facing the camera
    Front,
    /// skip triangles facing away from the camera
    Back,
}

impl From<CullFace> for mi::CullFace {
    fn from(v: CullFace) -> Self {
        match v {
            CullFace::Nothing => mi::CullFace::Nothing,
            CullFace::Front => mi::CullFace::Front,
            CullFace::Back => mi::CullFace::Back,
        }
    }
}


/// The vertex order that makes a triangle face the camera.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FrontFaceOrder {
    Clockwise,
    /// default
    CounterClockwise,
}

impl From<FrontFaceOrder> for mi::FrontFaceOrder {
    fn from(v: FrontFaceOrder) -> Self {
        match v {
            FrontFaceOrder::Clockwise => mi::FrontFaceOrder::Clockwise,
            FrontFaceOrder::CounterClockwise => mi::FrontFaceOrder::CounterClockwise,
        }
    }
}


/// How the weighted source (new pixel) and destination (pixel already on screen) are combined.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Equation {
    /// source + destination
    Add,
    /// source - destination
    Subtract,
    /// destination - source
    ReverseSubtract,
}

impl From<Equation> for mi::Equation {
    fn from(v: Equation) -> Self {
        match v {
            Equation::Add => mi::Equation::Add,
            Equation::Subtract => mi::Equation::Subtract,
            Equation::ReverseSubtract => mi::Equation::ReverseSubtract,
        }
    }
}


/// A value a `BlendFactor` can be built from.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendValue {
    SourceColor,
    SourceAlpha,
    DestinationColor,
    DestinationAlpha,
}

impl From<BlendValue> for mi::BlendValue {
    fn from(v: BlendValue) -> Self {
        match v {
            BlendValue::SourceColor => mi::BlendValue::SourceColor,
            BlendValue::SourceAlpha => mi::BlendValue::SourceAlpha,
            BlendValue::DestinationColor => mi::BlendValue::DestinationColor,
            BlendValue::DestinationAlpha => mi::BlendValue::DestinationAlpha,
        }
    }
}


/// What the source or destination is multiplied by before the `Equation` combines them.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendFactor {
    /// 0
    Zero(),
    /// 1
    One(),
    /// the given value
    Value(BlendValue),
    /// 1 - the given value
    OneMinusValue(BlendValue),
    /// min(source alpha, 1 - destination alpha)
    SourceAlphaSaturate(),
}

impl From<BlendFactor> for mi::BlendFactor {
    fn from(v: BlendFactor) -> Self {
        match v {
            BlendFactor::Zero() => mi::BlendFactor::Zero,
            BlendFactor::One() => mi::BlendFactor::One,
            BlendFactor::Value(b) => mi::BlendFactor::Value(b.into()),
            BlendFactor::OneMinusValue(b) => mi::BlendFactor::OneMinusValue(b.into()),
            BlendFactor::SourceAlphaSaturate() => mi::BlendFactor::SourceAlphaSaturate,
        }
    }
}


/// One blend function: `equation(source * sfactor, destination * dfactor)`.
///
/// Usual alpha blending:
/// ```
/// BlendState(Equation.Add,
///            BlendFactor.Value(BlendValue.SourceAlpha),
///            BlendFactor.OneMinusValue(BlendValue.SourceAlpha))
/// ```
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BlendState {
    #[pyo3(get)]
    pub equation: Equation,
    #[pyo3(get)]
    pub sfactor: BlendFactor,
    #[pyo3(get)]
    pub dfactor: BlendFactor,
}

#[pymethods]
impl BlendState {
    #[new]
    const fn new(equation: Equation, sfactor: BlendFactor, dfactor: BlendFactor) -> Self {
        BlendState { equation, sfactor, dfactor }
    }
}

impl From<BlendState> for mi::BlendState {
    fn from(v: BlendState) -> Self {
        mi::BlendState::new(v.equation.into(), v.sfactor.into(), v.dfactor.into())
    }
}


/// What happens to the stencil value of a pixel.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StencilOp {
    /// leave it (default)
    Keep,
    Zero,
    /// set it to `test_ref`
    Replace,
    IncrementClamp,
    DecrementClamp,
    Invert,
    IncrementWrap,
    DecrementWrap,
}

impl From<StencilOp> for mi::StencilOp {
    fn from(v: StencilOp) -> Self {
        match v {
            StencilOp::Keep => mi::StencilOp::Keep,
            StencilOp::Zero => mi::StencilOp::Zero,
            StencilOp::Replace => mi::StencilOp::Replace,
            StencilOp::IncrementClamp => mi::StencilOp::IncrementClamp,
            StencilOp::DecrementClamp => mi::StencilOp::DecrementClamp,
            StencilOp::Invert => mi::StencilOp::Invert,
            StencilOp::IncrementWrap => mi::StencilOp::IncrementWrap,
            StencilOp::DecrementWrap => mi::StencilOp::DecrementWrap,
        }
    }
}

impl From<Comparison> for mi::CompareFunc {
    fn from(v: Comparison) -> Self {
        match v {
            Comparison::Never => mi::CompareFunc::Never,
            Comparison::Less => mi::CompareFunc::Less,
            Comparison::LessOrEqual => mi::CompareFunc::LessOrEqual,
            Comparison::Greater => mi::CompareFunc::Greater,
            Comparison::GreaterOrEqual => mi::CompareFunc::GreaterOrEqual,
            Comparison::Equal => mi::CompareFunc::Equal,
            Comparison::NotEqual => mi::CompareFunc::NotEqual,
            Comparison::Always => mi::CompareFunc::Always,
        }
    }
}


/// The stencil test for one side of the triangles.
/// A pixel passes when `(test_ref & test_mask) <test_func> (stencil & test_mask)`.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StencilFaceState {
    /// when the stencil test fails
    #[pyo3(get)]
    pub fail_op: StencilOp,
    /// when the stencil test passes but the depth test fails
    #[pyo3(get)]
    pub depth_fail_op: StencilOp,
    /// when both pass (or the stencil test passes and there is no depth test)
    #[pyo3(get)]
    pub pass_op: StencilOp,
    #[pyo3(get)]
    pub test_func: Comparison,
    #[pyo3(get)]
    pub test_ref: i32,
    #[pyo3(get)]
    pub test_mask: u32,
    /// which stencil bits may be written
    #[pyo3(get)]
    pub write_mask: u32,
}

#[pymethods]
impl StencilFaceState {
    #[new]
    #[pyo3(signature = (
        fail_op = StencilOp::Keep,
        depth_fail_op = StencilOp::Keep,
        pass_op = StencilOp::Keep,
        test_func = Comparison::Always,
        test_ref = 0,
        test_mask = u32::MAX,
        write_mask = u32::MAX,
    ))]
    const fn new(
        fail_op: StencilOp,
        depth_fail_op: StencilOp,
        pass_op: StencilOp,
        test_func: Comparison,
        test_ref: i32,
        test_mask: u32,
        write_mask: u32,
    ) -> Self {
        StencilFaceState { fail_op, depth_fail_op, pass_op, test_func, test_ref, test_mask, write_mask }
    }
}

impl From<StencilFaceState> for mi::StencilFaceState {
    fn from(v: StencilFaceState) -> Self {
        mi::StencilFaceState {
            fail_op: v.fail_op.into(),
            depth_fail_op: v.depth_fail_op.into(),
            pass_op: v.pass_op.into(),
            test_func: v.test_func.into(),
            test_ref: v.test_ref,
            test_mask: v.test_mask,
            write_mask: v.write_mask,
        }
    }
}


/// The stencil test, separately for front- and back-facing triangles.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StencilState {
    #[pyo3(get)]
    pub front: StencilFaceState,
    #[pyo3(get)]
    pub back: StencilFaceState,
}

#[pymethods]
impl StencilState {
    #[new]
    const fn new(front: StencilFaceState, back: StencilFaceState) -> Self {
        StencilState { front, back }
    }
}

impl From<StencilState> for mi::StencilState {
    fn from(v: StencilState) -> Self {
        mi::StencilState { front: v.front.into(), back: v.back.into() }
    }
}


/// The kind of geometry the material draws.
/// Most draw functions produce triangles; lines come from functions like
/// `draw_line_3d` or the `*_wires` functions.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitiveType {
    /// default
    Triangles,
    Lines,
    Points,
}

impl From<PrimitiveType> for mi::PrimitiveType {
    fn from(v: PrimitiveType) -> Self {
        match v {
            PrimitiveType::Triangles => mi::PrimitiveType::Triangles,
            PrimitiveType::Lines => mi::PrimitiveType::Lines,
            PrimitiveType::Points => mi::PrimitiveType::Points,
        }
    }
}


/// Fixed-function settings of a `Material`, decided once at creation.
///
/// The defaults draw like 2D: no depth test, no depth write, no blending.
/// For 3D, pass at least `depth_test=Comparison.LessOrEqual, depth_write=True`.
/// For transparency, pass a `color_blend`.
#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PipelineParams {
    #[pyo3(get)]
    pub cull_face: CullFace,
    #[pyo3(get)]
    pub front_face_order: FrontFaceOrder,
    /// `Comparison.Always` means no depth test.
    #[pyo3(get)]
    pub depth_test: Comparison,
    #[pyo3(get)]
    pub depth_write: bool,
    /// `(factor, units)` depth offset, against z-fighting. None means no offset.
    #[pyo3(get)]
    pub depth_write_offset: Option<(f32, f32)>,
    /// RGB blend function. None disables blending.
    #[pyo3(get)]
    pub color_blend: Option<BlendState>,
    /// Alpha blend function. None blends alpha the same way as `color_blend`.
    #[pyo3(get)]
    pub alpha_blend: Option<BlendState>,
    /// None disables the stencil test.
    #[pyo3(get)]
    pub stencil_test: Option<StencilState>,
    /// Which of (r, g, b, a) are written.
    #[pyo3(get)]
    pub color_write: (bool, bool, bool, bool),
    #[pyo3(get)]
    pub primitive_type: PrimitiveType,
}

#[pymethods]
impl PipelineParams {
    #[new]
    #[pyo3(signature = (
        cull_face = CullFace::Nothing,
        front_face_order = FrontFaceOrder::CounterClockwise,
        depth_test = Comparison::Always,
        depth_write = false,
        depth_write_offset = None,
        color_blend = None,
        alpha_blend = None,
        stencil_test = None,
        color_write = (true, true, true, true),
        primitive_type = PrimitiveType::Triangles,
    ))]
    const fn new(
        cull_face: CullFace,
        front_face_order: FrontFaceOrder,
        depth_test: Comparison,
        depth_write: bool,
        depth_write_offset: Option<(f32, f32)>,
        color_blend: Option<BlendState>,
        alpha_blend: Option<BlendState>,
        stencil_test: Option<StencilState>,
        color_write: (bool, bool, bool, bool),
        primitive_type: PrimitiveType,
    ) -> Self {
        PipelineParams {
            cull_face, front_face_order, depth_test, depth_write, depth_write_offset,
            color_blend, alpha_blend, stencil_test, color_write, primitive_type,
        }
    }
}

impl Default for PipelineParams {
    fn default() -> Self {
        PipelineParams::new(
            CullFace::Nothing, FrontFaceOrder::CounterClockwise, Comparison::Always, false, None,
            None, None, None, (true, true, true, true), PrimitiveType::Triangles,
        )
    }
}

impl From<PipelineParams> for mi::PipelineParams {
    fn from(v: PipelineParams) -> Self {
        mi::PipelineParams {
            cull_face: v.cull_face.into(),
            front_face_order: v.front_face_order.into(),
            depth_test: v.depth_test.into(),
            depth_write: v.depth_write,
            depth_write_offset: v.depth_write_offset,
            color_blend: v.color_blend.map(Into::into),
            alpha_blend: v.alpha_blend.map(Into::into),
            stencil_test: v.stencil_test.map(Into::into),
            color_write: v.color_write,
            primitive_type: v.primitive_type.into(),
        }
    }
}
