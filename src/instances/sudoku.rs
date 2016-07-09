use problem::{Problem};
use solver::{Solver};

/// A `SudokuAction` is filling a particular square (identified by
/// `row` and `column`) with a particular number. An nxn sudoku will
/// have n^3 possible actions, of which n^2 form a particular solution.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SudokuAction {
    pub cell: usize,
    pub row: usize,
    pub col: usize
}

/// Constraint `Location`s are 0-indexed.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Location {
    Row(usize),
    Col(usize),
    Box(usize, usize)
}

/// A `Constraint` encodes the existence and uniqueness constraints
/// that a sudoku solution must satisfy.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SudokuConstraint {
    /// Existence constraints enforce that every square must be filled in.
    Existence(usize, usize),

    /// Unique constraints that enforce that each number (from 1, ..,
    /// n) must be occurs exactly once in each column, row, and block.
    Uniqueness(usize, Location),
}

impl SudokuAction {
    pub fn new(v: usize, r: usize, c: usize) -> SudokuAction {
        SudokuAction{ cell: v, row: r, col: c }
    }

    pub fn constraints(&self, box_size: usize) -> [SudokuConstraint; 4] {
        [SudokuConstraint::Uniqueness(self.cell, Location::Row(self.row)),
         SudokuConstraint::Uniqueness(self.cell, Location::Col(self.col)),
         SudokuConstraint::Uniqueness(self.cell, Location::Box(self.row / box_size, self.col / box_size)),
         SudokuConstraint::Existence(self.row, self.col)]
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

/// Return a fully-specified sudoku problem of the given size `n`. `n`
/// must be a perfect square; otherwise, `sudoku_problem()` will
/// return None.
pub fn sudoku_problem(n: usize) -> Option<SudokuProblem> {
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
pub fn sudoku_solver(cells: &[usize]) -> Result<SudokuSolver, String> {
    // Verify the problem.
    let n = cells.len();
    let psize = isqrt(n);
    if psize * psize != n {
        return Err("Not a square array".to_string());
    }

    for (i, arr) in cells.chunks(psize).enumerate() {
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
    for (i, arr) in cells.chunks(psize).enumerate() {
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

/// Given a solution (as a list of actions), return an vector of vectors that
/// represents the solved sudoku.
pub fn solution_as_matrix(n: usize, actions: &[SudokuAction]) -> Vec<Vec<usize>> {
    let mut sol: Vec<Vec<usize>> = (0..n).map(|_| vec![0; n]).collect();
    for action in actions {
        sol[action.row][action.col] = action.cell;
    }
    sol
}
