extern crate exact_cover;

use exact_cover::instances::sudoku::{SudokuSolver, sudoku_solver, solution_as_matrix};
use std::io::{stdin};

fn read_sudoku() -> (String, Option<SudokuSolver>) {
    let mut line = String::new();
    if stdin().read_line(&mut line).is_err() {
        return (line, None)
    }

    let v: Vec<usize> = line.chars().map(|x| if x == '.' { '0' } else { x })
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as usize).collect();

    if v.len() != 81 {
        return (line, None)
    }

    // This crate also contains implemtations for specific problems.
    // For instance, `sudoku_solver()` takes a 1-d slice of usizes
    // representing a sudoku problem and returns a `Solver` object
    // that will generate solutions.
    (line, sudoku_solver(&v).ok())
}

/// Read a sudoku from the command line and solve.
fn main() {

    loop {
        let (s, solver) = read_sudoku();
        if solver.is_none() {
            break;
        }
        let sol = solver.unwrap().first_solution();
        match sol {
            Some(a) => {
                print!("{}", s);
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
}
