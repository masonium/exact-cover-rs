extern crate exact_cover;

use exact_cover::{Problem};
use exact_cover::{Solver};

#[test]
fn first_of_many() {
    let mut p = Problem::new();
    p.add_action(0, &[0, 1]);
    p.add_action(1, &[2, 3]);
    p.add_action(2, &[0, 3]);
    p.add_action(3, &[1, 2]);

    let solver = Solver::new(p);
    let sol = solver.first_solution().unwrap();

    assert_eq!(sol.len(), 2);
}

fn test_prob() -> Problem<&'static str, usize> {
    let mut p = Problem::new();
    p.add_action("01", &[0, 1]);
    p.add_action("03", &[0, 3]);
    p.add_action("12", &[1, 2]);
    p.add_action("23", &[2, 3]);
    p.add_action("0", &[0]);
    p.add_action("1", &[1]);
    p.add_action("2", &[2]);
    p.add_action("3", &[3]);

    p
}

#[test]
fn count_multiple() {
    let iter = Solver::new(test_prob()).into_iter();
    assert_eq!(iter.count(), 7);
}

#[test]
fn first_in_iter() {
    let mut p = Problem::new();
    p.add_action(0, &[0, 1]);
    p.add_action(1, &[2, 3]);

    let solver = Solver::new(p);
    let mut iter = solver.into_iter();

    let first_sol = iter.next();
    assert!(first_sol.is_some());
    let fs = first_sol.unwrap();

    let real_solution = vec![0, 1];
    for i in real_solution.iter() {
        assert!(fs.iter().find(|x| {*x == i}).is_some());
    }
}
