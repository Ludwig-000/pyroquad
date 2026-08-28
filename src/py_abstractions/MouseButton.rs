use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;



use macroquad::prelude as mq;


#[cfg_attr(feature = "abi_314", pyclass(eq, hash, frozen, immutable_type,from_py_object))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq, hash, frozen,from_py_object))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
    Unknown = 255,
}



impl From<mq::MouseButton> for MouseButton {

    fn from(mq_key: mq::MouseButton) -> Self {
        match mq_key {
            mq::MouseButton::Left => MouseButton::Left,
            mq::MouseButton::Middle => MouseButton::Middle,
            mq::MouseButton::Right => MouseButton::Right,
            mq::MouseButton::Unknown => MouseButton::Unknown,
        }
    }
}
