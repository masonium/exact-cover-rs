extern crate dancing_links;

use dancing_links::{Problem};
use dancing_links::{Solver};

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
fn full_presolve() {
    let mut p = Problem::new();
    p.add_action(0, &["a"]);
    p.add_action(1, &["a", "b", "c"]);
    p.add_action(2, &["c", "e"]);
    p.add_action(3, &["d", "e"]);

    let mut solver = Solver::new(p);
    assert!(solver.require_action(1).is_ok());
    assert!(solver.require_action(3).is_ok());

    let res: Vec<usize> = vec![1, 3];
    let sol = solver.first_solution().unwrap();
    assert_eq!(sol.len(), res.len());

    for x in &res {
        assert!(sol.iter().find(|y| *y == x).is_some())
    }
}

#[test]
fn partial_presolve() {
    let mut p = Problem::new();
    p.add_action(0, &["a"]);
    p.add_action(1, &["a", "b", "c"]);
    p.add_action(2, &["c", "e"]);
    p.add_action(3, &["d", "e"]);

    let mut solver = Solver::new(p);
    assert!(solver.require_action(1).is_ok());

    let res: Vec<usize> = vec![1, 3];
    let sol = solver.first_solution().unwrap();
    assert_eq!(sol.len(), res.len());

    for x in &res {
        assert!(sol.iter().find(|y| *y == x).is_some())
    }
}
