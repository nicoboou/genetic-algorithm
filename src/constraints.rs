pub struct Constraints {
    pub max_weight: u32,
    pub population_size: usize,
    pub generations: usize,
    pub mutation_rate: f64,
}

impl Constraints {
    pub fn new(max_weight: u32, population_size: usize, generations: usize, mutation_rate: f64) -> Self {
        Constraints {
            max_weight,
            population_size,
            generations,
            mutation_rate,
        }
    }
}
