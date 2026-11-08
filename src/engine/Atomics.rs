use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use macroquad::prelude as mq;


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




pub struct AtomicVec2(AtomicU64);

impl AtomicVec2 {
    pub const fn new(x: f32, y: f32) -> AtomicVec2 {
        let l = x.to_bits() as u64;
        let r = y.to_bits() as u64;

        let num: u64 = r*(u32::MAX as u64 +1) + l;
        AtomicVec2(AtomicU64::new(num))
    }

    pub fn load(&self, order: Ordering) -> mq::Vec2{ 
        let bits = self.0.load(order);

        let l = bits as u32;
        let r = (bits >> 32) as u32;
        mq::Vec2 { 
            x: f32::from_bits(l as u32), 
            y: f32::from_bits(r as u32) 
        }
    }

    pub fn store(&self, v: mq::Vec2, order: Ordering){
        let l = v.x.to_bits() as u64;
        let r = v.y.to_bits() as u64;

        let num: u64 = r*(u32::MAX as u64 +1) + l;
        
        self.0.store(num, order);
    }
}