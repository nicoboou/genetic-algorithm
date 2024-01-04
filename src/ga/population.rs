use crate::ga::chromosome::Chromosome;
use rand::Rng;

pub fn initialize_population(size: usize, chromosome_length: usize) -> Vec<Chromosome> {
    let mut population = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        let chromosome = (0..chromosome_length)
            .map(|_| rng.gen())
            .collect();
        population.push(chromosome);
    }

    population
}
