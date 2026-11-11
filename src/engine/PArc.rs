use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use std::fmt::{Debug, Formatter};

/// Pythonic Arc.
/// this Wrapper ensures the object gets dropped on the macroquad thread.
/// This is mostly useful for anything that implements 'Texture2D', since it has an
/// unsafe destructor that panics if run on the python thread.
///
/// When the last handle is dropped, the bare `T` is taken out of the Arc and
/// sent to the macroquad thread, which drops it there.
pub struct PArc<T: Send + Sync + 'static>{
    item: ManuallyDrop<Arc<T>>,
}

impl<T: Send + Sync + 'static> PArc<T>{
    pub fn new(item: T)-> Self{
        PArc { item: ManuallyDrop::new(Arc::new(item)) }
    }

    pub fn ptr_address(&self) -> usize {
        Arc::as_ptr(&self.item) as usize
    }
}

impl<T: Send + Sync + 'static> Deref for PArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T: Send + Sync + 'static> DerefMut for PArc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::get_mut(&mut self.item).expect("Cannot get mutable reference: multiple strong references exist")
    }
}

impl<T: Send + Sync + 'static> Drop for PArc<T> {
    fn drop(&mut self) {

        let arc = unsafe { ManuallyDrop::take(&mut self.item) };

        if let Some(inner) = Arc::into_inner(arc) {
            COMMAND_QUEUE.push(Command::DropThisItem(Box::new(inner)));
        }
    }
}


impl<T: Send + Sync + 'static> Clone for PArc<T> {
    fn clone(&self) -> Self {
        PArc { item: ManuallyDrop::new(Arc::clone(&self.item)) }
    }
}


impl<T: Send + Sync + 'static + Debug> Debug for PArc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (**self.item).fmt(f)
    }
}


impl<T: Send + Sync + 'static + PartialEq> PartialEq for PArc<T> {
    fn eq(&self, other: &Self) -> bool {
        **self.item == **other.item
    }
}
