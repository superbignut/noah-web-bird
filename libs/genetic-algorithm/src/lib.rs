mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;

use rand::{seq::SliceRandom, Rng, RngCore};

pub use self::chromosome::*;
pub use self::crossover::*;
pub use self::individual::*;
pub use self::mutation::*;
pub use self::selection::*;
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
        // 总体逻辑为，随机选择两个 Individual 杂交生成新的 Chromosome
        // 适当的突变
        // 生成新的
        assert!(!population.is_empty());
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
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
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

    #[derive(Clone, Debug, PartialEq)]
    enum TestIndividual {
        WithChromosome { chromosome: Chromosome },
        WithFitness { fitness: f32 },
    }
    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self::WithFitness  { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome { chromosome } => {
                    // 染色体的求和是fitness
                    chromosome.iter().sum()
                }
                Self::WithFitness { fitness } => *fitness
            }
        }
        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithChromosome { chromosome } => return chromosome,
                Self::WithFitness { fitness } => {
                    panic!("error withFitness")
                }
            }
        }
        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }

        }
    }
    #[test]
    fn genetic_algorithm() {

        fn individual(genes: &[f32]) -> TestIndividual {
            TestIndividual::create(genes.iter().cloned().collect())
        }

        let mut rng = ChaCha8Rng::from_seed(Default::default());


        // 这里直接用unit-like-struct作为参数，都不需要创建实例的
        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];
        
        // We're running `.evolve()` a few times, so that the differences between the
        // input and output population are easier to spot.
        //
        // No particular reason for a number of 10 - this test would be fine for 5, 20 or
        // even 1000 generations - the only thing that'd change is the magnitude of the
        // difference between the populations.
        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
            dbg!(population.len());
        }

        let expected_population = vec![
            individual(&[0.44769490, 2.0648358, 4.3058133]),
            individual(&[1.21268670, 1.5538777, 2.8869110]),
            individual(&[1.06176780, 2.2657390, 4.4287640]),
            individual(&[0.95909685, 2.4618788, 4.0247330]),
        ];

        assert_eq!(population, expected_population);
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

        let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();
        let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();
        dbg!(diff_a, diff_b);
        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }

    #[cfg(test)]
    mod tests {

        mod gaussian_mutation {
            use super::super::{GaussianMutation, MutationMethod};
            use super::*;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

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
