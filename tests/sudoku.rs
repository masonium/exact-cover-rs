extern crate dancing_links;

use dancing_links::instances::sudoku::{sudoku_solver, sudoku_problem};
use dancing_links::{Solver};


#[test]
fn empty() {
    for i in 1..4 {
        let n = i*i;
        let p = sudoku_problem(n);
        let mut s = Solver::new(p.unwrap());

        assert_eq!(s.problem().num_constraints(), 3*n*n);

        let sol = s.first_solution();
        assert!(sol.is_some());
        assert!(sol.unwrap().len() == n*n);
    }
}

fn rotate_vec(v: &Vec<usize>, i: usize) -> Vec<usize> {
    let n = v.len();
    let mut r = Vec::with_capacity(n);
    for x in 0..n {
        r.push(v[(x+i)%n]);
    }
    r
}

#[test]
fn trivial() {
    let r: Vec<usize> = (1..10).collect();
    let s = sudoku_solver(&[&rotate_vec(&r, 0), &rotate_vec(&r, 3), &rotate_vec(&r, 6),
                            &rotate_vec(&r, 1), &rotate_vec(&r, 4), &rotate_vec(&r, 7),
                            &rotate_vec(&r, 2), &rotate_vec(&r, 5), &rotate_vec(&r, 8)]);
    if let Err(ref x) = s {
        println!("{}", x);
    }

    assert!(s.is_ok());
    assert!(s.unwrap().first_solution().is_some());
}

#[test]
fn easy1() {
    let r = [vec![7, 0, 5, 0, 6, 0, 0, 0, 0],
             vec![0, 3, 0, 7, 9, 0, 0, 5, 2],
             vec![0, 2, 0, 1, 3, 0, 0, 4, 6],
             vec![0, 0, 0, 2, 5, 0, 0, 0, 4],
             vec![2, 1, 0, 0, 0, 0, 0, 7, 8],
             vec![5, 0, 0, 0, 8, 1, 0, 0, 0],
             vec![9, 5, 0, 0, 2, 8, 0, 1, 0],
             vec![1, 6, 0, 0, 7, 3, 0, 8, 0],
             vec![0, 0, 0, 0, 1, 0, 3, 0, 9]];

    let rs: Vec<&[usize]> = r.iter().map(|ref x| x.as_slice()).collect();

    let s = sudoku_solver(&rs);

    if let Err(ref x) = s {
        println!("{}", x);
    }


    assert!(s.is_ok());
    assert!(s.unwrap().first_solution().is_some());
}

#[test]
fn evil1() {
    let r = [vec![0, 2, 0, 8, 0, 0, 0, 1, 0],
             vec![0, 6, 0, 0, 5, 0, 0, 0, 0],
             vec![5, 0, 7, 0, 0, 2, 0, 0, 0],
             vec![0, 0, 3, 0, 9, 7, 8, 0, 4],
             vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
             vec![4, 0, 1, 3, 2, 0, 9, 0, 0],
             vec![0, 0, 0, 4, 0, 0, 3, 0, 6],
             vec![0, 0, 0, 0, 7, 0, 0, 8, 0],
             vec![0, 1, 0, 0, 0, 9, 0, 5, 0]];

    let rs: Vec<&[usize]> = r.iter().map(|ref x| x.as_slice()).collect();

    let s = sudoku_solver(&rs);

    if let Err(ref x) = s {
        println!("{}", x);
    }


    assert!(s.is_ok());
    assert!(s.unwrap().first_solution().is_some());
}
