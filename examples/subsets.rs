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
