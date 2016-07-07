extern crate dancing_links;

use dancing_links::problem::{Problem};
use dancing_links::solver::{Solver};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct BasicAction(usize);

#[test]
fn partial_solve_fail() {
    let mut p = Problem::new();
    p.add_action(BasicAction(0), &[0]);
    p.add_action(BasicAction(1), &[0, 1, 2]);
    p.add_action(BasicAction(2), &[2, 4]);
    p.add_action(BasicAction(3), &[3, 4]);

    let mut solver = Solver::new(p);
    assert!(solver.require_action(BasicAction(1)).is_ok());
    assert!(solver.require_action(BasicAction(2)).is_err());
}

#[test]
fn partial_solve_succeses() {
    let mut p = Problem::new();
    p.add_action(0, &["a"]);
    p.add_action(1, &["a", "b", "c"]);
    p.add_action(2, &["c", "e"]);
    p.add_action(3, &["d", "e"]);

    let mut solver = Solver::new(p);
    assert!(solver.require_action(1).is_ok());
    assert!(solver.require_action(3).is_ok());
    let res: Vec<usize> = vec![1, 3];
    let sol = solver.first_solution();
    assert_eq!(sol.unwrap().len(), res.len());
}
