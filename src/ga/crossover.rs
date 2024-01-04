use rand::Rng;
use crate::ga::chromosome::Chromosome;

pub fn crossover(parent1: &Chromosome, parent2: &Chromosome) -> (Chromosome, Chromosome) {
    let mut rng = rand::thread_rng();
    let crossover_point = rng.gen_range(0..parent1.len());
    let mut child1 = parent1[..crossover_point].to_vec();
    let mut child2 = parent2[..crossover_point].to_vec();

    child1.extend_from_slice(&parent2[crossover_point..]);
    child2.extend_from_slice(&parent1[crossover_point..]);

    (child1, child2)
}