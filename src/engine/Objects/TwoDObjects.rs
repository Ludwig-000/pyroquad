use crate::py_abstractions::structs::TwoDObjects::{Circle::Circle, Rectangle::Rectangle};
use macroquad::prelude as mq;




pub fn draw_circle(circle: &Circle) {
    let color = [(circle.color.r*255.) as u8,
     (circle.color.g*255.) as u8,
     (circle.color.b*255.) as u8,
     (circle.color.a*255.) as u8];

    // 1. Determine number of sides (clamp to minimum 3)
    let sides = circle.sides.max(3) as usize;
    
    let mut vertices = Vec::with_capacity(sides + 1);
    let mut indices = Vec::with_capacity(sides * 3);

    let (rot_sin, rot_cos) = circle.rotation.sin_cos();
    
    vertices.push(mq::Vertex {
        position: mq::vec3(circle.position.x, circle.position.y, 0.0),
        uv: mq::vec2(0.5, 0.5), 
        color, 
        normal: glam::Vec4::ZERO
    });

    for i in 0..sides {
        let local_angle = (i as f32 / sides as f32) * std::f32::consts::TAU;

        let rx = local_angle.cos();
        let ry = local_angle.sin();

        let uv = mq::vec2(0.5 + rx * 0.5, 0.5 + ry * 0.5);
        let rotated_x = rx * rot_cos - ry * rot_sin;
        let rotated_y = rx * rot_sin + ry * rot_cos;

        let pos = mq::vec3(
            circle.position.x + rotated_x * circle.radius,
            circle.position.y + rotated_y * circle.radius,
            0.0
        );

        vertices.push(mq::Vertex {
            position: pos,
            uv,
            color,
            normal: glam::Vec4::ZERO
        });
    }

    

    for i in 0..sides {
        let center = 0;
        let current = (i + 1) as u16;
        let next = if i + 1 == sides { 1 } else { (i + 2) as u16 };

        indices.push(center);
        indices.push(current);
        indices.push(next);
    }

    let tex_ref = circle.texture.as_ref().map(|t| &*t.texture);
    draw_2D_geomentry(&vertices, &indices, tex_ref);
}

pub fn draw_rect(rect: &Rectangle) {
    let w = rect.scale.x;
    let h = rect.scale.y;
    let x = rect.position.x;
    let y = rect.position.y;
    let rot = rect.rotation;
    let color = [(rect.color.r*255.) as u8,
     (rect.color.g*255.) as u8,
     (rect.color.b*255.) as u8,
     (rect.color.a*255.) as u8];

    
    let (sin, cos) = rot.sin_cos();

    let transform = |lx: f32, ly: f32| -> mq::Vec3 {
        let rx = lx * cos - ly * sin;
        let ry = lx * sin + ly * cos;
        mq::vec3(x + rx, y + ry, 0.0)
    };

    // Define vertices relative to center (0,0)
    // TL, TR, BR, BL
    let v_tl = transform(-w * 0.5, -h * 0.5);
    let v_tr = transform( w * 0.5, -h * 0.5);
    let v_br = transform( w * 0.5,  h * 0.5);
    let v_bl = transform(-w * 0.5,  h * 0.5);

    

    let vertices = [
        // Top Left (0)
        mq::Vertex { position: v_tl, uv: mq::vec2(0.0, 0.0), color,normal: glam::Vec4::ZERO},
        // Top Right (1)
        mq::Vertex { position: v_tr, uv: mq::vec2(1.0, 0.0), color,normal: glam::Vec4::ZERO },
        // Bottom Right (2)
        mq::Vertex { position: v_br, uv: mq::vec2(1.0, 1.0), color,normal: glam::Vec4::ZERO },
        // Bottom Left (3)
        mq::Vertex { position: v_bl, uv: mq::vec2(0.0, 1.0), color,normal: glam::Vec4::ZERO },
    ];

    // Standard Quad Indices (Counter-Clockwise)
    // Triangle 1: 0 -> 1 -> 2
    // Triangle 2: 0 -> 2 -> 3
    let indices = [0, 1, 2, 0, 2, 3];
    let tex_ref = rect.texture.as_ref().map(|t| &*t.texture);
    draw_2D_geomentry(&vertices, &indices, tex_ref);
}



fn draw_2D_geomentry(vertices: &[mq::Vertex], indices: &[u16], texture: Option<&mq::Texture2D>){

        let context = unsafe { mq::get_internal_gl().quad_gl };

        context.texture(texture);
        context.draw_mode(mq::DrawMode::Triangles);
        context.geometry(vertices, indices);
}