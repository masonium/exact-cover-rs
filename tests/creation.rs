extern crate dancing_links;

use dancing_links::problem::{Problem};
use dancing_links::solver::{Solver};

#[test]
fn create_problem() {
    let mut p = Problem::new();
    p.add_action(&[0, 1, 2]);
    assert_eq!(p.count_cells(), 3);
    p.add_action(&[3, 4]);
    assert_eq!(p.count_cells(), 5);
    p.add_action(&[2, 4]);
    assert_eq!(p.count_cells(), 7);
    p.assert_header_counts();
}

#[test]
fn solve_problem() {
    let mut p = Problem::new();
    p.add_action(&[0, 1, 2]);
    p.add_action(&[3, 4]);
    p.add_action(&[2, 4]);
    assert_eq!(p.count_cells(), 7);

    let mut solver = Solver::new(p);
    solver.first_solution();
}
