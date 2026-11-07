use macroquad::{prelude as mq, window::get_internal_gl};
use rapier3d::na::SimdComplexField;


pub fn draw_cube_wires(position: mq::Vec3, size: mq::Vec3, rotation: mq::Vec3, color: mq::Color) {
    let (hw, hh, hl) = (size.x / 2., size.y / 2., size.z / 2.);

    let (xsin, xcos) = rotation.x.sin_cos();
    let (ysin, ycos) = rotation.y.sin_cos();
    let (zsin, zcos) = rotation.z.sin_cos();

    let mut corners = [
        // bottom ring
        mq::vec3(-hw, -hh,  hl),
        mq::vec3( hw, -hh,  hl),
        mq::vec3( hw, -hh, -hl),
        mq::vec3(-hw, -hh, -hl),
        // top ring
        mq::vec3(-hw,  hh,  hl),
        mq::vec3( hw,  hh,  hl),
        mq::vec3( hw,  hh, -hl),
        mq::vec3(-hw,  hh, -hl),
    ];

    for c in corners.iter_mut() {
        let (y, z) = (c.y * xcos - c.z * xsin, c.y * xsin + c.z * xcos);
        c.y = y;
        c.z = z;

        let (z, x) = (c.z * ycos - c.x * ysin, c.z * ysin + c.x * ycos);
        c.z = z;
        c.x = x;

        let (x, y) = (c.x * zcos - c.y * zsin, c.x * zsin + c.y * zcos);
        c.x = x;
        c.y = y;

        *c += position;
    }

    let conv_color: [u8; 4] = color.into();
    let vertices = corners.map(|v| mq::Vertex {
        position: v,
        uv: mq::Vec2::ZERO,
        color: conv_color,
        normal: mq::Vec4::ZERO,
    });

    let indices: [u16; 24] = [
        0, 1, 1, 2, 2, 3, 3, 0,
        4, 5, 5, 6, 6, 7, 7, 4,
        0, 4, 1, 5, 2, 6, 3, 7,
    ];

    unsafe {
        let gl = get_internal_gl().quad_gl;
        gl.draw_mode(mq::DrawMode::Lines);
        gl.texture(None);
        gl.geometry(&vertices, &indices);
    }
}