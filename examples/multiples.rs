extern crate exact_cover;

use exact_cover::{Problem};
use exact_cover::{Solver};

fn main() {
    let mut p = Problem::new();
    p.add_action(0, &[0, 1]);
    p.add_action(1, &[2, 3]);
    p.add_action(2, &[0, 3]);
    p.add_action(3, &[1, 2]);

    let solver = Solver::new(p);
    let mut iter = solver.into_iter();

    let first_sol = iter.next();
//    assert!(first_sol.is_some());
//    assert_eq!(first_sol.unwrap().len(), 2);
}
