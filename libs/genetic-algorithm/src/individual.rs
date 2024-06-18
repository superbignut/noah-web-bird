use rand::{seq::SliceRandom, Rng, RngCore};
use crate::*;



pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome)-> Self;
}