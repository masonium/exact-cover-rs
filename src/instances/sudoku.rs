use problem::{Problem};
use solver::{Solver};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SudokuAction {
    pub cell: usize,
    pub row: usize,
    pub col: usize
}

/// Locations are 0-indexed.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Location {
    Row(usize),
    Col(usize),
    Box(usize, usize)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SudokuConstraint {
    cell: usize,
    location: Location
}


impl SudokuAction {
    pub fn new(v: usize, r: usize, c: usize) -> SudokuAction {
        SudokuAction{ cell: v, row: r, col: c }
    }

    pub fn constraints(&self, box_size: usize) -> [SudokuConstraint; 3] {
        [SudokuConstraint{ cell: self.cell, location: Location::Row(self.row) },
         SudokuConstraint{ cell: self.cell, location: Location::Col(self.col) },
         SudokuConstraint{ cell: self.cell, location: Location::Box(self.row / box_size, self.col / box_size) }]
    }
}

pub type SudokuProblem = Problem<SudokuAction, SudokuConstraint>;
pub type SudokuSolver = Solver<SudokuAction, SudokuConstraint>;

fn isqrt(n: usize) -> usize {
    //// n is typically small.
    for i in 1..(n+1) {
        if i*i == n {
            return i
        }
        if i*i > n {
            return i-1
        }
    }
    return 0
}

/// Return a fully-specified sudoku problem of the given size.
pub fn sudoku_problem(n: usize) -> Option<SudokuProblem> {
    println!("{}: {}", n, isqrt(n));
    let mut p = Problem::new();
    let box_size = isqrt(n);
    for i in 1..(n+1) {
        for row in 0..n {
            for col in 0..n {
                let a = SudokuAction::new(i, row, col);
                p.add_action(a, &a.constraints(box_size));
            }
        }
    }
    Some(p)
}

/// Return a solver for a partially-filled sudoku problem.
pub fn sudoku_solver(cells: &[&[usize]]) -> Result<SudokuSolver, String> {
    // Verify the problem.
    let psize  = cells.len();

    for (i, arr) in cells.iter().enumerate() {
        if arr.len() != psize {
            return Err(format!("Size mismatch in cells: Row {} is of size {} (instead of {})", i, arr.len(), psize).to_string());
        }
        for (j, x) in arr.iter().enumerate() {
            if  *x > psize {
                return Err(format!("Invalid entry at ({}, {}): {} >= {}", i, j, *x, psize).to_string())
            }
        }
    }

    let p = sudoku_problem(psize);
    let mut s = Solver::new(p.unwrap());

    // Try to insert of all the actions as stated, returning
    // prematurely if any insertions fail.
    for (i, arr) in cells.iter().enumerate() {
        for (j, x) in arr.iter().enumerate() {
            if *x != 0 {
                let a =  SudokuAction::new(*x, i, j);
                if s.require_action(a).is_err() {
                    return Err(format!("Could not require action {:?}", a).to_string());
                }
            }
        }
    }

    Ok(s)
}