extern crate dancing_links;

use dancing_links::instances::sudoku::{sudoku_solver, sudoku_problem};
use dancing_links::{Solver};


#[test]
fn empty() {
    for i in 1..4 {
        let n = i*i;
        let p = sudoku_problem(n);
        let mut s = Solver::new(p.unwrap());

        assert_eq!(s.problem().num_constraints(), 4*n*n);

        let sol = s.first_solution();
        assert!(sol.is_some());
        assert!(sol.unwrap().len() == n*n);
    }
}

// #[test]
// fn trivial() {
//     let r: Vec<usize> = (1..10).collect();
//     let v = r.iter().flat_map().collect();
//     let s = sudoku_solver(&v);
//     if let Err(ref x) = s {
//         println!("{}", x);
//     }

//     assert!(s.is_ok());
//     assert!(s.unwrap().first_solution().is_some());
// }

#[test]
fn easy1() {
    let r = [7, 0, 5, 0, 6, 0, 0, 0, 0,
             0, 3, 0, 7, 9, 0, 0, 5, 2,
             0, 2, 0, 1, 3, 0, 0, 4, 6,
             0, 0, 0, 2, 5, 0, 0, 0, 4,
             2, 1, 0, 0, 0, 0, 0, 7, 8,
             5, 0, 0, 0, 8, 1, 0, 0, 0,
             9, 5, 0, 0, 2, 8, 0, 1, 0,
             1, 6, 0, 0, 7, 3, 0, 8, 0,
             0, 0, 0, 0, 1, 0, 3, 0, 9];

    let s = sudoku_solver(&r);

    if let Err(ref x) = s {
        println!("{}", x);
    }


    assert!(s.is_ok());
    assert!(s.unwrap().first_solution().is_some());
}

#[test]
fn evil1() {
    let r = [0, 2, 0, 8, 0, 0, 0, 1, 0,
             0, 6, 0, 0, 5, 0, 0, 0, 0,
             5, 0, 7, 0, 0, 2, 0, 0, 0,
             0, 0, 3, 0, 9, 7, 8, 0, 4,
             0, 0, 0, 0, 0, 0, 0, 0, 0,
             4, 0, 1, 3, 2, 0, 9, 0, 0,
             0, 0, 0, 4, 0, 0, 3, 0, 6,
             0, 0, 0, 0, 7, 0, 0, 8, 0,
             0, 1, 0, 0, 0, 9, 0, 5, 0];

    let s = sudoku_solver(&r);

    if let Err(ref x) = s {
        println!("{}", x);
    }


    assert!(s.is_ok());
    assert!(s.unwrap().first_solution().is_some());
}
