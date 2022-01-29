use super::vertex::Vertex;
use crate::{buffer::VBuffer, device::VDevice, impl_get, impl_get_ref};
use glam::Mat4;
use itertools::izip;

pub type Index = u32;

#[derive(Default, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,

    vertex_buffer: VBuffer,
    index_buffer: VBuffer,
}

impl Mesh {
    pub fn new(device: &VDevice, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let vertex_buffer =
            VBuffer::new_vertex_buffer(device, &vertices).expect("Failed to create vertex buffer.");

        let index_buffer =
            VBuffer::new_index_buffer(device, &indices).expect("Failed to create index buffer.");

        Self {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn from_file(device: &VDevice, file: &str) -> gltf::Result<Mesh> {
        let (gltf, buffers, _) = gltf::import(file)?;

        let mut vertices = Vec::with_capacity(buffers.len());
        let mut indices = Vec::with_capacity(buffers.len());

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let (Some(pos_iter), Some(norm_iter), Some(tex_iter)) = (
                    reader.read_positions(),
                    reader.read_normals(),
                    reader.read_tex_coords(0),
                ) {
                    assert_eq!(pos_iter.len(), norm_iter.len());
                    for (position, normal, uv) in izip!(pos_iter, norm_iter, tex_iter.into_f32()) {
                        vertices.push(Vertex::new(position.into(), normal.into(), uv.into()));
                    }
                }
                if let Some(iter) = reader.read_indices() {
                    for index in iter.into_u32() {
                        indices.push(index)
                    }
                }
            }
        }
        Ok(Mesh::new(device, vertices, indices))
    }
}

impl_get_ref!(Mesh, vertices, &[Vertex]);
impl_get_ref!(Mesh, indices, &[Index]);
impl_get!(Mesh, vertex_buffer, VBuffer);
impl_get!(Mesh, index_buffer, VBuffer);

// TODO Temp
pub struct MeshPushConstants {
    pub mvp: Mat4,
}
