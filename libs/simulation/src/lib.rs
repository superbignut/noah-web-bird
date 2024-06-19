use nalgebra as na;


#[derive(Debug)]
pub struct Simulation{
    world: World,
}
#[derive(Debug)]
pub struct  World{
    animals: Vec<Animal>,
    foods: Vec<Food>,
}
#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
