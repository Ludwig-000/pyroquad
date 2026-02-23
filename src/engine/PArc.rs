use std::ops::{Deref, DerefMut};
use std::thread;
use std::sync::Arc;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use std::fmt::{Debug, Formatter};

/// Pythonic Arc.
/// this Wrapper ensures the object gets dropped in the same Thread it was created in.
/// (kinda)
/// This is mostly useful for anything that implements 'Texture2D', since it has an
/// unsafe destructor that panics if run on the python thread.
pub struct PArc<T: Send + Sync + 'static>{
    item: Arc<T>,
    origin: thread::ThreadId,
}

impl<T: Send + Sync + 'static> PArc<T>{
    pub fn new(item: T)-> Self{
        let id: thread::ThreadId = thread::current().id();
        let item  =Arc::new(item);
        PArc {  item, origin: id, }
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

        let count = Arc::strong_count(&self.item);
        if  count > 1  {return}
        

        let id = thread::current().id();
        if id == self.origin {return}

        
        let cloned_arc  =self.item.clone();
        COMMAND_QUEUE.push( Command::DropThisItem(cloned_arc)       );
    }
}


impl<T: Send + Sync + 'static> Clone for PArc<T> {
    fn clone(&self) -> Self {
        PArc {
            item: self.item.clone(),
            origin: self.origin,
        }
    }
}


impl<T: Send + Sync + 'static + Debug> Debug for PArc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.item.fmt(f)
        
    }
}


impl<T: Send + Sync + 'static + PartialEq> PartialEq for PArc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}