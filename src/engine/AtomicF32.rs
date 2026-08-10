use std::sync::atomic::{AtomicU32, Ordering};



#[derive(Default)]
pub struct AtomicF32(AtomicU32);

impl AtomicF32 {
    pub const fn new(v: f32)-> AtomicF32{
        let u: u32 = v.to_bits();
        AtomicF32(AtomicU32::new(u))
    }
    pub fn load(&self, order: Ordering)-> f32{
        let bits = self.0.load(order);
        f32::from_bits(bits)
    }
    pub fn store(&self, v: f32, order: Ordering){
        let u: u32 = v.to_bits();
        self.0.store(u, order);
    }
}

impl From<f32> for AtomicF32 {
    fn from(v: f32) -> Self {
        Self::new(v)
    }
}