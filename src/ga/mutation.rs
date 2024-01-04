use rand::Rng;
use crate::ga::chromosome::Chromosome;

pub fn mutate(chromosome: &mut Chromosome, mutation_rate: f64) {
    let mut rng = rand::thread_rng();

    for gene in chromosome.iter_mut() {
        if rng.gen::<f64>() < mutation_rate {
            *gene = !*gene; // Flip the gene value
        }
    }
}