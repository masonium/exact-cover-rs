use problem::Problem;
use node::{iter_row, column_index, row_index};

pub struct Solver {
    pub problem: Problem
}

impl Solver {
    /// Choose a column to cover from among those available.
    pub fn first_solution(&mut self) -> Option<Vec<usize>> {
        let mut solution: Vec<usize> = Vec::new();

        loop {
            let constraint = self.problem.choose_column();
            if let None = constraint {
                println!("Found constraint with no actions.");
                break
            }

            let cindex = constraint.unwrap();
            println!("using constraint {}", cindex);
            self.problem.cover_column(cindex);

            // pick an action for the constraint to satisfy
            let action_nodes = self.problem.constraints[cindex].iter();

            // Try that action, and return the solution to partial
            // problem, if possible.
            for action in action_nodes {
                solution.push(row_index(&action).unwrap());

                for c in iter_row(&action, false).skip(1) {
                    self.problem.cover_column(column_index(&c).unwrap())
                }

                if let Some(x) = self.first_solution() {
                    return Some(x);
                }

                for c in iter_row(&action, true).skip(1) {
                    self.problem.uncover_column(column_index(&c).unwrap())
                }

                solution.pop();
            }
            break;
        }

        None
    }
}
