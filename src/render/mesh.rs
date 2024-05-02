use cgmath::Vector3;

use crate::scene::terrain::chunk::{Chunk, CHUNK_AREA, CHUNK_Y_SIZE};

use super::{atlas::MaterialType, pipelines::terrain::BlockVertex, Vertex};

#[derive(Clone)]

/// Represents a vec-based mesh on the CPU
pub struct Mesh<V: Vertex> {
    verts: Vec<V>,
    indices: Vec<u32>
}

impl<V: Vertex> Mesh<V>
        
{
    /// Create a new `Mesh`.
    pub fn new() -> Self { Self { verts: Vec::new(), indices: Vec::new() } }

    pub fn from(chunk: &Chunk) -> Option<Self>
        where Vec<V>: Extend<BlockVertex>
    {

        let mut mesh = Self::new();
        // Example condition: return None if the chunk has no blocks
        if chunk.blocks.iter().flat_map(|y| y.iter()).flat_map(|x| x.iter()).all(|b| b.lock().unwrap().material_type == MaterialType::AIR) {
            None
        } else {
            mesh.push_chunk(chunk);
            Some(mesh)
        }
    }

    /// Clear vertices, allows reusing allocated memory of the underlying Vec.
    pub fn clear(&mut self) { self.verts.clear(); }

    /// Get a slice referencing the vertices of this mesh.
    pub fn vertices(&self) -> &[V] { &self.verts }

    pub fn push(&mut self, vert: V) { self.verts.push(vert); }

    // new method to add indices
    pub fn push_indices(&mut self, indices: &[u32]) {
        self.indices.extend_from_slice(indices);
    }

    // returns the indices
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn iter_verts(&self) -> std::slice::Iter<V> { self.verts.iter() }

    pub fn iter_indices(&self) -> std::vec::IntoIter<u32> { self.indices.clone().into_iter() }

    pub fn push_chunk(&mut self, chunk: &Chunk)
        where Vec<V>: Extend<BlockVertex>
    {
        for y in 0.. CHUNK_Y_SIZE{
            for z in 0..CHUNK_AREA {
                for x in 0..CHUNK_AREA {

                    let block = chunk.blocks[y][x][z].lock().unwrap();
                    let mut block_vertices = Vec::with_capacity(4 * 6);
                    let mut block_indices: Vec<u32> = Vec::with_capacity(6 * 6);

                    if block.material_type as i32 == MaterialType::AIR as i32 {
                        continue;
                    }

                    let mut quad_counter = 0;
                    for quad in block.quads.iter() {
                        let mut visible = false;
                        let neighbour_pos: Vector3<i32> = block.get_vec_position() + quad.side.to_vec();

                        if Chunk::pos_in_chunk_bounds(neighbour_pos) {
                            let neighbour_block = chunk.blocks[neighbour_pos.y as usize][neighbour_pos.x as usize][neighbour_pos.z as usize].lock().unwrap();
                            if neighbour_block.material_type as u16 == MaterialType::AIR as u16 {
                                visible = true;
                            }

                        } else {
                            visible = true
                        }
                        if visible {
                            block_vertices.extend_from_slice(&quad.vertices);
                            block_indices.extend_from_slice(&quad.get_indices(quad_counter));
                            quad_counter += 1;
                            
                        }
                    }
                    
                    block_indices = block_indices.iter().map(|i| i + self.verts.len() as u32).collect();
                    self.verts.extend(block_vertices);
                    self.indices.extend(block_indices);
                }
            }
        }
    }

    
}

