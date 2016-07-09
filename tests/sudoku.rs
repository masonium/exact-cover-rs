extern crate dancing_links;

use dancing_links::instances::sudoku::{sudoku_solver, sudoku_problem, SudokuAction, solution_as_matrix};
use dancing_links::{Solver};


#[test]
fn empty() {
    for i in 1..4 {
        let n = i*i;
        let p = sudoku_problem(n);
        let s = Solver::new(p.unwrap());

        assert_eq!(s.problem().num_constraints(), 4*n*n);

        let sol = s.first_solution();
        assert!(sol.is_some());
        assert!(sol.unwrap().len() == n*n);
    }
}

fn solution_as_array(sol: &Vec<SudokuAction>) -> Vec<usize> {
    solution_as_matrix(9, &sol).iter().flat_map(|x| x.iter() ).map(|x| *x).collect()
}

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

    let real_solution = [7, 4, 5, 8, 6, 2, 9, 3, 1,
                         6, 3, 1, 7, 9, 4, 8, 5, 2,
                         8, 2, 9, 1, 3, 5, 7, 4, 6,
                         3, 9, 8, 2, 5, 7, 1, 6, 4,
                         2, 1, 6, 3, 4, 9, 5, 7, 8,
                         5, 7, 4, 6, 8, 1, 2, 9, 3,
                         9, 5, 3, 4, 2, 8, 6, 1, 7,
                         1, 6, 2, 9, 7, 3, 4, 8, 5,
                         4, 8, 7, 5, 1, 6, 3, 2, 9];

    let s = sudoku_solver(&r);

    if let Err(ref x) = s {
        println!("{}", x);
    }


    assert!(s.is_ok());
    let sol = s.unwrap().first_solution().unwrap();
    let x = solution_as_array(&sol);

    for i in 0..81 {
        assert_eq!(x[i], real_solution[i]);
    }
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

    let real_solution = [3, 2, 9, 8, 6, 4, 5, 1, 7,
                         1, 6, 8, 7, 5, 3, 2, 4, 9,
                         5, 4, 7, 9, 1, 2, 6, 3, 8,
                         6, 5, 3, 1, 9, 7, 8, 2, 4,
                         7, 9, 2, 5, 4, 8, 1, 6, 3,
                         4, 8, 1, 3, 2, 6, 9, 7, 5,
                         2, 7, 5, 4, 8, 1, 3, 9, 6,
                         9, 3, 6, 2, 7, 5, 4, 8, 1,
                         8, 1, 4, 6, 3, 9, 7, 5, 2];

    let s = sudoku_solver(&r);

    if let Err(ref x) = s {
        println!("{}", x);
    }


    assert!(s.is_ok());
    let sol = s.unwrap().first_solution().unwrap();
    let res = solution_as_array(&sol);
    for i in 0..81 {
        assert_eq!(res[i], real_solution[i]);
    }

}
