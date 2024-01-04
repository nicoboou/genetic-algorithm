# Genetic Algorithm for the Knapsack Problem

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)

This project implements a genetic algorithm to solve the Knapsack Problem, a classic problem in combinatorial optimization. The goal is to determine the most valuable combination of items to include in a knapsack without exceeding its weight capacity.

## Features

- **Modular Code Structure**: Organized into separate modules for different aspects of the genetic algorithm, such as selection, crossover, mutation, and fitness calculation.
- **Customizable Parameters**: Easy adjustment of algorithm parameters like population size, mutation rate, and number of generations.
- **Knapsack Problem Implementation**: Demonstrates the application of genetic algorithms to a well-known optimization problem.

## Project Structure

- `src/main.rs`: The main entry point of the program.
- `src/item.rs`: Defines the `Item` struct.
- `src/ga/`: Contains modules related to the genetic algorithm.
  - `chromosome.rs`: Represents a solution.
  - `fitness.rs`: Fitness function for the algorithm.
  - `selection.rs`: Selection process for breeding.
  - `crossover.rs`: Crossover function for generating new offspring.
  - `mutation.rs`: Mutation function to introduce variability.
- `src/constraints.rs`: Defines the constraints for the Knapsack Problem.
- `src/variables.rs`: Contains the definition and initialization of items.

## Usage

1. **Setup**: Clone the repository and navigate to the project directory.
2. **Build**: Run `cargo build` to compile the project.
3. **Run**: Execute the program with `cargo run`.

## Customization

- **Items**: Modify the items in `src/variables.rs` to experiment with different sets of items for the knapsack.
- **Constraints**: Adjust the genetic algorithm parameters in `src/constraints.rs` like `max_weight`, `population_size`, `generations`, and `mutation_rate`.

## Dependencies

- `rand`: Used for generating random numbers in various parts of the algorithm.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a pull request or create an issue for any improvements or bug fixes.
