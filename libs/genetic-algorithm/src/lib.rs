use rand::{RngCore, seq::SliceRandom, Rng};

pub struct Chromosome{genes: Vec<f32>}
impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect()
        }
    }
}

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverAlgorithm>,
    mutation_method: Box<dyn MutationAlgorithm>
}

impl<S> GeneticAlgorithm<S>
where S: SelectionAlgorithm {
    pub fn new(selection_method: S, crossover_method: impl CrossoverAlgorithm + 'static, mutation_method: impl MutationAlgorithm + 'static) -> GeneticAlgorithm<S> {
        GeneticAlgorithm {
            selection_method: selection_method, crossover_method: Box::new(crossover_method), mutation_method: Box::new(mutation_method)
        }
    }
    pub fn evolve<Agent: Individual>(&self, rng: &mut dyn RngCore, population: &[Agent]) -> Vec<Agent> {
        population.iter().map(|_| {
            let parent_a = self.selection_method.select(rng, population).chromosome();
            let parent_b = self.selection_method.select(rng, population).chromosome();
            let mut child = (*self.crossover_method).crossover(rng, parent_a, parent_b);
            self.mutation_method.mutate(rng, &mut child);
            Agent::create(child)
        }).collect::<Vec<_>>()
    }
}

pub trait SelectionAlgorithm {
    fn select<'a, Agent: Individual>(&self, rng: &mut dyn RngCore, individuals: &'a [Agent]) -> &'a Agent;
}

pub struct RouletteWheelSelection {}
impl SelectionAlgorithm for RouletteWheelSelection {
    fn select<'a, Agent: Individual>(&self, rng: &mut dyn RngCore, individuals: &'a [Agent]) -> &'a Agent {
       let random_agent =  individuals.choose_weighted(rng, |individual| individual.fitness());
        random_agent.unwrap()
    }
}

pub trait CrossoverAlgorithm {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome;
}

pub struct UniformCrossover;
impl CrossoverAlgorithm for UniformCrossover {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome {
        parent_a.iter().zip(parent_b.iter()).map(|(&a, &b)| if rng.gen_bool(0.5) {a} else {b}).collect::<Chromosome>()
    }
}

pub trait MutationAlgorithm {
    fn mutate(&self, rng: &mut dyn RngCore, chromosome: &mut Chromosome);
}

pub struct GaussianMutation {
    chance: f32, coeff: f32
}

impl GaussianMutation {
    pub fn new(change: f32, coeff: f32) -> Self {
        assert!(change >= 0.0 && change <= 1.0);
        Self { chance: change, coeff } }
}

impl MutationAlgorithm for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, chromosome: &mut Chromosome) {
        for gene in chromosome.iter_mut() {
            let sign = if rng.gen_bool(0.5) {-1} else {1} as f32;
            if rng.gen_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    struct TestAgent {
        fitness: f32
    }

    impl TestAgent {
        fn new(fitness: f32) -> TestAgent {
            TestAgent { fitness }
        }
    }

    impl Individual for TestAgent {
        fn fitness(&self) -> f32 {
            self.fitness
        }

        fn chromosome(&self) -> &Chromosome {
            unimplemented!("tests")
        }

        fn create(_: Chromosome) -> Self {
            todo!()
        }
    }


    #[test]
    fn roulette_wheel_selection() {
        let population: Vec<TestAgent> =
            (0..10).map(|i| i as f32).map(TestAgent::new).collect::<Vec<_>>();
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut histogram = BTreeMap::new();
        for _ in 0..100_000 {
            let selected = RouletteWheelSelection{}.select(&mut rng, &population).fitness as i32;
            *histogram.entry(selected).or_insert(0) += 1;
        }
        let expected_histogram = BTreeMap::from_iter([
            (1, 2238),
            (2, 4448),
            (3, 6671),
            (4, 8839),
            (5, 11013),
            (6, 13386),
            (7, 15424),
            (8, 17645),
            (9, 20336),
        ]);
        assert_eq!(histogram, expected_histogram)
    }
}
