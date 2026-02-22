



#[macro_export]
macro_rules! implement_basic_magic_methods3D {
    ($name:ident) => {
        paste::paste! {

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {

                #[doc = "Equality for " $name " is based on unique ID."]
                fn __eq__(&self, other: &Self) -> bool {
                    self.key == other.key
                }
            
                #[doc = "Hash for " $name " is based on unique ID, not it's fields."]
                fn __hash__(&self) -> u64 {
                    let mut s = std::collections::hash_map::DefaultHasher::new();
                    self.key.hash(&mut s);
                    s.finish()
                }
            
                fn __repr__(&self) -> String {
                    let pos = self.pos();
                    let rot = self.rot();
                    let scale  = self.scale();
                    let has_tick_function = if self.function_key == None {false} else {true};
                    format!("{}(position={:?}, rotation={:?}, scale={:?}, has_tick_function={})", 
                        stringify!($name), pos, rot, scale, has_tick_function)
                }
            
                fn __str__(&self)-> PyResult<String>{
                    let pos = self.pos()?;
                    Ok(format!("{} at ({:.2}, {:.2}, {:.2})", 
                        stringify!($name), pos.x, pos.y, pos.z))
                }
            }
        }

    };
}







#[macro_export]
macro_rules! implement_basic_getter_methods3D {
    ($name:ident) => {
        paste::paste! {

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {
#[doc = 
"
Accesses the position of the given object.
Note that individual values of an object can NOT be changed via:
```
>>>" $name ".pos.x += 1
```
since object.pos returns a copy of its position, one has to write:
```
>>>" $name ".pos += Vec3(1, 0, 0)
```
"
]
                #[getter]
                fn pos(&self) -> PyResult<Vec3> {
                    if let Some(cache) = self.cache {
                        return Ok(cache.position.into())
                    }
            
                    let (sender, receiver) = PChannel::sync_channel(1);
                    let command = Command::GetObjectPos { key: self.key, sender: sender };
                    COMMAND_QUEUE.push(command);
                    Ok(receiver.recv()?.into())
                }

#[doc = 
"
Accesses the rotation of the given object.
Note that individual values of an object can NOT be changed via:
```
>>>" $name ".rot.x += 1
```
since " $name ".rot returns a copy of its rotation, one has to write:
```
>>>" $name ".rot += Vec3(1, 0, 0)
```
"
]
                #[getter]
                fn rot(&self) -> PyResult<Vec3> {
                    if let Some(cache) = self.cache {
                        return Ok(cache.rotation.into())
                    }
            
                    let (sender, receiver) = PChannel::sync_channel(1);
                    let command = Command::GetObjectRotation{ key: self.key, sender: sender };
                    COMMAND_QUEUE.push(command);
                    Ok(receiver.recv()?.into())
                }

#[doc = 
"
Accesses the scale of the given object.
Note that individual values of an object can NOT be changed via:
```
>>>" $name ".scale.x += 1
```
since " $name ".scale returns a copy of its scale, one has to write:
```
>>>" $name ".scale += Vec3(1, 0, 0)
```
"
]
                #[getter]
                fn scale(&self) -> PyResult<Vec3> {
                    if let Some(cache) = self.cache {
                        return Ok(cache.scale.into())
                    }
            
                    let (sender, receiver) = PChannel::sync_channel(1);
                    let command = Command::GetObjectScale { key: self.key, sender: sender };
                    COMMAND_QUEUE.push(command);
                    Ok(receiver.recv()?.into())
                }



            }
        }

    };
}









#[macro_export]
macro_rules! implement_basic_setter_methods3D {
    ($name:ident) => {paste::paste! {

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {
                
                #[setter]
                fn set_scale(&mut self, value: Vec3) {
                    if let Some(cache) = &mut self.cache {
                        cache.scale = value.into();
                    }
                    let command = Command::SetObjectScale { key: self.key, scale: value.into() };
                    COMMAND_QUEUE.push(command);
                }
                #[setter]
                fn set_pos(&mut self, value: Vec3) {
                    if let Some(cache) = &mut self.cache {
                        cache.position = value.into();
                    }
                    let command = Command::SetObjectPos { key: self.key, position: value.into() };
                    COMMAND_QUEUE.push(command);
                }
                #[setter]
                fn set_rot(&mut self, value: Vec3) {
                    if let Some(cache) = &mut self.cache {
                        cache.rotation = value.into();
                    }
                    let command = Command::SetObjectRotation { key: self.key, rotation: value.into() };
                    COMMAND_QUEUE.push(command);
                }
            }

    }};
}








#[macro_export]
macro_rules! implement_check_collision3D {
    ($name:ident) => {
        paste::paste! {

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {

#[doc = 
"
Returns any object, with active collision, that is either
intersected or inserted in the current object.
 
Example:
 
```
>>>big" $name ": " $name " = " $name "(pos=Vec3.splat(50))
>>>intersected: list[" $name "] = big" $name ".check_collision()
...
...# since the returned objects are references, we can edit them directly
...# without creating duplicates.
>>>for i in intersected:
...   i.pos = Vec3.ZERO()
```
"
]
                pub fn check_collision<'py>(&self, py: Python<'py> )-> PyResult<Vec<Bound<'py, PyAny>>>{

                    let (sender, receiver) = PChannel::sync_channel(1);
                    let command = Command::GetColissionObjects { key: self.key, sender };
                    COMMAND_QUEUE.push(command);
                    let res: Vec<std::sync::Arc<Py<PyWeakref>>> = receiver.recv()?;
            
                    // we filter map, since any weakref we hold may already be invalid.
                    Ok(res.into_iter().filter_map(|pyObj|{
            
                        let weak_py: &Py<PyWeakref> = &*pyObj;
            
                        let weak_bound: Bound<'py, PyWeakref> = weak_py.bind(py).clone();
            
                        let upgraded: Bound<'py, PyAny> = weak_bound.call0().ok()?;
            
                        if upgraded.is_none() {
                            None
                        } else {
                            Some(upgraded)
                        }
                    }).collect())
                }
            }
        }

    };
}



#[macro_export]
macro_rules! implement_tick3D {
    ($name:ident,  $py_constructor:expr) => {
        paste::paste! {

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {

#[doc = 
"
Add a function to this object, which will automatically be executed each frame.
The function must take the object it is attatched to as an argument.
 
Example:
 
```
...# arguments from outside the scope may be included.
>>>delta_time = 0
>>>def update" $name "(obj: " $name "):
...    obj.rot += Vec3.splat(0.2*delta_time)
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
            }
        }

    };
}




#[macro_export]
macro_rules! implement_remove_tick3D {
    ($name:ident) => {
        paste::paste! {

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {

                pub fn remove_tick(&mut self)-> PyResult<()>{

                    let mut storage = ObjectFunctionStorage::get_fun_storage();
                    let key = match self.function_key{
                        None => { 
                            return Err(
                                PyRuntimeError::new_err("No function found, that can be removed.")
                            );
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
macro_rules! implement_set_collider3D {
    ($name:ident) => {
        paste::paste! {

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {
                
                /// overwrites the current collider with the input option.
                pub fn set_collider(&self, collider_type: ColliderOptions){
                    todo!()
                }
            }
        }

    };
}

#[macro_export]
macro_rules! implement_Drop3D {
    ($name:ident) => {
        paste::paste! {
            impl Drop for $name {
                fn drop(&mut self) {
                    // function storage MUST be cleaned first, since a function inside fun-storage may rely on the object still living.
                    if let Some(key) = self.function_key{
                        let mut storage = ObjectFunctionStorage::get_fun_storage();
                        storage.remove(key);
                    }
                    COMMAND_QUEUE.push( Command::DeleteObject { key: self.key });
                }
            }
        }

    };
}



#[macro_export]
macro_rules! implement_manual_drawing_options3D {
    ($name:ident,  $py_constructor:expr) => {
        paste::paste! {

            #[gen_stub_pymethods]
            #[pymethods]
            impl $name {
                
#[doc = 
"
Decides wether to draw the " $name " during all subsequent 'draw_all_objects()'.
If set to false, this object will forever be invisible unless drawn manually, which will ignore this flag.

 
Example:
 
```
...# we flicker the " $name " every second frame.
>>>my" $name " = " $py_constructor "
>>>flag = True
>>>while True:
>>>     flag = not flag
>>>     my" $name ".set_draw_each_frame(flag)
```
"
]
                pub fn set_draw_each_frame(&self, drawing: bool){
                    todo!()
                }

#[doc = 
"
This function will draw the " $name " right now.
If 'set_draw_each_frame' is set to true,
this will still be drawn during 'draw_all_objects()'
 
Example:
 
```
...# we have turned an enemy invisible,
... #but want to reveal him if he has been spottet for a short duration
>>>enemy = " $py_constructor "
>>>while True:
>>>     
>>>     if enemy_used_invisibility:
>>>         enemy.set_draw_each_frame(False)
>>>     if is_spottet:
>>>         enemy.manually_draw_now()
```
"
]
                pub fn manually_draw_now(&self){
                    todo!()
                }
            }
        }

    };
}






