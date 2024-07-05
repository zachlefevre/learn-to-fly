use nalgebra as na;
use rand::{Rng, RngCore};

// the simulation engine
pub struct Simulation {
    world: World
}

impl Simulation {
    fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng)
        }
    }
}

pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>
}

impl World {
    fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            animals: (0..40).map(|_| Animal::random(rng)).collect(),
            foods: (0..60).map(|_| Food::random(rng)).collect(),
        }
    }
}

struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32

}

impl Animal {
    fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002
        }
    }
}

struct Food {
    position: na::Point2<f32>
}

impl Food {
    fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen()
        }
    }
}
