use noise::{NoiseFn, Perlin};

pub struct NoiseGenerator {
    perlin: Perlin,
    octaves: u32,
    persistence: f32,
    lacunarity: f32,
}

impl NoiseGenerator {
    pub fn new(seed: u32, octaves: u32, persistence: f32, lacunarity: f32) -> Self {
        let perlin = Perlin::new(seed);
        Self { perlin, octaves, persistence, lacunarity }
    }

    pub fn get_height(&self, x: f32, z: f32, frequency: f32, amplitude: f32) -> f32 {
        let mut total = 0.0;
        let mut max_value = 0.0;
        let mut amplitude = amplitude;
        let mut frequency = frequency;

        for _ in 0..self.octaves {
            total += self.perlin.get([x as f64 * frequency as f64, z as f64 * frequency as f64]) as f32 * amplitude;
            max_value += amplitude;

            amplitude *= self.persistence;
            frequency *= self.lacunarity;
        }

        total / max_value
    }
}