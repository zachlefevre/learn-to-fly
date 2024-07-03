// the simulation engine
pub struct Simulation {
    world: World
}

pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>
}

struct Point2<T>{x: T, y: T}

impl<T> std::ops::Add<Point2<T>> for Point2<T>
where T: std::ops::Add<T, Output=T>
{
    type Output = Point2<T>;
    fn add(self, other: Point2<T>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Rotation2<T>{f: T}

struct Animal {
    position: Point2<f32>,
    rotation: Rotation2<f32>
}
struct Food {
    position: Point2<f32>,
    rotation: Rotation2<f32>,
    speed: f32
}
