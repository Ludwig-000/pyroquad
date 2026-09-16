use macroquad::{prelude as mq};
use glam::{Vec3A, Mat3A, Quat, EulerRot};
use gltf::mesh::util::ReadIndices;
use glam::{ Mat4, Mat3, Vec3};

/// Largest relative index we allow in a rendering chunk.
const MAX_CHUNK_INDEX: usize = 65_000;

#[derive(Clone, Copy, Debug)]
pub struct SplitPoint {
    pub vertex: usize,
    pub index: usize,
}

#[derive(Clone)]
pub struct Mesh {
    
    pub scale: mq::Vec3,
    pub position: mq::Vec3,
    pub rotation: mq::Vec3,

    pub draw_each_frame: bool,

    pub splitpoints: Option<Vec<SplitPoint>>,
    pub vertices: Vec<mq::Vertex>,
    pub indices: Vec<u16>,
    pub texture: Option<mq::Texture2D>,
}

impl Mesh{
    pub fn draw(&self, gl: &mut macroquad::prelude::QuadGl ){
        
        gl.texture(self.texture.as_ref());

        // println!("Drawing {} Vertices and {} Indices", self.vertices.len(), self.indices.len());
        match self.splitpoints.as_deref() {
            None => gl.geometry(&self.vertices, &self.indices),
            Some(splitpoints) => {

                let mut vtx_start = 0usize;
                let mut idx_start = 0usize;

                for sp in splitpoints {
                    gl.geometry(
                        &self.vertices[vtx_start..sp.vertex],
                        &self.indices[idx_start..sp.index],
                    );
                    vtx_start = sp.vertex;
                    idx_start = sp.index;
                }

                if vtx_start < self.vertices.len() {
                    gl.geometry(
                        &self.vertices[vtx_start..],
                        &self.indices[idx_start..],
                    );
                }
            }
        }
    }

    pub fn absolute_indices_u32(&self) -> Vec<u32> {
        let Some(splitpoints) = self.splitpoints.as_deref() else {
            return self.indices.iter().map(|&i| i as u32).collect();
        };

        let mut result = Vec::with_capacity(self.indices.len());
        let mut vtx_start: u32 = 0;
        let mut idx_start: usize = 0;

        for sp in splitpoints {
            for &idx in &self.indices[idx_start..sp.index] {
                result.push(idx as u32 + vtx_start);
            }
            vtx_start = sp.vertex as u32;
            idx_start = sp.index;
        }

        for &idx in &self.indices[idx_start..] {
            result.push(idx as u32 + vtx_start);
        }

        result
    }

    pub fn load_from_bytes(data: &[u8], texture: Option<mq::Texture2D>) -> Result<Self, String> {
        if data.len() >= 4 && &data[0..4] == b"glTF" {
            return Self::load_from_gltf(data, texture);
        }

        let start = data.iter().position(|&b| !b.is_ascii_whitespace() && b != 0xEF && b != 0xBB && b != 0xBF);
        if let Some(idx) = start {
            if data[idx] == b'{' {
                return Self::load_from_gltf(data, texture);
            }
        }

        let is_ascii_ish = data.iter().take(512).all(|&b| b.is_ascii() || b > 127);
        if is_ascii_ish {
            return Self::load_from_obj(data, texture);
        }

        Err("Unrecognized mesh format. Supported formats: glTF/GLB (.glb, .gltf) and Wavefront OBJ (.obj)".to_string())
    }



    pub fn load_from_gltf(data: &[u8], texture: Option<mq::Texture2D>) -> Result<Self, String> {
        let (document, buffers, images) =
            gltf::import_slice(data).map_err(|e| format!("glTF import error: {e}"))?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut splitpoints: Option<Vec<SplitPoint>> = None;
        let mut extracted_texture: Option<mq::Texture2D> = None;

        let mut chunk_vertex_count: usize = 0;

        let mut global_transforms = vec![Mat4::IDENTITY; document.nodes().count()];
        let mut stack = Vec::new();

        if let Some(scene) = document.default_scene().or_else(|| document.scenes().next()) {
            for node in scene.nodes() {
                stack.push((node, Mat4::IDENTITY));
            }
        }

        while let Some((node, parent_transform)) = stack.pop() {
            let local_transform = Mat4::from_cols_array_2d(&node.transform().matrix());
            let global_transform = parent_transform * local_transform;
            global_transforms[node.index()] = global_transform;

            for child in node.children() {
                stack.push((child, global_transform));
            }
        }

        if texture.is_none() && !images.is_empty() {
            extracted_texture = try_decode_gltf_image(&images[0]);
        }

        let final_texture = texture.or(extracted_texture);

        for node in document.nodes() {
            if let Some(mesh) = node.mesh() {
                let transform = global_transforms[node.index()];
                let normal_matrix = Mat3::from_mat4(transform).inverse().transpose();

                for primitive in mesh.primitives() {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                    let material_color = extract_material_color(&primitive.material());

                    let positions: Vec<[f32; 3]> = match reader.read_positions() {
                        Some(iter) => iter.collect(),
                        None => continue,
                    };

                    let prim_vertex_count = positions.len();

                    let primitive_indices: Vec<u32> = match reader.read_indices() {
                        Some(read_indices) => match read_indices {
                            ReadIndices::U16(iter) => iter.map(u32::from).collect(),
                            ReadIndices::U32(iter) => iter.collect(),
                            ReadIndices::U8(iter) => iter.map(u32::from).collect(),
                        },
                        None => Vec::new(),
                    };

                    if let Some(&max_index) = primitive_indices.iter().max() {
                        let max_index = max_index as usize;
                        if max_index > MAX_CHUNK_INDEX {
                            return Err(format!(
                                "glTF primitive has referenced index {max_index}, which exceeds the configured u16 chunk limit"
                            ));
                        }

                        if chunk_vertex_count > 0
                            && chunk_vertex_count + max_index > MAX_CHUNK_INDEX
                        {
                            splitpoints.get_or_insert_with(Vec::new).push(SplitPoint {
                                vertex: vertices.len(),
                                index: indices.len(),
                            });
                            chunk_vertex_count = 0;
                        }
                    }

                    let normals: Vec<[f32; 3]> = reader
                        .read_normals()
                        .map(|n| n.collect())
                        .unwrap_or_else(|| vec![[0.0, 1.0, 0.0]; prim_vertex_count]);

                    let tex_coords: Vec<_> = reader
                        .read_tex_coords(0)
                        .map(|uv| uv.into_f32().map(|v| mq::vec2(v[0], v[1])).collect())
                        .unwrap_or_else(|| vec![mq::vec2(0.0, 0.0); prim_vertex_count]);

                    let colors: Vec<_> = reader
                        .read_colors(0)
                        .map(|c| {
                            c.into_rgba_f32()
                                .map(|rgba| mq::Color::from_vec(mq::vec4(rgba[0], rgba[1], rgba[2], rgba[3])))
                                .collect()
                        })
                        .unwrap_or_else(|| vec![material_color; prim_vertex_count]);

                    let vertex_start = chunk_vertex_count;

                    for i in 0..prim_vertex_count {
                        let local_pos = Vec3::from_array(positions[i]);
                        let world_pos = transform.transform_point3(local_pos);

                        let local_normal = Vec3::from_array(normals[i]);
                        let world_normal = normal_matrix.mul_vec3(local_normal).normalize_or_zero();

                        vertices.push(mq::Vertex {
                            position: mq::vec3(world_pos.x, world_pos.y, world_pos.z),
                            uv: tex_coords[i],
                            color: colors[i].into(),
                            normal: mq::vec3(world_normal.x, world_normal.y, world_normal.z).extend(0.0),
                        });
                    }

                    chunk_vertex_count += prim_vertex_count;

                    indices.extend(primitive_indices.into_iter().map(|i| {
                        (i as usize + vertex_start) as u16
                    }));
                }
            }
        }
        Ok(Self {
            scale: mq::vec3(1.0, 1.0, 1.0),
            position: mq::vec3(0.0, 0.0, 0.0),
            rotation: mq::vec3(0.0, 0.0, 0.0),
            draw_each_frame: true,
            splitpoints,
            vertices,
            indices,
            texture: final_texture,
        })
    }


    pub fn load_from_obj(data: &[u8], texture: Option<mq::Texture2D>) -> Result<Self, String> {

        let mut cursor = std::io::Cursor::new(data);

        let (models, _materials_result) = tobj::load_obj_buf(
            &mut cursor,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |_mtl_path| Err(tobj::LoadError::GenericFailure),
        ).map_err(|e| format!("OBJ load error: {e}"))?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut splitpoints: Option<Vec<SplitPoint>> = None;
        let mut chunk_vertex_count: usize = 0;

        for model in &models {
            let mesh = &model.mesh;
            let num_verts = mesh.positions.len() / 3;

            let max_index = mesh.indices.iter().copied().max().unwrap_or(0) as usize;
            if max_index > MAX_CHUNK_INDEX {
                return Err(format!(
                    "OBJ mesh contains index {max_index}, which cannot be represented by the u16 renderer"
                ));
            }

            if chunk_vertex_count > 0 && chunk_vertex_count + max_index > MAX_CHUNK_INDEX {
                splitpoints.get_or_insert_with(Vec::new).push(SplitPoint {
                    vertex: vertices.len(),
                    index: indices.len(),
                });
                chunk_vertex_count = 0;
            }

            let vertex_start = chunk_vertex_count;

            for i in 0..num_verts {
                let px = mesh.positions[i * 3];
                let py = mesh.positions[i * 3 + 1];
                let pz = mesh.positions[i * 3 + 2];

                let (nx, ny, nz) = if !mesh.normals.is_empty() && i * 3 + 2 < mesh.normals.len() {
                    (mesh.normals[i * 3], mesh.normals[i * 3 + 1], mesh.normals[i * 3 + 2])
                } else {
                    (0.0, 1.0, 0.0)
                };

                let uv = if !mesh.texcoords.is_empty() && i * 2 + 1 < mesh.texcoords.len() {
                    mq::vec2(mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1])
                } else {
                    mq::vec2(0.0, 0.0)
                };

                let color = if !mesh.vertex_color.is_empty() && i * 3 + 2 < mesh.vertex_color.len() {
                    mq::Color::new(
                        mesh.vertex_color[i * 3],
                        mesh.vertex_color[i * 3 + 1],
                        mesh.vertex_color[i * 3 + 2],
                        1.0,
                    )
                } else {
                    mq::WHITE
                };

                vertices.push(mq::Vertex {
                    position: mq::vec3(px, py, pz),
                    uv,
                    color: color.into(),
                    normal: mq::vec3(nx, ny, nz).extend(0.0),
                });
            }

            chunk_vertex_count += num_verts;

            for &idx in &mesh.indices {
                indices.push((idx as usize + vertex_start) as u16);
            }
        }

        Ok(Self {
            scale: mq::vec3(1.0, 1.0, 1.0),
            position: mq::vec3(0.0, 0.0, 0.0),
            rotation: mq::vec3(0.0, 0.0, 0.0),
            draw_each_frame: true,
            splitpoints,
            vertices,
            indices,
            texture,
        })
    }


    pub fn  recalculate_pos(&mut self, old_pos: mq::Vec3, new_pos: mq::Vec3) {
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

        for vertex in self.vertices.iter_mut() {
            let pos = Vec3A::from(vertex.position);
            
            let offset = pos - pivot_simd;
            let final_pos = pivot_simd + (offset * ratio);

            vertex.position = glam::Vec3::from(final_pos);
        }
    }
}


fn extract_material_color(material: &gltf::Material) -> mq::Color {
    let pbr = material.pbr_metallic_roughness();
    let base = pbr.base_color_factor();

    let mut r = base[0];
    let mut g = base[1];
    let mut b = base[2];
    let a = base[3];

    let emissive = material.emissive_factor();
    if emissive[0] > 0.0 || emissive[1] > 0.0 || emissive[2] > 0.0 {
        r = (r + emissive[0]).min(1.0);
        g = (g + emissive[1]).min(1.0);
        b = (b + emissive[2]).min(1.0);
    }

    mq::Color::new(r, g, b, a)
}


/// Attempts to decode an embedded glTF image into a macroquad Texture2D.
/// Returns None on failure (graceful — we just skip the texture).
fn try_decode_gltf_image(image_data: &gltf::image::Data) -> Option<mq::Texture2D> {
    let (width, height) = (image_data.width, image_data.height);

    // Convert pixel data to RGBA8 regardless of source format
    let rgba_bytes = match image_data.format {
        gltf::image::Format::R8G8B8A8 => {
            image_data.pixels.clone()
        }
        gltf::image::Format::R8G8B8 => {
            // Expand RGB → RGBA
            let mut rgba = Vec::with_capacity(image_data.pixels.len() / 3 * 4);
            for chunk in image_data.pixels.chunks_exact(3) {
                rgba.push(chunk[0]);
                rgba.push(chunk[1]);
                rgba.push(chunk[2]);
                rgba.push(255);
            }
            rgba
        }
        gltf::image::Format::R8 => {
            // Grayscale → RGBA
            let mut rgba = Vec::with_capacity(image_data.pixels.len() * 4);
            for &p in &image_data.pixels {
                rgba.push(p);
                rgba.push(p);
                rgba.push(p);
                rgba.push(255);
            }
            rgba
        }
        gltf::image::Format::R8G8 => {
            // RG → RGBA (treat as grayscale + alpha)
            let mut rgba = Vec::with_capacity(image_data.pixels.len() * 2);
            for chunk in image_data.pixels.chunks_exact(2) {
                rgba.push(chunk[0]);
                rgba.push(chunk[0]);
                rgba.push(chunk[0]);
                rgba.push(chunk[1]);
            }
            rgba
        }
        gltf::image::Format::R16 | gltf::image::Format::R16G16 |
        gltf::image::Format::R16G16B16 | gltf::image::Format::R16G16B16A16 => {
            // 16-bit formats: convert to 8-bit by taking the high byte
            let bytes_per_component = 2;
            let components_per_pixel = match image_data.format {
                gltf::image::Format::R16 => 1,
                gltf::image::Format::R16G16 => 2,
                gltf::image::Format::R16G16B16 => 3,
                gltf::image::Format::R16G16B16A16 => 4,
                _ => return None,
            };
            let pixel_count = (image_data.pixels.len() / (components_per_pixel * bytes_per_component)) as usize;
            let mut rgba = Vec::with_capacity(pixel_count * 4);

            for pixel_idx in 0..pixel_count {
                let base = pixel_idx * components_per_pixel * bytes_per_component;
                // Read high byte of each 16-bit component (little-endian: high byte is at offset 1)
                let r = if components_per_pixel >= 1 { image_data.pixels[base + 1] } else { 0 };
                let g = if components_per_pixel >= 2 { image_data.pixels[base + bytes_per_component + 1] } else { r };
                let b = if components_per_pixel >= 3 { image_data.pixels[base + 2 * bytes_per_component + 1] } else { r };
                let a = if components_per_pixel >= 4 { image_data.pixels[base + 3 * bytes_per_component + 1] } else { 255 };
                rgba.push(r);
                rgba.push(g);
                rgba.push(b);
                rgba.push(a);
            }
            rgba
        }
        // R32G32B32FLOAT or R32G32B32A32FLOAT
        gltf::image::Format::R32G32B32FLOAT | gltf::image::Format::R32G32B32A32FLOAT => {
            let components = match image_data.format {
                gltf::image::Format::R32G32B32FLOAT => 3,
                gltf::image::Format::R32G32B32A32FLOAT => 4,
                _ => return None,
            };
            let pixel_count = image_data.pixels.len() / (components * 4);
            let mut rgba = Vec::with_capacity(pixel_count * 4);

            for pixel_idx in 0..pixel_count {
                let base = pixel_idx * components * 4;
                let r = f32::from_le_bytes([image_data.pixels[base], image_data.pixels[base+1], image_data.pixels[base+2], image_data.pixels[base+3]]);
                let g = f32::from_le_bytes([image_data.pixels[base+4], image_data.pixels[base+5], image_data.pixels[base+6], image_data.pixels[base+7]]);
                let b = f32::from_le_bytes([image_data.pixels[base+8], image_data.pixels[base+9], image_data.pixels[base+10], image_data.pixels[base+11]]);
                let a = if components == 4 {
                    f32::from_le_bytes([image_data.pixels[base+12], image_data.pixels[base+13], image_data.pixels[base+14], image_data.pixels[base+15]])
                } else {
                    1.0
                };
                rgba.push((r.clamp(0.0, 1.0) * 255.0) as u8);
                rgba.push((g.clamp(0.0, 1.0) * 255.0) as u8);
                rgba.push((b.clamp(0.0, 1.0) * 255.0) as u8);
                rgba.push((a.clamp(0.0, 1.0) * 255.0) as u8);
            }
            rgba
        }
    };

    let expected_len = (width * height * 4) as usize;
    if rgba_bytes.len() < expected_len {
        return None; // Data mismatch — skip gracefully
    }

    let mq_texture = mq::Texture2D::from_rgba8(width as u16, height as u16, &rgba_bytes);
    Some(mq_texture)
}
