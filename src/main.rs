mod constraints;
mod ga;
mod variables;

use ga::crossover::crossover;
use ga::fitness::fitness;
use ga::mutation::mutate;
use ga::population::initialize_population;
use ga::selection::select;

use constraints::Constraints;
use variables::get_items;

fn main() {
    // retrieving variables & constraints
    let items = get_items();
    let constraints = Constraints::new(80, 160, 70, 0.01);

    let mut population = initialize_population(constraints.population_size, items.len());

    for _ in 0..constraints.generations {
        let fitness_values: Vec<i32> = population
            .iter()
            .map(|chromosome| fitness(chromosome, &items, constraints.max_weight))
            .collect();

        let mut new_population = Vec::new();

        while new_population.len() < constraints.population_size {
            let (parent1, parent2) = select(&population, &fitness_values);
            let (mut child1, mut child2) = crossover(&parent1, &parent2);

            mutate(&mut child1, constraints.mutation_rate);
            mutate(&mut child2, constraints.mutation_rate);

            new_population.push(child1);
            new_population.push(child2);
        }

        population = new_population;
    }

    // Find the best solution in the final population
    let best_solution = population
        .iter()
        .max_by_key(|chromosome| fitness(chromosome, &items, constraints.max_weight))
        .unwrap();

    println!("Best solution: {:?}", best_solution);
}
