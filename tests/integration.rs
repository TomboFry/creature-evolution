extern crate cmp6102;
extern crate rand;

mod common;

use cmp6102::optimisationmethods::OptimisationMethod;
use cmp6102::optimisationmethods::genetic_algorithm::GeneticAlgorithm;
use cmp6102::optimisationmethods::simulated_annealing::SimulatedAnnealing;
use cmp6102::optimisationmethods::hill_climbing::HillClimbing;

/// Tests all three Optimisation Methods and ensures they all succeeded
/// with the same number of generations and creatures
#[test]
fn three_opt_methods() {
	// Setup the constants to use in this specific test
	let generation_count = 200;
	let population_size = 200;
	let print_data = true;

	let pop = common::init(population_size);
	let mut om: Vec<Box<OptimisationMethod>> = Vec::with_capacity(3);

	// Clone the population for the first two, then move the ownership on the
	// final OM, as we won't need it anymore.
	om.push(GeneticAlgorithm::new(pop.clone(), print_data));
	om.push(SimulatedAnnealing::new(pop.clone(), print_data));
	om.push(HillClimbing::new(pop, print_data));

	// Run the specified number of generations on each OM
	for idx in 0 .. om.len() {
		for _ in 0 .. generation_count {
			if om[idx].generation_single().is_err() {
				// If any of the optimisation methods fail, fail the whole test
				assert!(false);
			}
		}
	}

	// Make sure each optimisation method has the same number of creatures
	// and generations completed.
	for idx in 0 .. om.len() {
		let data = om[idx].get_data();
		assert_eq!(data.gen, generation_count);
		assert_eq!(
			data.generations[generation_count - 1].creatures.len(),
			population_size
		);
	}
}