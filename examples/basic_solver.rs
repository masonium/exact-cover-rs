extern crate dancing_links;

use dancing_links::solver::Solver;
use dancing_links::problem::Problem;

fn main() {
    let mut p = Problem::new();
    p.add_action(&[0, 1, 2]);
    p.add_action(&[3, 4]);
    p.add_action(&[2, 4]);
    println!("Number of cells: {}", p.count_cells());

    for c in p.all_constraints() {
        for node in c.iter() {
            let n = node.upgrade().unwrap();
            print!("{} | ", *n.borrow());
        }
        println!("");
    }

    let mut solver = Solver { problem : p };
    let x = solver.first_solution();
}
