use crate::scene::world::{block::{Block, QuadVertex}};

#[derive(Clone)]

/// Represents a vec-based mesh on the CPU
pub struct Mesh{
    verts: Vec<QuadVertex>,
    indices: Vec<u16>
}

impl Mesh{
    /// Create a new `Mesh`.
    pub fn new() -> Self { Self { verts: Vec::new(), indices: Vec::new() } }

    /// Clear vertices, allows reusing allocated memory of the underlying Vec.
    pub fn clear(&mut self) { self.verts.clear(); }

    /// Get a slice referencing the vertices of this mesh.
    pub fn vertices(&self) -> &[QuadVertex] { &self.verts }

    pub fn push(&mut self, vert: QuadVertex) { self.verts.push(vert); }

    // new method to add indices
    pub fn push_indices(&mut self, indices: &[u16]) {
        self.indices.extend_from_slice(indices);
    }

    // returns the indices
    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn iter_verts(&self) -> std::slice::Iter<QuadVertex> { self.verts.iter() }

    pub fn iter_indices(&self) -> std::vec::IntoIter<u16> { self.indices.clone().into_iter() }

    pub fn push_block(&mut self, block: Block) {


        let mut block_vertices = Vec::with_capacity(4 * 6);
        let mut block_indices = Vec::with_capacity(6 * 6);
        let mut face_counter: u16 = 0;
        for face in block.quads.iter() {
            block_vertices.extend_from_slice(&face.vertices);
            block_indices.extend_from_slice(&face.get_indices(face_counter));
            face_counter += 1;
        }

        self.verts.extend(block_vertices);
        self.indices.extend(block_indices)



    }

    
}