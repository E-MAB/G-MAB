use std::collections::HashSet;

use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::arm::{Arm, OptimizationFn};

pub(crate) struct GeneticAlgorithm<F: OptimizationFn> {
    mutation_rate: f64,
    crossover_rate: f64,
    mutation_span: f64,
    pub(crate) population_size: usize,
    pub(crate) opti_function: F,
    max_simulations: i32,
    dimension: usize,
    lower_bound: Vec<i32>,
    upper_bound: Vec<i32>,
    pub(crate) simulations_used: i32,
}

impl<F: OptimizationFn + Clone> GeneticAlgorithm<F> {
    pub(crate) fn update_simulations_used(&mut self, number_of_new_simulations: i32) {
        self.simulations_used += number_of_new_simulations;
    }

    pub(crate) fn budget_reached(&self) -> bool {
        self.simulations_used >= self.max_simulations
    }

    pub(crate) fn new(
        opti_function: F,
        population_size: usize,
        mutation_rate: f64,
        crossover_rate: f64,
        mutation_span: f64,
        max_simulations: i32,
        dimension: usize,
        lower_bound: Vec<i32>,
        upper_bound: Vec<i32>,
    ) -> Self {
        Self {
            mutation_rate,
            crossover_rate,
            mutation_span,
            population_size,
            opti_function,
            max_simulations,
            dimension,
            lower_bound,
            upper_bound,
            simulations_used: 0,
        }
    }

    pub(crate) fn generate_new_population(&self) -> Vec<Arm> {
        let mut individuals: Vec<Arm> = Vec::new();
        let mut rng = rand::thread_rng();

        while individuals.len() < self.population_size {
            let candidate_solution: Vec<i32> = (0..self.dimension)
                .map(|j| rng.gen_range(self.lower_bound[j]..=self.upper_bound[j]))
                .collect();

            let candidate_arm = Arm::new(&candidate_solution);

            if !individuals.contains(&candidate_arm) {
                individuals.push(candidate_arm);
            }
        }
        individuals
    }

    pub(crate) fn crossover(&self, population: &[Arm]) -> Vec<Arm> {
        let mut crossover_pop: Vec<Arm> = Vec::new();
        let population_size = self.population_size;
        let mut rng = rand::thread_rng();

        for i in (0..population_size).step_by(2) {
            if rand::random::<f64>() < self.crossover_rate {
                // Crossover
                let max_dim_index = self.dimension - 1;
                let swap_rv = rng.gen_range(1..=max_dim_index);

                for j in 1..=max_dim_index {
                    if swap_rv == j {
                        let mut cross_vec_1: Vec<i32> =
                            population[i].get_action_vector()[0..j].to_vec();
                        cross_vec_1.extend_from_slice(
                            &population[i + 1].get_action_vector()[j..=max_dim_index],
                        );

                        let mut cross_vec_2: Vec<i32> =
                            population[i + 1].get_action_vector()[0..j].to_vec();
                        cross_vec_2.extend_from_slice(
                            &population[i].get_action_vector()[j..=max_dim_index],
                        );

                        let new_individual_1 = Arm::new(&cross_vec_1);
                        let new_individual_2 = Arm::new(&cross_vec_2);

                        crossover_pop.push(new_individual_1);
                        crossover_pop.push(new_individual_2);
                    }
                }
            } else {
                // No Crossover
                crossover_pop.push(population[i].clone());
                crossover_pop.push(population[i + 1].clone());
            }
        }

        crossover_pop
    }

    pub(crate) fn mutate(&self, population: &[Arm]) -> Vec<Arm> {
        let mut mutated_population = Vec::new();
        let mut seen = HashSet::new();
        let mut rng = rand::thread_rng();

        for individual in population.iter() {
            // Clone the action vector
            let mut new_action_vector = individual.get_action_vector().to_vec(); // Here I assumed `get_action_vector` returns a slice or Vec

            for (i, value) in new_action_vector.iter_mut().enumerate() {
                if rng.gen::<f64>() < self.mutation_rate {
                    let adjustment = Normal::new(
                        0.0,
                        self.mutation_span * (self.upper_bound[i] - self.lower_bound[i]) as f64,
                    )
                    .unwrap()
                    .sample(&mut rng);

                    *value = (*value as f64 + adjustment)
                        .max(self.lower_bound[i] as f64)
                        .min(self.upper_bound[i] as f64) as i32;
                }
            }

            let new_individual = Arm::new(new_action_vector.as_slice());

            if seen.insert(new_individual.clone()) {
                mutated_population.push(new_individual);
            }
        }

        mutated_population
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock optimization function for testing
    fn mock_opti_function(_vec: &[i32]) -> f64 {
        0.0
    }

    #[test]
    fn test_get_population_size() {
        let ga = GeneticAlgorithm::new(
            mock_opti_function,
            10,
            0.1,
            0.9,
            0.5,
            100,
            2,
            vec![0, 0],
            vec![10, 10],
        );
        assert_eq!(ga.get_population_size(), 10);
    }

    #[test]
    fn test_get_individuals() {
        let mut ga = GeneticAlgorithm::new(
            mock_opti_function,
            10,
            0.1,
            0.9,
            0.5,
            100,
            2,
            vec![0, 0],
            vec![10, 10],
        );
        assert_eq!(ga.get_individuals().len(), 10);
    }

    #[test]
    fn test_get_simulations_used() {
        let ga = GeneticAlgorithm::new(
            mock_opti_function,
            10,
            0.1,
            0.9,
            0.5,
            100,
            2,
            vec![0, 0],
            vec![10, 10],
        );
        assert_eq!(ga.get_simulations_used(), 0);
    }

    #[test]
    fn test_update_simulations_used() {
        let mut ga = GeneticAlgorithm::new(
            mock_opti_function,
            10,
            0.1,
            0.9,
            0.5,
            100,
            2,
            vec![0, 0],
            vec![10, 10],
        );
        ga.update_simulations_used(5);
        assert_eq!(ga.get_simulations_used(), 5);
    }

    #[test]
    fn test_budget_reached() {
        let mut ga = GeneticAlgorithm::new(
            mock_opti_function,
            10,
            0.1,
            0.9,
            0.5,
            100,
            2,
            vec![0, 0],
            vec![10, 10],
        );
        assert_eq!(ga.budget_reached(), false);
        ga.update_simulations_used(100);
        assert_eq!(ga.budget_reached(), true);
    }

    #[test]
    fn test_mutate() {
        let ga = GeneticAlgorithm::new(
            mock_opti_function,
            2,   // Two individuals in population
            1.0, // 100% mutation rate for demonstration
            0.9,
            1.0,
            100,
            2,
            vec![0, 0],
            vec![10, 10],
        );

        let initial_population = vec![Arm::new(&vec![1, 1]), Arm::new(&vec![2, 2])];

        let mutated_population = ga.mutate(&initial_population);

        // Assuming the mutation is deterministic and in the expected bounds, you'd check like this:
        for (i, individual) in mutated_population.iter().enumerate() {
            let init_vector = initial_population[i].get_action_vector();
            let mut_vector = individual.get_action_vector();

            for j in 0..ga.dimension {
                assert!(mut_vector[j] >= ga.lower_bound[j]);
                assert!(mut_vector[j] <= ga.upper_bound[j]);
            }

            assert_ne!(mut_vector, init_vector); // since mutation rate is 100%
        }
    }

    #[test]
    fn test_crossover() {
        let ga = GeneticAlgorithm::new(
            mock_opti_function,
            2, // Two individuals for simplicity
            0.1,
            1.0, // 100% crossover rate for demonstration
            0.5,
            100,
            10, // higher dimension for demonstration so low probability of crossover leading to identical individuals
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        );

        let initial_population = vec![
            Arm::new(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Arm::new(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
        ];

        let crossover_population = ga.crossover(&initial_population);

        // Since the crossover rate is 100%, the two individuals should not be identical to the original individuals
        assert_ne!(
            crossover_population[0].get_action_vector(),
            initial_population[0].get_action_vector()
        );
        assert_ne!(
            crossover_population[1].get_action_vector(),
            initial_population[1].get_action_vector()
        );
    }
}
