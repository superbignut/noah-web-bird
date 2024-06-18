mod chromosome;
mod crossover;
mod individual;
mod selection;
mod mutation;

use rand::{seq::SliceRandom, Rng, RngCore};

pub use self::chromosome::*;
pub use self::crossover::*;
pub use self::individual::*;
pub use self::selection::*;
pub use self::mutation::*;
/*
    
*/

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S> {
    pub fn evolve<T>(&self, rng: &mut dyn RngCore, population: &[T]) -> Vec<T>
    where
        T: Individual,
        S: SelectionMethod,
    {
        assert!(population.is_empty());
        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                self.mutation_method.mutate(rng, &mut child);
                
                T::create(child)
            })
            .collect()
    }

    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static
    ) -> Self {
        Self { 
            selection_method, 
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method)
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    struct TestIndividual {
        fitness: f32,
    }
    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
        fn chromosome(&self) -> &Chromosome {
            todo!()
        }
        fn create(chromosome: Chromosome)-> Self {
            todo!()
        }
    }

    #[test]
    fn it_works() {
        let mut count: BTreeMap<&str, usize> = BTreeMap::new();

        let test_str = "abc";

        *count.entry(test_str).or_insert(1) += 1;
    }

    #[test]
    fn roulette_wheel_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        for _ in 0..1000 {
            // 逻辑是，选一个数然后插入到btree里面吗
            let fitness = RouletteWheelSelection
                .select(&mut rng, &population)
                .fitness() as i32;

            *actual_histogram.entry(fitness).or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter([(1, 0), (2, 0), (3, 0), (4, 0)]);

        assert_eq!(actual_histogram, expected_histogram);
    }


    #[test]
    fn uniform_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();
        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

        let diff_a = child.iter().zip(parent_a).filter(|(c,p)| *c != p).count();
        let diff_b = child.iter().zip(parent_b).filter(|(c,p)| *c != p).count();
        dbg!(diff_a, diff_b);
        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }



    #[cfg(test)]
    mod tests {
        
        mod gaussian_mutation {
            use super::*;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;
            use super::super::{GaussianMutation, MutationMethod};

            fn actual(chance: f32, coeff: f32) -> Vec<f32> {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
    
                GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);
    
                child.into_iter().collect()
            }
            mod given_zero_chance {
                mod and_zero_coefficient {
                    #[test]
                    fn does_not_change_the_original_chromosome() {
                        todo!();
                    }
                }

                mod and_nonzero_coefficient {
                    #[test]
                    fn does_not_change_the_original_chromosome() {
                        todo!();
                    }
                }
            }

            mod given_fifty_fifty_chance {
                mod and_zero_coefficient {
                    #[test]
                    fn does_not_change_the_original_chromosome() {
                        todo!();
                    }
                }

                mod and_nonzero_coefficient {
                    #[test]
                    fn slightly_changes_the_original_chromosome() {
                        todo!();
                    }
                }
            }

            mod given_max_chance {
                mod and_zero_coefficient {
                    #[test]
                    fn does_not_change_the_original_chromosome() {
                        todo!();
                    }
                }

                mod and_nonzero_coefficient {
                    #[test]
                    fn entirely_changes_the_original_chromosome() {
                        todo!();
                    }
                }
            }
        }
    }
}

// --nocapture