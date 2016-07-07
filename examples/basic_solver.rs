extern crate dancing_links;

use dancing_links::solver::Solver;
use dancing_links::problem::Problem;

fn main() {
    let mut p = Problem::new();
    p.add_action(&[0, 1, 2]);
    p.add_action(&[2, 4]);
    p.add_action(&[3, 4]);
}
