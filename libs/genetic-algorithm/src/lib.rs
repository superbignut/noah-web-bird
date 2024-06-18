use rand::RngCore;

pub struct GeneticAlgorithm;

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, poptlation: &'a [T]) -> &'a T
    where
        T: Individual,
    {
        todo!()
    }
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, poptlation: &'a [T]) -> &'a T
    where
        T: Individual,
    {
        todo!()
    }
}

impl GeneticAlgorithm {
    pub fn evolve<T>(&self, population: &[T]) -> Vec<T>
    where
        T: Individual,
    {
        assert!(population.is_empty());
        todo!()
    }
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
