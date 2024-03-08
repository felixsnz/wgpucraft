use cgmath::Vector3;

use crate::render::atlas::MaterialType;

use crate::render::pipelines::terrain::TerrainVertex as QuadVertex;

pub const CHUNK_Y_SIZE: usize = 200;
pub const CHUNK_Z_SIZE: usize = 16;
pub const CHUNK_X_SIZE: usize = 16;


pub fn quad_vertex(pos: [i8; 3], material_type: MaterialType, texture_corners: [u32; 2], position: [i32; 3], quad_side: QuadSide) -> QuadVertex {
    let tc = material_type.get_texture_coordinates(texture_corners, quad_side);
    QuadVertex {
        pos: [
            pos[0] as f32 + position[0] as f32,
            pos[1] as f32 + position[1] as f32,
            pos[2] as f32 + position[2] as f32,
        ],
        texture_coordinates: [tc[0] as f32, tc[1] as f32],
    }
}

#[derive(Copy, Clone, Debug)]
pub enum QuadSide {
    TOP,
    BOTTOM,
    RIGHT,
    LEFT,
    FRONT,
    BACK,
}

impl QuadSide {
    pub fn to_vec(self) -> Vector3<i32> {
        match self {
            QuadSide::TOP => Vector3::new(0, 1, 0),
            QuadSide::BOTTOM => Vector3::new(0, -1, 0),
            QuadSide::RIGHT => Vector3::new(1, 0, 0),
            QuadSide::LEFT => Vector3::new(-1, 0, 0),
            QuadSide::FRONT => Vector3::new(0, 0, 1),
            QuadSide::BACK => Vector3::new(0, 0, -1),
        }
    }

    fn get_vertices(self, material_type: MaterialType, position: [i32; 3]) -> [QuadVertex; 4] {
        match self {
            QuadSide::TOP => [
                quad_vertex([0, 1, 0], material_type, [0, 0], position, self),
                quad_vertex([0, 1, 1], material_type, [0, 1], position, self),
                quad_vertex([1, 1, 1], material_type, [1, 1], position, self),
                quad_vertex([1, 1, 0], material_type, [1, 0], position, self),
            ],
            QuadSide::BOTTOM => [
                quad_vertex([0, 0, 1], material_type, [0, 0], position, self),
                quad_vertex([0, 0, 0], material_type, [0, 1], position, self),
                quad_vertex([1, 0, 0], material_type, [1, 1], position, self),
                quad_vertex([1, 0, 1], material_type, [1, 0], position, self),
            ],
            QuadSide::RIGHT => [
                quad_vertex([1, 1, 1], material_type, [0, 0], position, self),
                quad_vertex([1, 0, 1], material_type, [0, 1], position, self),
                quad_vertex([1, 0, 0], material_type, [1, 1], position, self),
                quad_vertex([1, 1, 0], material_type, [1, 0], position, self),
            ],
            QuadSide::LEFT => [
                quad_vertex([0, 1, 0], material_type, [0, 0], position, self),
                quad_vertex([0, 0, 0], material_type, [0, 1], position, self),
                quad_vertex([0, 0, 1], material_type, [1, 1], position, self),
                quad_vertex([0, 1, 1], material_type, [1, 0], position, self),
            ],
            QuadSide::FRONT => [
                quad_vertex([0, 1, 1], material_type, [0, 0], position, self),
                quad_vertex([0, 0, 1], material_type, [0, 1], position, self),
                quad_vertex([1, 0, 1], material_type, [1, 1], position, self),
                quad_vertex([1, 1, 1], material_type, [1, 0], position, self),
            ],
            QuadSide::BACK => [
                quad_vertex([1, 1, 0], material_type, [0, 0], position, self),
                quad_vertex([1, 0, 0], material_type, [0, 1], position, self),
                quad_vertex([0, 0, 0], material_type, [1, 1], position, self),
                quad_vertex([0, 1, 0], material_type, [1, 0], position, self),
            ],
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Quad {
    pub vertices: [QuadVertex; 4],
    pub side: QuadSide,
}

impl Quad {
    fn new(material_type: MaterialType, quad_side: QuadSide, position: [i32; 3]) -> Self {
        Self {
            vertices: quad_side.get_vertices(material_type, position),
            side: quad_side,
        }
    }

    pub fn get_indices(&self, i: u16) -> [u16; 6] {
        let displacement = i * 4;
        [
            0 + displacement,
            1 + displacement,
            2 + displacement,
            2 + displacement,
            3 + displacement,
            0 + displacement,
        ]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub quads: [Quad; 6],
    pub position: [i32; 3],
    pub material_type: MaterialType,
}

impl Block {
    pub fn new(material_type: MaterialType, position: [i32; 3], chunk_offset: [i32; 3]) -> Self {
        let quads = Block::generate_quads(material_type, position, chunk_offset);

        Self {
            quads,
            position,
            material_type,
        }
    }

    fn generate_quads(material_type: MaterialType, position: [i32; 3], chunk_offset: [i32; 3]) -> [Quad; 6] {
        let world_pos = [
            position[0] + (chunk_offset[0] * CHUNK_X_SIZE as i32),
            position[1],
            position[2] + (chunk_offset[2] * CHUNK_Z_SIZE as i32),
        ];

        let top = Quad::new(material_type, QuadSide::TOP, world_pos);
        let bottom = Quad::new(material_type, QuadSide::BOTTOM, world_pos);
        let right = Quad::new(material_type, QuadSide::RIGHT, world_pos);
        let left = Quad::new(material_type, QuadSide::LEFT, world_pos);
        let front = Quad::new(material_type, QuadSide::FRONT, world_pos);
        let back = Quad::new(material_type, QuadSide::BACK, world_pos);

        [top, bottom, right, left, front, back]
    }

    pub fn update(&mut self, new_material_type: MaterialType, offset: [i32; 3]) {
        self.material_type = new_material_type;
        self.quads = Block::generate_quads(new_material_type, self.position, offset);
    }
}
