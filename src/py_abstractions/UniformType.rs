use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;
use macroquad::prelude as mq;


use crate::{engine::{PArc::PArc, PChannel::PChannel}, py_abstractions::{Loading::FileData::FileData, Textures_and_Images::FilterMode}};
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;




#[pyclass(from_py_object)]
#[derive(Debug, Clone, Copy)]
pub enum UniformType {
    /// One 32-bit wide float (equivalent to `f32`)
    Float1,
    /// Two 32-bit wide floats (equivalent to `[f32; 2]`)
    Float2,
    /// Three 32-bit wide floats (equivalent to `[f32; 3]`)
    Float3,
    /// Four 32-bit wide floats (equivalent to `[f32; 4]`)
    Float4,
    /// One unsigned 32-bit integers (equivalent to `[u32; 1]`)
    Int1,
    /// Two unsigned 32-bit integers (equivalent to `[u32; 2]`)
    Int2,
    /// Three unsigned 32-bit integers (equivalent to `[u32; 3]`)
    Int3,
    /// Four unsigned 32-bit integers (equivalent to `[u32; 4]`)
    Int4,
    /// Four by four matrix of 32-bit floats
    Mat4,
}



impl From<UniformType> for mq::UniformType {
    fn from(ut: UniformType) -> Self {
        match ut {
            UniformType::Float1 => mq::UniformType::Float1,
            UniformType::Float2 => mq::UniformType::Float2,
            UniformType::Float3 => mq::UniformType::Float3,
            UniformType::Float4 => mq::UniformType::Float4,
            UniformType::Int1 => mq::UniformType::Int1,
            UniformType::Int2 => mq::UniformType::Int2,
            UniformType::Int3 => mq::UniformType::Int3,
            UniformType::Int4 => mq::UniformType::Int4,
            UniformType::Mat4 => mq::UniformType::Mat4,
        }
    }
}
impl From<mq::UniformType> for UniformType {
    fn from(ut: mq::UniformType) -> Self {
        match ut {
            mq::UniformType::Float1 => UniformType::Float1,
            mq::UniformType::Float2 => UniformType::Float2,
            mq::UniformType::Float3 => UniformType::Float3,
            mq::UniformType::Float4 => UniformType::Float4,
            mq::UniformType::Int1 => UniformType::Int1,
            mq::UniformType::Int2 => UniformType::Int2,
            mq::UniformType::Int3 => UniformType::Int3,
            mq::UniformType::Int4 => UniformType::Int4,
            mq::UniformType::Mat4 => UniformType::Mat4,
        }
    }
}



/// Euler rotation sequences.
///
/// The angles are applied starting from the right.
/// E.g. XYZ will first apply the z-axis rotation.
///
/// YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).

#[pyclass(from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EulerRot {
    /// Intrinsic three-axis rotation ZYX
    ZYX,
    /// Intrinsic three-axis rotation ZXY
    ZXY,
    /// Intrinsic three-axis rotation YXZ
    YXZ,
    /// Intrinsic three-axis rotation YZX
    YZX,
    /// Intrinsic three-axis rotation XYZ
    XYZ,
    /// Intrinsic three-axis rotation XZY
    XZY,
}

impl From<EulerRot> for mq::EulerRot {
    fn from(er: EulerRot) -> Self {
        match er {
            EulerRot::ZYX => mq::EulerRot::ZYX,
            EulerRot::ZXY => mq::EulerRot::ZXY,
            EulerRot::YXZ => mq::EulerRot::YXZ,
            EulerRot::YZX => mq::EulerRot::YZX,
            EulerRot::XYZ => mq::EulerRot::XYZ,
            EulerRot::XZY => mq::EulerRot::XZY,
        }
    }
}
impl From<mq::EulerRot> for EulerRot {
    fn from(er: mq::EulerRot) -> Self {
        match er {
            mq::EulerRot::ZYX => EulerRot::ZYX,
            mq::EulerRot::ZXY => EulerRot::ZXY,
            mq::EulerRot::YXZ => EulerRot::YXZ,
            mq::EulerRot::YZX => EulerRot::YZX,
            mq::EulerRot::XYZ => EulerRot::XYZ,
            mq::EulerRot::XZY => EulerRot::XZY,
        }
    }
}
impl Default for EulerRot {
    /// Default `YXZ` as yaw (y-axis), pitch (x-axis), roll (z-axis).
    fn default() -> Self {
        Self::YXZ
    }
}




/// A pixel-wise comparison function.

#[pyclass(from_py_object)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Comparison {
    Never,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    Always,
}


impl From<Comparison> for mq::Comparison {
    fn from(cmp: Comparison) -> Self {
        match cmp {
            Comparison::Never => mq::Comparison::Never,
            Comparison::Less => mq::Comparison::Less,
            Comparison::LessOrEqual => mq::Comparison::LessOrEqual,
            Comparison::Greater => mq::Comparison::Greater,
            Comparison::GreaterOrEqual => mq::Comparison::GreaterOrEqual,
            Comparison::Equal => mq::Comparison::Equal,
            Comparison::NotEqual => mq::Comparison::NotEqual,
            Comparison::Always => mq::Comparison::Always,
        }
    }
}
impl From<mq::Comparison> for Comparison {
    fn from(cmp: mq::Comparison) -> Self {
        match cmp {
            mq::Comparison::Never => Comparison::Never,
            mq::Comparison::Less => Comparison::Less,
            mq::Comparison::LessOrEqual => Comparison::LessOrEqual,
            mq::Comparison::Greater => Comparison::Greater,
            mq::Comparison::GreaterOrEqual => Comparison::GreaterOrEqual,
            mq::Comparison::Equal => Comparison::Equal,
            mq::Comparison::NotEqual => Comparison::NotEqual,
            mq::Comparison::Always => Comparison::Always,
        }
    }
}