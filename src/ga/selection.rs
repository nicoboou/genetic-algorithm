// Selection is a process where individual chromosomes are chosen from a population for later breeding (crossover).
// Chose the common method of the "roulette wheel" selection, where probability of individual being selected is proportional to its fitness.

use rand::Rng;
use crate::ga::chromosome::Chromosome;

pub fn select(population: &[Chromosome], fitness_values: &[i32]) -> (Chromosome, Chromosome) {
    let total_fitness: i32 = fitness_values.iter().sum();
    let mut rng = rand::thread_rng();

    let select_individual = |rng: &mut rand::rngs::ThreadRng, total_fitness: i32| -> Chromosome {
        let mut wheel = rng.gen_range(0..total_fitness);
        let mut idx = 0;

        while wheel > 0 {
            wheel -= fitness_values[idx];
            idx += 1;
        }

        population[idx - 1].clone()
    };

    let parent1 = select_individual(&mut rng, total_fitness);
    let parent2 = select_individual(&mut rng, total_fitness);

    (parent1, parent2)
}