extern crate exact_cover;

use exact_cover::{Problem};
use exact_cover::{Solver};

fn main() {
    let mut p = Problem::new();
    p.add_action("01", &[0, 1]);
    p.add_action("03", &[0, 3]);
    p.add_action("12", &[1, 2]);
    p.add_action("23", &[2, 3]);
    p.add_action("0", &[0]);
    p.add_action("1", &[1]);
    p.add_action("2", &[2]);
    p.add_action("3", &[3]);


    let solver = Solver::new(p);
    let iter = solver.into_iter();

    for sol in iter {
        println!("{:?}", sol);
    }
}
