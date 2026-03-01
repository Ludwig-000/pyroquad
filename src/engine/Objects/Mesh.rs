use macroquad::{prelude as mq};
use glam::{Vec3A, Mat3A, Quat, EulerRot};
use gltf::mesh::util::ReadIndices;

pub struct Mesh{
    
    pub scale: mq::Vec3,
    pub position: mq::Vec3,
    pub rotation: mq::Vec3,
    pub color: mq::Color,

    pub mesh: mq::Mesh,
    pub draw_each_frame: bool,

}

///i do NOT KNOW why the FUCK mq::Mesh does not have CLONE BUT I AM DOING IT MYSELF 
impl Clone for Mesh{
    fn clone(&self) -> Self {
        Mesh { 
            scale: self.scale,
            position: self.position, 
            rotation: self.rotation, 
            color: self.color, 
            mesh: mq::Mesh { 
                vertices: self.mesh.vertices.clone(), 
                indices: self.mesh.indices.clone(), 
                texture: self.mesh.texture.clone()
            },
            draw_each_frame: self.draw_each_frame
        }
    }
}
impl Mesh{
    pub fn draw(&self, gl: &mut macroquad::prelude::QuadGl ){
        gl.texture(self.mesh.texture.as_ref());
        gl.geometry(&self.mesh.vertices, &self.mesh.indices);
    }

    pub fn load_from_gltf(data: &[u8], texture: Option<mq::Texture2D>) -> Result<Self, gltf::Error> {
        let (document, buffers, _) = gltf::import_slice(data)?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        // Default transforms
        let mut final_pos = mq::vec3(0.0, 0.0, 0.0);
        let mut final_scale = mq::vec3(1.0, 1.0, 1.0);
        let mut final_rot = mq::vec3(0.0, 0.0, 0.0);

        // 1. ITERATE NODES (Not Meshes) to get Position/Rotation/Scale
        for node in document.nodes() {
            if let Some(mesh) = node.mesh() {
                
                // Get the transform of this node (Location from Blender)
                let (trans, rot, scale) = node.transform().decomposed();
                
                // Convert to Macroquad/Glam types to store in your struct
                final_pos = mq::vec3(trans[0], trans[1], trans[2]);
                final_scale = mq::vec3(scale[0], scale[1], scale[2]);
                
                // Convert Quaternion to Euler (if you really need Euler for your struct)
                // Note: It is often better to store the Quat, but here is the Euler conversion:
                let q = Quat::from_array(rot);
                let (x, y, z) = q.to_euler(glam::EulerRot::XYZ);
                final_rot = mq::vec3(x, y, z);

                for primitive in mesh.primitives() {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                    // --- MATERIAL COLOR FIX ---
                    // Get the base color from the material (The BSDF Color)
                    let material = primitive.material();
                    let pbr = material.pbr_metallic_roughness();
                    let base_color_factor = pbr.base_color_factor(); // [r, g, b, a]
                    let material_color = mq::Color::from_vec(mq::vec4(
                        base_color_factor[0],
                        base_color_factor[1],
                        base_color_factor[2],
                        base_color_factor[3],
                    ));

                    let positions_reader = match reader.read_positions() {
                        Some(iter) => iter,
                        None => continue,
                    };

                    let positions: Vec<_> = positions_reader.map(mq::Vec3::from).collect();

                    let normals: Vec<_> = reader
                        .read_normals()
                        .map(|n| n.map(mq::Vec3::from).collect())
                        .unwrap_or_else(|| vec![mq::vec3(0.0, 1.0, 0.0); positions.len()]);

                    let tex_coords: Vec<_> = reader
                        .read_tex_coords(0)
                        .map(|uv| uv.into_f32().map(|v| mq::vec2(v[0], v[1])).collect())
                        .unwrap_or_else(|| vec![mq::vec2(0.0, 0.0); positions.len()]);

                    // --- COLOR LOGIC UPDATE ---
                    // Try to read vertex colors. If they don't exist, fill with MATERIAL color.
                    let colors: Vec<_> = reader
                        .read_colors(0)
                        .map(|c| {
                            c.into_rgba_f32()
                                .map(|rgba| mq::Color::from_vec(mq::vec4(rgba[0], rgba[1], rgba[2], rgba[3])))
                                .collect()
                        })
                        .unwrap_or_else(|| vec![material_color; positions.len()]); // <--- Use material_color here

                    
                    // We need to offset the indices based on vertices we've already loaded
                    // (in case there are multiple primitives/meshes)
                    let vertex_start = vertices.len() as u16;

                    for i in 0..positions.len() {
                        vertices.push(mq::Vertex {
                            position: positions[i],
                            uv: tex_coords[i],
                            color: colors[i].into(),
                            normal: normals[i].extend(0.0),
                        });
                    }

                    if let Some(read_indices) = reader.read_indices() {
                        match read_indices {
                            ReadIndices::U16(iter) => indices.extend(iter.map(|i| i + vertex_start)),
                            ReadIndices::U32(iter) => indices.extend(iter.map(|i| (i as u16) + vertex_start)),
                            ReadIndices::U8(iter) => indices.extend(iter.map(|i| (i as u16) + vertex_start)),
                        }
                    }
                }
            }
        }

        Ok(Self {
            scale: final_scale,
            position: final_pos,
            rotation: final_rot,
            color: mq::WHITE,
            mesh: mq::Mesh {
                vertices,
                indices,
                texture,
            },
            draw_each_frame: true,
        })
    }

    pub fn  recalculate_pos(&mut self, old_pos: mq::Vec3, new_pos: mq::Vec3) {
        let old = Vec3A::from(old_pos);
        let new = Vec3A::from(new_pos);
        let delta = new - old;

        for vertex in self.mesh.vertices.iter_mut() {
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

        for vertex in self.mesh.vertices.iter_mut() {
            let pos = Vec3A::from(vertex.position);
            let local = pos - pivot_simd;
            let rotated = rot_matrix * local;
            vertex.position = glam::Vec3::from(pivot_simd + rotated);

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

        for vertex in self.mesh.vertices.iter_mut() {
            let pos = Vec3A::from(vertex.position);
            
            let offset = pos - pivot_simd;
            let final_pos = pivot_simd + (offset * ratio);

            vertex.position = glam::Vec3::from(final_pos);
        }
    }
}