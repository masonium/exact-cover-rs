///
extern crate dancing_links;

use dancing_links::instances::sudoku::{SudokuSolver, sudoku_solver, fill_from_solution};
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

    sudoku_solver(&v).ok()
}

/// Read a sudoku from the command line and solve.
fn main() {
    let s  = read_sudoku();
    let sol = s.unwrap().first_solution();
    match sol {
        Some(a) => {
            let x = fill_from_solution(9, &a);
            for v in x {
                for c in v {
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
