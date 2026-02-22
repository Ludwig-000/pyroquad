use macroquad::{color::Color, prelude::{self as mq,}};
use glam::{Vec3A, Mat3A, Quat, EulerRot};

// Global constants for mesh resolution
const SPHERE_STACKS: usize = 16;
const SPHERE_SLICES: usize = 16;
const VERTEX_COUNT: usize = (SPHERE_STACKS + 1) * (SPHERE_SLICES + 1);
const INDEX_COUNT: usize = SPHERE_STACKS * SPHERE_SLICES * 6;



#[derive(Debug, Clone)]
pub struct Sphere {
    pub scale: mq::Vec3,
    pub position: mq::Vec3,
    pub rotation: mq::Vec3,
    pub color: mq::Color,

    pub mesh: SphereMesh,
}

impl Sphere {
    pub fn new(size: mq::Vec3, position: mq::Vec3, rotation: mq::Vec3, color: mq::Color, texture: Option<mq::Texture2D>) -> Sphere {
        let mesh: SphereMesh = SphereMesh::new(size, position, rotation, texture, color);
        Sphere { scale: size, position, rotation, color, mesh }
    }

    pub fn draw(&self, gl: &mut macroquad::prelude::QuadGl) {
        gl.texture(self.mesh.texture.as_ref());
        gl.geometry(&self.mesh.vertices, &self.mesh.indices);
    }
}

#[derive(Clone, Debug)]
pub struct SphereMesh {
    pub vertices: [macroquad::prelude::Vertex; VERTEX_COUNT],
    pub indices: [u16; INDEX_COUNT],
    pub texture: Option<mq::Texture2D>,
}

impl SphereMesh {
    pub fn new(
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        texture: Option<mq::Texture2D>,
        color: mq::Color,
    ) -> Self {
        use mq::{Mat4, Vec3, Vertex};
        use std::f32::consts::PI;

        let rot_mat = Mat4::from_euler(mq::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
        let transform = Mat4::from_translation(position) * rot_mat;

        let mut vertices = [Vertex {
            position: mq::Vec3::ZERO,
            uv: mq::Vec2::ZERO,
            color: color.into(),
            normal: mq::Vec4::ZERO,
        }; VERTEX_COUNT];

        let mut indices = [0u16; INDEX_COUNT];

        let mut v_idx = 0;
        for i in 0..=SPHERE_STACKS {
            let v = i as f32 / SPHERE_STACKS as f32; // 0.0 to 1.0
            let phi = v * PI; // 0 to Pi (Top to Bottom)

            for j in 0..=SPHERE_SLICES {
                let u = j as f32 / SPHERE_SLICES as f32;
                let theta = u * PI * 2.0;


                let x = theta.sin() * phi.sin();
                let y = phi.cos();
                let z = theta.cos() * phi.sin();

                let normal_vec = Vec3::new(x, y, z);
                
                let rotated_normal = rot_mat.transform_vector3(normal_vec);

                let local_pos = normal_vec * (size * 0.5);

                let world_pos = transform.transform_point3(local_pos);

                vertices[v_idx] = Vertex {
                    position: world_pos,
                    uv: mq::vec2(u, v),
                    color: color.into(),
                    normal: mq::vec4(rotated_normal.x, rotated_normal.y, rotated_normal.z, 0.0),
                };

                v_idx += 1;
            }
        }

        let mut i_idx = 0;
        let stride = (SPHERE_SLICES + 1) as u16;

        for i in 0..SPHERE_STACKS as u16 {
            for j in 0..SPHERE_SLICES as u16 {
                let curr = i * stride + j;
                let next = curr + stride;

                indices[i_idx] = curr;
                indices[i_idx + 1] = curr + 1;
                indices[i_idx + 2] = next;

                indices[i_idx + 3] = next;
                indices[i_idx + 4] = curr + 1;
                indices[i_idx + 5] = next + 1;

                i_idx += 6;
            }
        }

        Self {
            vertices,
            indices,
            texture,
        }
    }

    pub fn recalculate_pos(&mut self, old_pos: mq::Vec3, new_pos: mq::Vec3) {
        let old = Vec3A::from(glam::Vec3::from(old_pos));
        let new = Vec3A::from(glam::Vec3::from(new_pos));
        let delta = new - old;

        for vertex in self.vertices.iter_mut() {
            let mut pos = Vec3A::from(glam::Vec3::from(vertex.position));
            pos += delta;
            vertex.position = glam::Vec3::from(pos).into();
        }
    }

    pub fn recalculate_rot(&mut self, pivot: mq::Vec3, old_rot: mq::Vec3, new_rot: mq::Vec3) {
        let pivot_simd = Vec3A::from(glam::Vec3::from(pivot));

        let q_old = Quat::from_euler(EulerRot::XYZ, old_rot.x, old_rot.y, old_rot.z);
        let q_new = Quat::from_euler(EulerRot::XYZ, new_rot.x, new_rot.y, new_rot.z);

        let q_delta = q_new * q_old.inverse();

        let rot_matrix = Mat3A::from_quat(q_delta);

        for vertex in self.vertices.iter_mut() {
            let pos = Vec3A::from(glam::Vec3::from(vertex.position));
            let local = pos - pivot_simd;
            let rotated = rot_matrix * local;
            vertex.position = glam::Vec3::from(pivot_simd + rotated).into();

            let norm = Vec3A::new(vertex.normal.x, vertex.normal.y, vertex.normal.z);
            let rot_norm = rot_matrix * norm;

            vertex.normal.x = rot_norm.x;
            vertex.normal.y = rot_norm.y;
            vertex.normal.z = rot_norm.z;
        }
    }

    pub fn recalculate_scale(&mut self, pivot: mq::Vec3, old_scale: mq::Vec3, new_scale: mq::Vec3) {
        let pivot_simd = Vec3A::from(glam::Vec3::from(pivot));
        let old_s = Vec3A::from(glam::Vec3::from(old_scale));
        let new_s = Vec3A::from(glam::Vec3::from(new_scale));

        let ratio = new_s / (old_s + Vec3A::splat(0.0001));

        for vertex in self.vertices.iter_mut() {
            let pos = Vec3A::from(glam::Vec3::from(vertex.position));

            let offset = pos - pivot_simd;
            let final_pos = pivot_simd + (offset * ratio);

            vertex.position = glam::Vec3::from(final_pos).into();
        }
    }
}