
use crate::scene::terrain::chunk::{Blocks, pos_in_chunk_bounds, CHUNK_AREA, CHUNK_Y_SIZE};


use super::{atlas::MaterialType, pipelines::terrain::BlockVertex, Vertex};


#[derive(Clone, Default)]


/// Represents a vec-based mesh on the CPU
pub struct Mesh<V: Vertex> {
    pub verts: Vec<V>,
    pub indices: Vec<u16>
}


impl<V: Vertex> Mesh<V>
       
{
    /// Create a new `Mesh`.
    pub fn new() -> Self { Self { verts: Vec::new(), indices: Vec::new() } }


    pub fn from(blocks: &Blocks) -> Option<Self>
        where Vec<V>: Extend<BlockVertex>
    {


        let mut mesh = Self::new();
        // Example condition: return None if the chunk has no blocks
        if blocks.iter().flat_map(|y| y.iter()).flat_map(|x| x.iter()).all(|b| b.read().unwrap().material_type == MaterialType::AIR) {
            None
        } else {
            mesh.push_chunk(blocks);
            Some(mesh)
        }
    }


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


    pub fn push_chunk(&mut self, blocks: &Blocks)
        where Vec<V>: Extend<BlockVertex>
    {
        for y in 0.. CHUNK_Y_SIZE{
            for z in 0..CHUNK_AREA {
                for x in 0..CHUNK_AREA {


                    let block = blocks[y][x][z].read().unwrap();
                    let mut block_vertices = Vec::with_capacity(4 * 6);
                    let mut block_indices: Vec<u16> = Vec::with_capacity(6 * 6);


                    if block.material_type as i32 == MaterialType::AIR as i32 {
                        continue;
                    }


                    let mut quad_counter = 0;
                    for quad in block.quads.iter() {
                        let mut visible = false;
                        let neighbor_pos = block.get_vec_position() + quad.side.to_vec();


                        if pos_in_chunk_bounds(neighbor_pos) {
                            let neighbor_block = blocks[neighbor_pos.y as usize][neighbor_pos.x as usize][neighbor_pos.z as usize].read().unwrap();
                            if neighbor_block.material_type as u16 == MaterialType::AIR as u16 {
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
                   
                    block_indices = block_indices.iter().map(|i| i + self.verts.len() as u16).collect();
                    self.verts.extend(block_vertices);
                    self.indices.extend(block_indices);
                }
            }
        }
    }


   
}







