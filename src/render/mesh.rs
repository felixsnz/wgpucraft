use cgmath::Vector3;

use crate::scene::terrain::{block::Block, chunk::{Chunk, CHUNK_AREA, CHUNK_Y_SIZE}};

use super::{atlas::MaterialType, pipelines::terrain::BlockVertex, Vertex};

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

    pub fn push_chunk(&mut self, chunk: &Chunk)
        where Vec<V>: Extend<BlockVertex>
    {
        for y in 0.. CHUNK_Y_SIZE{
            for z in 0..CHUNK_AREA {
                for x in 0..CHUNK_AREA {

                    let mut block_vertices = Vec::with_capacity(4 * 6 );
                    let mut block_indices = Vec::with_capacity(6 * 6);
                    let block = chunk.blocks[y][z][x];

                    if block.material_type as i32 == MaterialType::AIR as i32 {
                        continue;
                    }

                    let mut visible = false;

                    let mut quad_counter = 0;
                    for quad in block.quads {

                        let neighbour_pos: Vector3<i32> = block.get_vec_position() + quad.side.to_vec() ;


                        if Chunk::pos_in_chunk_bounds(neighbour_pos) {
                            let neighbour_block = chunk.blocks[neighbour_pos.y as usize][neighbour_pos.z as usize][neighbour_pos.x as usize];

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
                    let block_indices: Vec<u16> = block_indices.iter().map(|i| i + self.verts.len() as u16).collect();
                    self.verts.extend(block_vertices);
                    self.indices.extend(block_indices);
                }
            }
        }
    }

    
}

