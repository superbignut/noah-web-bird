use rand::{seq::SliceRandom, Rng, RngCore};
use crate::*;



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
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual, // T 具有 Individual 特征
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}