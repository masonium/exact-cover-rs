extern crate exact_cover;

use exact_cover::instances::sudoku::{SudokuSolver, sudoku_solver, solution_as_matrix};
use std::io::{stdin, BufRead, BufReader};

fn read_sudoku() -> Option<SudokuSolver> {
    let br = BufReader::new(stdin());
    let mut v: Vec<usize> = Vec::new();
    for line in br.lines() {
        if line.is_err() {
            break;
        }
        for d in line.unwrap().chars().filter_map(|x| x.to_digit(10)) {
            v.push(d as usize)
        }
    }

    if v.len() != 81 {
        return None
    }

    // This crate also contains implemtations for specific problems.
    // For instance, `sudoku_solver()` takes a 1-d slice of usizes
    // representing a sudoku problem and returns a `Solver` object
    // that will generate solutions.
    sudoku_solver(&v).ok()
}

/// Read a sudoku from the command line and solve.
fn main() {

    let s  = read_sudoku();
    let sol = s.unwrap().first_solution();
    match sol {
        Some(a) => {
            let x = solution_as_matrix(9, &a);
            for (i, v) in x.iter().enumerate() {
                if i == 3 || i == 6 {
                    println!("───┼───┼───");
                }
                for (j, c) in v.iter().enumerate() {
                    if j == 3 || j == 6 {
                        print!("│");
                    }
                    print!("{}", c);
                }
                println!("");
            }
        },
        None => {
            println!("No solution found.");
        }
    }
}
