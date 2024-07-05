use nalgebra as na;
use rand::{Rng, RngCore};

// the simulation engine
pub struct Simulation {
    world: World
}

pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>
}

struct Point2<T>{x: T, y: T}

struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>
}
struct Food {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32
}
