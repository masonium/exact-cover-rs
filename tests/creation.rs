extern crate exact_cover;

use exact_cover::{Problem};
use exact_cover::{Solver};

#[test]
fn solve_problem() {
    let mut p = Problem::new();
    p.add_action(0, &[0, 1, 2]);
    p.add_action(1, &[3, 4]);
    p.add_action(2, &[2, 4]);

    let solver = Solver::new(p);
    assert!(solver.first_solution().is_some());
}
