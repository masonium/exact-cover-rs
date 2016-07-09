# exact-cover #

`exact-cover` is a rust crate for solving exact cover
[exact cover](https://en.wikipedia.org/wiki/Exact_cover)
problems. Sudoku is probably the most well-known instance of an
exact-cover problem.


# Installing #
You can download the library via its github page, at
[https://github.com/masonium/exact-cover-rs](https://github.com/masonium/exact-cover-rs).

# Usage #

Here's an example program, located in `examples/subsets.rs`, which solves the detailed exact cover example from wikipedia.


	extern crate exact_cover;

	use exact_cover::{Problem, Solver};

	fn main() {
		// Create a new problem.
		let mut p = Problem::new();

		// The problem we define will be to find a collection of subsets
		// that form an exact cover for the full set {1, 2, ..., 7}. The
		// example is taken from Wikipedia. (https://en.wikipedia.org/wiki/Exact_cover#Detailed_example)

		// `add_action()` pairs a (named) action and a set of constraints
		// that the action satisfies. In this example, each constraint is
		// simply a number, which implicitly means that the subset
		// includes that number.
		p.add_action("A", &[1, 4, 7]);
		p.add_action("B", &[1, 4]);
		p.add_action("C", &[4, 5, 7]);
		p.add_action("D", &[3, 5, 6]);
		p.add_action("E", &[2, 3, 6, 7]);
		p.add_action("F", &[2, 7]);

		// Once we add the actions, create a solver to solve the problem.
		let solver = Solver::new(p);

		// first_solution returns the first solution to the problem, if it
		// exists. A solution is a vector of actions, referred to by name
		// as previously defined.
		let sol = solver.first_solution().unwrap();

		println!("{:?}", sol);
	}


The `examples/` folder contains other usage examples.

# Benchmarks #

# TODO #
The main features I plan to add are:
   * iterating through all solutions, instead of just the first one.
   * adding optional constraints, for the generalized exact cover problems
   * adding more built-in examples
   * benchmarking
