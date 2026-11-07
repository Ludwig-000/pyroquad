use macroquad::prelude as mq;
use rapier3d::na::SimdComplexField;


pub fn draw_cube_wires(position: mq::Vec3, size: mq::Vec3, rotation: mq::Vec3, color: mq::Color){
    let (x, y, z) = (position.x, position.y, position.z);
    let (width, height, length) = (size.x, size.y, size.z);
    let (hw, hh, hl) = (width / 2., height / 2., length / 2.);



    let (xsin,xcos) = rotation.x.simd_sin_cos();
    let (ysin,ycos) = rotation.y.simd_sin_cos();
    let (zsin,zcos) = rotation.z.simd_sin_cos();
    // vectors to corners, starting at 0,0,0

    let mut v1 = mq::vec3(hw, -hh, hl);

    // rotated by 3 axes:
    v1.x *= (ysin * zcos);
    

    /// rotating
    todo!()

}