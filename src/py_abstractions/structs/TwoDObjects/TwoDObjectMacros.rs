



#[macro_export]
macro_rules! implement_Drop2D {
    ($name:ident) => {
        paste::paste! {
            impl Drop for $name {
                fn drop(&mut self) {
                    // function storage MUST be cleaned first, since a function inside fun-storage may rely on the object still living.
                    if let Some(key) = self.function_key{
                        let mut storage = ObjectFunctionStorage::get_fun_storage();
                        storage.remove(key);
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

            #[gen_stub_pymethods]
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

                    let mut storage = ObjectFunctionStorage::get_fun_storage();
                    let mut self_ = slf.borrow_mut();
                    if let Some(key) = self_.function_key{
                        storage.remove(key);
                    }

                    let func_persistent = function.unbind();
                    let obj  = slf.into_any();

                    self_.function_key = Some(storage.add(obj, func_persistent));

                    Ok(())
                }


                /// Removes any assigned tick-function from this object.
                /// If the object does not have a tick function, this will do nothing.
                pub fn remove_tick(&mut self)-> PyResult<()>{

                    let mut storage = ObjectFunctionStorage::get_fun_storage();
                    let key = match self.function_key{
                        None => { 
                            return Ok(());
                        },
                        Some(key)=> { key },
                    };
                    storage.remove(key);
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

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {

                fn __copy__(&self) -> Self {
                    self.clone()
                }

                fn __deepcopy__(&self, _memo: &Bound<'_, PyDict>) -> Self {
                    self.clone()
                }

            }
        }

    };
}
