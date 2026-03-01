use macroquad::{prelude::{self as mq,}, texture::Texture2D};
use glam::{Vec3A, Mat3A, Quat, EulerRot};

#[derive( Debug, Clone)]
pub struct Cylinder{
    
    pub scale: mq::Vec3,
    pub position: mq::Vec3,
    pub rotation: mq::Vec3,
    pub color: mq::Color,

    pub mesh: CylinderMesh,
    pub draw_each_frame: bool,

}
impl Cylinder{
    pub fn new(size: mq::Vec3, position: mq::Vec3, rotation: mq::Vec3, color: mq::Color, texture: Option<Texture2D>)-> Cylinder{
        let mesh: CylinderMesh = CylinderMesh::new(size, position, rotation, texture, color,10);

        Cylinder { scale: size,position,rotation,color,  mesh, draw_each_frame: true }
    }

    pub fn draw(&self, gl: &mut macroquad::prelude::QuadGl ){
        gl.texture(self.mesh.texture.as_ref());
        gl.geometry(&self.mesh.vertices, &self.mesh.indices);
    
    }
}

#[derive(Clone, Debug)]
pub struct CylinderMesh {
    pub vertices: Vec<mq::Vertex>,
    pub indices: Vec<u16>,
    pub texture: Option<mq::Texture2D>,
}

impl CylinderMesh {
    pub fn new(
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        texture: Option<mq::Texture2D>,
        color: mq::Color,
        resolution: u32,
    ) -> Self {
        use mq::{Vec2, Vec3, Vec4, Vertex, Mat4};

        let res = resolution.max(3);
        let radius_x = size.x * 0.5;
        let radius_z = size.z * 0.5;
        let half_height = size.y * 0.5;

        // Reserve memory to avoid reallocations
        // Vertices: (res + 1) * 2 for sides + (res + 2) for top + (res + 2) for bottom
        let mut vertices = Vec::with_capacity(((res + 1) * 2 + (res + 2) * 2) as usize);
        let mut indices = Vec::with_capacity((res * 12) as usize);

        // --- 1. Generate Sides ---
        // We capture the starting index. Since this is the first operation, it is 0,
        // but calculating it is safer for refactoring.
        let side_start_idx = vertices.len() as u16; 
        
        for i in 0..=res {
            let theta = (i as f32 / res as f32) * std::f32::consts::TAU;
            let (sin_t, cos_t) = theta.sin_cos();

            let x = cos_t * radius_x;
            let z = sin_t * radius_z;
            let u = i as f32 / res as f32;

            let normal = Vec3::new(cos_t, 0.0, sin_t).normalize();

            // Top rim vertex
            vertices.push(Vertex {
                position: Vec3::new(x, half_height, z),
                uv: Vec2::new(u, 0.0),
                color: color.into(),
                normal: Vec4::new(normal.x, normal.y, normal.z, 0.0),
            });

            // Bottom rim vertex
            vertices.push(Vertex {
                position: Vec3::new(x, -half_height, z),
                uv: Vec2::new(u, 1.0),
                color: color.into(),
                normal: Vec4::new(normal.x, normal.y, normal.z, 0.0),
            });
        }

        // Indices for Sides
        for i in 0..res as u16 {
            let base = side_start_idx + i * 2;
            // Vertices are arranged: Top(i), Bot(i), Top(i+1), Bot(i+1)
            // Quad: TL -> BL -> TR, TR -> BL -> BR
            indices.push(base);     // Top Left
            indices.push(base + 1); // Bottom Left
            indices.push(base + 2); // Top Right

            indices.push(base + 2); // Top Right
            indices.push(base + 1); // Bottom Left
            indices.push(base + 3); // Bottom Right
        }

        // --- 2. Top Cap ---
        let top_center_idx = vertices.len() as u16;
        
        // Center Vertex
        vertices.push(Vertex {
            position: Vec3::new(0.0, half_height, 0.0),
            uv: Vec2::new(0.5, 0.5),
            color: color.into(),
            normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
        });

        // Rim Vertices
        for i in 0..=res {
            let theta = (i as f32 / res as f32) * std::f32::consts::TAU;
            let (sin_t, cos_t) = theta.sin_cos();
            
            vertices.push(Vertex {
                position: Vec3::new(cos_t * radius_x, half_height, sin_t * radius_z),
                uv: Vec2::new(cos_t * 0.5 + 0.5, sin_t * 0.5 + 0.5),
                color: color.into(),
                normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
            });
        }

        // Indices for Top Cap (CCW)
        for i in 0..res as u16 {
            indices.push(top_center_idx);
            indices.push(top_center_idx + 1 + i + 1); // Next
            indices.push(top_center_idx + 1 + i);     // Current
        }

        // --- 3. Bottom Cap ---
        let bot_center_idx = vertices.len() as u16;

        // Center Vertex
        vertices.push(Vertex {
            position: Vec3::new(0.0, -half_height, 0.0),
            uv: Vec2::new(0.5, 0.5),
            color: color.into(),
            normal: Vec4::new(0.0, -1.0, 0.0, 0.0),
        });

        // Rim Vertices
        for i in 0..=res {
            let theta = (i as f32 / res as f32) * std::f32::consts::TAU;
            let (sin_t, cos_t) = theta.sin_cos();

            vertices.push(Vertex {
                position: Vec3::new(cos_t * radius_x, -half_height, sin_t * radius_z),
                uv: Vec2::new(cos_t * 0.5 + 0.5, sin_t * 0.5 + 0.5),
                color: color.into(),
                normal: Vec4::new(0.0, -1.0, 0.0, 0.0),
            });
        }

        // Indices for Bottom Cap (CW from bottom view => CCW from outside)
        for i in 0..res as u16 {
            indices.push(bot_center_idx);
            indices.push(bot_center_idx + 1 + i);     // Current
            indices.push(bot_center_idx + 1 + i + 1); // Next
        }

        // --- 4. Apply Initial Transform ---
        let rot_mat = Mat4::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
        let transform = Mat4::from_translation(position) * rot_mat;

        for v in vertices.iter_mut() {
            // Apply Transform to Position
            let pos = transform.transform_point3(v.position);
            v.position = pos;

            // Apply Rotation to Normal (ignore translation)
            // transform_vector3 applies the upper-left 3x3 (scale + rotation)
            let n = mq::vec3(v.normal.x, v.normal.y, v.normal.z);
            let tn = transform.transform_vector3(n);
            v.normal = mq::vec4(tn.x, tn.y, tn.z, 0.0);
        }

        Self {
            vertices,
            indices,
            texture,
        }
    }

    /// 1. Recalculate mesh based on a change in position.
    pub fn recalculate_pos(&mut self, old_pos: mq::Vec3, new_pos: mq::Vec3) {
        let old = Vec3A::from(old_pos);
        let new = Vec3A::from(new_pos);
        let delta = new - old;

        for vertex in self.vertices.iter_mut() {
            let mut pos = Vec3A::from(vertex.position);
            pos += delta;
            vertex.position = glam::Vec3::from(pos);
        }
    }

    /// 2. Recalculate mesh based on a change in rotation.
    pub fn recalculate_rot(&mut self, pivot: mq::Vec3, old_rot: mq::Vec3, new_rot: mq::Vec3) {
        let pivot_simd = Vec3A::from(pivot);

        let q_old = Quat::from_euler(EulerRot::XYZ, old_rot.x, old_rot.y, old_rot.z);
        let q_new = Quat::from_euler(EulerRot::XYZ, new_rot.x, new_rot.y, new_rot.z);

        // Q_delta = Q_new * Q_old^(-1)
        let q_delta = q_new * q_old.inverse();

        let rot_matrix = Mat3A::from_quat(q_delta);

        for vertex in self.vertices.iter_mut() {
            // Rotate Position
            let pos = Vec3A::from(vertex.position);
            let local = pos - pivot_simd;
            let rotated = rot_matrix * local;
            vertex.position = glam::Vec3::from(pivot_simd + rotated);

            // Rotate Normal
            let norm = Vec3A::new(vertex.normal.x, vertex.normal.y, vertex.normal.z);
            let rot_norm = rot_matrix * norm;
            
            vertex.normal.x = rot_norm.x;
            vertex.normal.y = rot_norm.y;
            vertex.normal.z = rot_norm.z;
        }
    }

    /// 3. Recalculate mesh based on a change in scale.
    pub fn recalculate_scale(&mut self, pivot: mq::Vec3, old_scale: mq::Vec3, new_scale: mq::Vec3) {
        let pivot_simd = Vec3A::from(pivot);
        let old_s = Vec3A::from(old_scale);
        let new_s = Vec3A::from(new_scale);

        let ratio = new_s / old_s;

        for vertex in self.vertices.iter_mut() {
            let pos = Vec3A::from(vertex.position);
            
            let offset = pos - pivot_simd;
            let final_pos = pivot_simd + (offset * ratio);

            vertex.position = glam::Vec3::from(final_pos);
        }
    }
}