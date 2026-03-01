use macroquad::{prelude::{self as mq,}, texture::Texture2D};
use glam::{Vec3A, Mat3A, Quat, EulerRot};

#[derive( Debug, Clone)]
pub struct Pill{
    
    pub scale: mq::Vec3,
    pub position: mq::Vec3,
    pub rotation: mq::Vec3,
    pub color: mq::Color,

    pub mesh: PillMesh,
    pub draw_each_frame: bool,

}

impl Pill{
    pub fn new(size: mq::Vec3, position: mq::Vec3, rotation: mq::Vec3, color: mq::Color, texture: Option<Texture2D>)-> Pill{
        let mesh: PillMesh = PillMesh::new(size, position, rotation, texture, color, 20);

        Pill { scale: size,position,rotation,color,  mesh,draw_each_frame: true}
    }
    pub fn draw(&self, gl: &mut macroquad::prelude::QuadGl ){
        gl.texture(self.mesh.texture.as_ref());
        gl.geometry(&self.mesh.vertices, &self.mesh.indices);
    
    }
}


#[derive(Clone, Debug)]
pub struct PillMesh {
    pub vertices: Vec<mq::Vertex>,
    pub indices: Vec<u16>,
    pub texture: Option<mq::Texture2D>,
}

impl PillMesh {
    pub fn new(
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        texture: Option<mq::Texture2D>,
        color: mq::Color,
        cuts: usize, // Resolution for both rings and sectors
    ) -> Self {
        use mq::{Mat4, Vec3, Vertex};

        // Dimensions
        let radius = size.x * 0.5;
        // Total height is Y, cylinder height is remainder
        let cyl_height = (size.y - size.x).max(0.0);
        let half_cyl = cyl_height * 0.5;

        // Transform setup
        let rot_mat = Mat4::from_euler(mq::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
        let transform = Mat4::from_translation(position) * rot_mat;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let sectors = cuts.max(3);
        let hemi_rings = cuts.max(2) / 2;

        // We generate the pill in two loops (Bottom Hemi -> Top Hemi)
        // This creates two "equator" rings separated by the cylinder height.
        // The index generation logic below stitches them together.
        for part in 0..2 {
            let is_top = part == 1;
            let y_offset = if is_top { half_cyl } else { -half_cyl };
            let start_angle = if is_top { 0.0 } else { -std::f32::consts::FRAC_PI_2 };

            for r in 0..=hemi_rings {
                let r_ratio = r as f32 / hemi_rings as f32;
                // Angle: -PI/2 -> 0 (Bottom), 0 -> PI/2 (Top)
                let phi = start_angle + (r_ratio * std::f32::consts::FRAC_PI_2);

                let y_sphere = phi.sin();
                let xz_scale = phi.cos();

                let local_y = (y_sphere * radius) + y_offset;

                for s in 0..=sectors {
                    let s_ratio = s as f32 / sectors as f32;
                    let theta = s_ratio * std::f32::consts::TAU;

                    let x = theta.cos() * xz_scale * radius;
                    let z = theta.sin() * xz_scale * radius;

                    // Apply Transform
                    let raw_pos = Vec3::new(x, local_y, z);
                    let pos = (transform * raw_pos.extend(1.0)).truncate();

                    // Normal (based on sphere curvature, ignoring cylinder offset)
                    let n_raw = mq::vec4(theta.cos() * xz_scale, y_sphere, theta.sin() * xz_scale, 0.0);
                    let normal = rot_mat * n_raw;

                    // UV Mapping (V based on total height)
                    let v_map = (raw_pos.y + size.y * 0.5) / size.y;

                    vertices.push(Vertex {
                        position: pos,
                        uv: mq::vec2(s_ratio, 1.0 - v_map),
                        color: color.into(),
                        normal,
                    });
                }
            }
        }

        // Generate Indices
        let vertices_per_ring = sectors + 1;
        let total_rings = (hemi_rings + 1) * 2;

        for r in 0..(total_rings - 1) {
            for s in 0..sectors {
                let curr = (r * vertices_per_ring) + s;
                let next = curr + 1;
                let below = ((r + 1) * vertices_per_ring) + s;
                let below_next = below + 1;

                // Triangle 1
                indices.push(curr as u16);
                indices.push(below as u16);
                indices.push(next as u16);

                // Triangle 2
                indices.push(next as u16);
                indices.push(below as u16);
                indices.push(below_next as u16);
            }
        }

        Self {
            vertices,
            indices,
            texture,
        }
    }

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

    pub fn recalculate_rot(&mut self, pivot: mq::Vec3, old_rot: mq::Vec3, new_rot: mq::Vec3) {
        let pivot_simd = Vec3A::from(pivot);

        let q_old = Quat::from_euler(EulerRot::XYZ, old_rot.x, old_rot.y, old_rot.z);
        let q_new = Quat::from_euler(EulerRot::XYZ, new_rot.x, new_rot.y, new_rot.z);
        let q_delta = q_new * q_old.inverse();
        let rot_matrix = Mat3A::from_quat(q_delta);

        for vertex in self.vertices.iter_mut() {
            // Pos
            let pos = Vec3A::from(vertex.position);
            let local = pos - pivot_simd;
            let rotated = rot_matrix * local;
            vertex.position = glam::Vec3::from(pivot_simd + rotated);

            // Normal
            let norm = Vec3A::new(vertex.normal.x, vertex.normal.y, vertex.normal.z);
            let rot_norm = rot_matrix * norm;
            vertex.normal.x = rot_norm.x;
            vertex.normal.y = rot_norm.y;
            vertex.normal.z = rot_norm.z;
        }
    }

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