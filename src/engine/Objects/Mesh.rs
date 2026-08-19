use macroquad::{prelude as mq};
use glam::{Vec3A, Mat3A, Quat, EulerRot};
use gltf::mesh::util::ReadIndices;
use glam::{ Mat4, Mat3, Vec3};


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

        // 1. RESOLVE GLOBAL TRANSFORMS FOR ALL NODES
        // glTF nodes have hierarchies. We need to calculate the global transformation 
        // matrix for each node by multiplying it by its parent's matrix.
        let mut global_transforms = vec![Mat4::IDENTITY; document.nodes().count()];
        let mut stack = Vec::new();

        // Start with root nodes from the default scene
        if let Some(scene) = document.default_scene().or_else(|| document.scenes().next()) {
            for node in scene.nodes() {
                stack.push((node, Mat4::IDENTITY));
            }
        }

        // Traverse the tree iteratively
        while let Some((node, parent_transform)) = stack.pop() {
            // Get local transform (gltf matrix is column-major)
            let local_transform = Mat4::from_cols_array_2d(&node.transform().matrix());
            let global_transform = parent_transform * local_transform;
            
            global_transforms[node.index()] = global_transform;

            for child in node.children() {
                stack.push((child, global_transform));
            }
        }

        // 2. ITERATE NODES AND READ MESHES
        for node in document.nodes() {
            if let Some(mesh) = node.mesh() {
                
                // Get the baked global transform we calculated for this specific node
                let transform = global_transforms[node.index()];
                
                // To rotate normals correctly (especially if non-uniform scaling is used), 
                // we need the inverse-transpose of the 3x3 portion of the transform matrix.
                let normal_matrix = Mat3::from_mat4(transform).inverse().transpose();

                for primitive in mesh.primitives() {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                    // --- MATERIAL COLOR ---
                    let material = primitive.material();
                    let pbr = material.pbr_metallic_roughness();
                    let base_color_factor = pbr.base_color_factor();
                    let material_color = mq::Color::from_vec(mq::vec4(
                        base_color_factor[0],
                        base_color_factor[1],
                        base_color_factor[2],
                        base_color_factor[3],
                    ));

                    // --- READ RAW POSITIONS ---
                    let positions_reader = match reader.read_positions() {
                        Some(iter) => iter,
                        None => continue,
                    };
                    // Keep as raw arrays initially to easily convert to Glam vectors
                    let positions: Vec<[f32; 3]> = positions_reader.collect(); 

                    // --- READ RAW NORMALS ---
                    let normals: Vec<[f32; 3]> = reader
                        .read_normals()
                        .map(|n| n.collect())
                        .unwrap_or_else(|| vec![[0.0, 1.0, 0.0]; positions.len()]);

                    let tex_coords: Vec<_> = reader
                        .read_tex_coords(0)
                        .map(|uv| uv.into_f32().map(|v| mq::vec2(v[0], v[1])).collect())
                        .unwrap_or_else(|| vec![mq::vec2(0.0, 0.0); positions.len()]);

                    let colors: Vec<_> = reader
                        .read_colors(0)
                        .map(|c| {
                            c.into_rgba_f32()
                                .map(|rgba| mq::Color::from_vec(mq::vec4(rgba[0], rgba[1], rgba[2], rgba[3])))
                                .collect()
                        })
                        .unwrap_or_else(|| vec![material_color; positions.len()]);

                    let vertex_start = vertices.len() as u16;

                    // --- APPLY TRANSFORMS TO VERTICES ---
                    for i in 0..positions.len() {
                        // Position: Multiply local position by the global transformation matrix
                        let local_pos = Vec3::from_array(positions[i]);
                        let world_pos = transform.transform_point3(local_pos);

                        // Normal: Multiply local normal by the normal matrix
                        let local_normal = Vec3::from_array(normals[i]);
                        let world_normal = normal_matrix.mul_vec3(local_normal).normalize_or_zero();

                        vertices.push(mq::Vertex {
                            position: mq::vec3(world_pos.x, world_pos.y, world_pos.z),
                            uv: tex_coords[i],
                            color: colors[i].into(),
                            normal: mq::vec3(world_normal.x, world_normal.y, world_normal.z).extend(0.0),
                        });
                    }

                    // --- INDICES ---
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

        // We bake the nodes into the vertices, so the structural transform of the overall 
        // Macroquad "Mesh" container starts perfectly clean at the origin (0,0,0) at scale 1.
        Ok(Self {
            scale: mq::vec3(1.0, 1.0, 1.0),
            position: mq::vec3(0.0, 0.0, 0.0),
            rotation: mq::vec3(0.0, 0.0, 0.0),
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