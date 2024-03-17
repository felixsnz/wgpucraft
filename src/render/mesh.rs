use crate::scene::terrain::{block::Block, chunk::Chunk};

use super::{pipelines::terrain::BlockVertex, Vertex};

#[derive(Clone)]

//TODO: change TerrainVertex for Vertex trait

/// Represents a vec-based mesh on the CPU
pub struct Mesh<V: Vertex> {
    verts: Vec<V>,
    indices: Vec<u16>
}

impl<V: Vertex> Mesh<V>
        
{
    /// Create a new `Mesh`.
    pub fn new() -> Self { Self { verts: Vec::new(), indices: Vec::new() } }

    /// Clear vertices, allows reusing allocated memory of the underlying Vec.
    pub fn clear(&mut self) { self.verts.clear(); }

    /// Get a slice referencing the vertices of this mesh.
    pub fn vertices(&self) -> &[V] { &self.verts }

    pub fn push(&mut self, vert: V) { self.verts.push(vert); }

    // new method to add indices
    pub fn push_indices(&mut self, indices: &[u16]) {
        self.indices.extend_from_slice(indices);
    }

    // returns the indices
    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn iter_verts(&self) -> std::slice::Iter<V> { self.verts.iter() }

    pub fn iter_indices(&self) -> std::vec::IntoIter<u16> { self.indices.clone().into_iter() }

    pub fn push_block(&mut self, block: Block, block_num: u16)
        where Vec<V>: Extend<BlockVertex>
    {


        let mut block_vertices = Vec::with_capacity(4 * 6);
        let mut block_indices = Vec::with_capacity(6 * 6);
        let mut quad_counter: u16 = 6 * block_num;

        for quad in block.quads.iter() {
            block_vertices.extend_from_slice(&quad.vertices);
            block_indices.extend_from_slice(&quad.get_indices(quad_counter));
            quad_counter += 1;
        }

        self.verts.extend(block_vertices);
        self.indices.extend(block_indices)



    }

    pub fn push_chunk(&mut self, chunk: &Chunk)
        where Vec<V>: Extend<BlockVertex>
    {

        let mut block_counter = 0;
        for block in &chunk.blocks {
            self.push_block(*block, block_counter);
            block_counter += 1;
        }
        
    }

    
}

