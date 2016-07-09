extern crate exact_cover;

use exact_cover::{Problem, Solver};

fn main() {
    const NUM_COLUMNS: usize = 7;

    // This problem is equivalent to the problem in subsets.rs.
    // However, we express it directly as the matrix. We want to find
    // a subset of rows such that every column has exactly one 1.
    let matrix = [1, 0, 0, 1, 0, 0, 1,
                  1, 0, 0, 1, 0, 0, 0,
                  0, 0, 0, 1, 1, 0, 1,
                  0, 0, 1, 0, 1, 1, 0,
                  0, 1, 1, 0, 0, 1, 1,
                  0, 1, 0, 0, 0, 0, 1];

    let mut p = Problem::new();
    for (i, r) in matrix.chunks(NUM_COLUMNS).enumerate() {
        let column_indices: Vec<usize> = r.iter().enumerate().filter_map(|(i, c)| if *c == 1 { Some(i) } else { None }).collect();
        p.add_action(i, &column_indices);
    }

    let mut solver = Solver::new(p);
    let sol = solver.first_solution().unwrap();
    println!("{:?}", sol);

    // We can also specify that some actions must exist in the final solution.
    solver.require_action(1).ok();
    let sol = solver.first_solution().unwrap();
    println!("{:?}", sol);

    // If we specifiy actions that are a-priori wrong, the method will
    // fail (and the solver will not change state.)
    assert!(solver.require_action(0).is_err());
    let sol = solver.first_solution().unwrap();
    println!("{:?}", sol);
}
