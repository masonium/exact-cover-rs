extern crate dancing_links;

use dancing_links::problem::{Problem};
use dancing_links::solver::{Solver};

#[test]
fn partial_solve_fail() {
    let mut p = Problem::new();
    p.add_action(&[0]);
    p.add_action(&[0, 1, 2]);
    p.add_action(&[2, 4]);
    p.add_action(&[3, 4]);

    let mut solver = Solver::new(p);
    assert!(solver.require_action(1).is_ok());
    assert!(solver.require_action(2).is_err());
}

#[test]
fn partial_solve_succeses() {
    let mut p = Problem::new();
    p.add_action(&[0]);
    p.add_action(&[0, 1, 2]);
    p.add_action(&[2, 4]);
    p.add_action(&[3, 4]);

    let mut solver = Solver::new(p);
    assert!(solver.require_action(1).is_ok());
    assert!(solver.require_action(3).is_ok());
    let res: Vec<usize> = vec![1, 3];
    let sol = solver.first_solution();
    assert_eq!(sol.unwrap().len(), res.len());
}
