#![feature(slice_concat_ext)]

extern crate dancing_links;

use dancing_links::{Problem};
use dancing_links::{Solver};
use std::slice::SliceConcatExt;

fn main() {
    let mut p = Problem::new();
    p.add_action(0, &["1", "4", "7"]);
    p.add_action(1, &["1", "4"]);
    p.add_action(2, &["4", "5", "7"]);
    p.add_action(3, &["3", "5", "6"]);
    p.add_action(4, &["2", "3", "6", "7"]);
    p.add_action(5, &["2", "7"]);

    let mut solver = Solver::new(p);
    let sol = solver.first_solution().unwrap();
    let acts = sol.iter().map(|x| format!("{}", x) );
    let sol_str = acts.collect::<Vec<String>>().join(", ").to_string();
    println!("{}", sol_str);
}

