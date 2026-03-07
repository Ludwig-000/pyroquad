use macroquad::prelude as mq;

#[derive(Clone,Copy,PartialEq)]
pub struct ThreeDObjCache{
    // an object's data can only be cached, if it cannot be influenced by anything else.
    // F.E.: gravity.

    pub position: mq::Vec3,
    pub rotation: mq::Vec3,
    pub scale: mq::Vec3,
    pub color: mq::Color,

}
impl ThreeDObjCache{
    pub fn new(location: mq::Vec3, rotation: mq::Vec3, scale: mq::Vec3, color: mq::Color)-> Option<ThreeDObjCache>{
        Some (ThreeDObjCache{
            position: location,
            rotation,
            scale,
            color,
        })
    }

}