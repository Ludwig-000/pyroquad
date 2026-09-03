use std::sync::Arc;

use crate::py_abstractions::structs::TwoDObjects::{Circle::Circle, Rectangle::Rectangle};





#[macro_export]
macro_rules! implement_Drop2D {
    ($name:ident) => {
        paste::paste! {
            impl Drop for $name {
                fn drop(&mut self) {
                    // function storage MUST be cleaned first, since a function inside fun-storage may rely on the object still living.
                    if let Some(key) = self.function_key{
                        crate::py_abstractions::structs::ThreeDObjects::ObjectFunStorage::remove_function(key);
                    }
                }
            }
        }
    };
}



#[macro_export]
macro_rules! implement_tick2D {
    ($name:ident,  $py_constructor:expr) => {
        paste::paste! {

            #[pymethods]
            impl $name {

#[doc = 
"
Add a function to this object, which will automatically be executed each frame.
The function must take the object it is attatched to as an argument.
If a tick function is already assigned, this call will overwrite the existing function.

Example:
 
```
...# arguments from outside the scope may be included.
>>>delta_time = 0
>>>def update" $name "(obj: " $name "):
...    obj.rot += Vec2.splat(0.2*delta_time)
...
>>>my" $name " = " $py_constructor "
>>>my" $name ".tick(update" $name ")
...
>>>while True:
...    # dt would have to get updated each frame.
...    delta_time = get_delta_time()
...
...    #'next_frame' runs the update function for every object.
...    next_frame()
```
"
]
                #[pyo3(signature = (function))]
                pub fn tick(slf: Bound<'_, Self>, function: Bound<'_,PyAny>)-> PyResult<()>{

                    if !function.is_callable(){
                        return Err(PyRuntimeError::new_err(format!("Attatched object {:?} is not callable.",function)));
                    }

                    let mut self_ = slf.borrow_mut();
                    if let Some(key) = self_.function_key{
                        crate::py_abstractions::structs::ThreeDObjects::ObjectFunStorage::remove_function(key);
                    }

                    let func_persistent = function.unbind();
                    let obj  = slf.into_any();


                    self_.function_key = Some(
                        crate::py_abstractions::structs::ThreeDObjects::ObjectFunStorage::add_function(&obj, func_persistent)?
                    );

                    Ok(())
                }

                /// returns if the object has a registered tick function
                pub fn has_tick(&self)-> bool {
                    if let Some(key) = self.function_key{
                        true
                    } else {
                        false
                    }
                }


                /// Removes any assigned tick-function from this object.
                /// If the object does not have a tick function, this will do nothing.
                pub fn remove_tick(&mut self)-> PyResult<()>{

                    let key = match self.function_key{
                        None => { 
                            return Ok(());
                        },
                        Some(key)=> { key },
                    };
                    crate::py_abstractions::structs::ThreeDObjects::ObjectFunStorage::remove_function(key);
                    self.function_key  = None;
                    Ok(())
                }
            }
        }
    

    };
}






#[macro_export]
macro_rules! implement_magic_methods2D {
    ($name:ident) => {
        paste::paste! {
            use pyo3::types::{PyBytes, PyTuple};
            #[pymethods]
            impl $name {

                fn __repr__(&self) -> String {
                    use crate::py_abstractions::structs::TwoDObjects::TwoDObjectMacros::ShapeDebug;
                    format!("{}({}, has_tick={})", 
                        stringify!($name), 
                        <Self as ShapeDebug>::fmt_fields(self), 
                        self.function_key.is_some()
                    )
                }


            }
        }

    };
}



pub trait ShapeDebug {
    fn fmt_fields(&self) -> String;
}
impl ShapeDebug for Rectangle {
    fn fmt_fields(&self) -> String {

        let t = if let Some(tex) = &self.texture{ &format!("{}", tex.texture.ptr_address()) } else{ "None" };

        format!("position={:?}, scale={:?}, color={:?}, texture={}", self.position, self.scale, self.color, t)
    }
}

impl ShapeDebug for Circle {
    fn fmt_fields(&self) -> String {
        let t = if let Some(tex) = &self.texture{ &format!("{}", tex.texture.ptr_address()) } else{ "None" };
        format!("position={:?}, radius={:?},  sides={:?}, color={:?}, texture={}", self.position, self.radius, self.sides, self.color, t)
    }
}