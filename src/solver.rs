use problem::Problem;
use node::{iter_row, column_index, row_index};

pub struct Solver {
    problem: Problem,
    partial_solution: Vec<usize>
}

impl Solver {
    pub fn new(problem: Problem) -> Solver {
        Solver { problem: problem, partial_solution: Vec::new() }
    }

    pub fn require_action(&mut self, action: usize) -> Result<(), String> {
        match self.problem.require_row(action) {
            Ok(r) => {
                self.partial_solution.push(action);
                Ok(r)
            },
            Err(s) => {
                Err(s)
            }
        }
    }

    pub fn first_solution(&mut self) -> Option<Vec<usize>> {
        self.first_solution_aux()
    }

    /// Choose a column to cover from among those available.
    fn first_solution_aux(&mut self) -> Option<Vec<usize>> {
        let constraint = self.problem.choose_column();
        if let None = constraint {
            return Some(self.partial_solution.clone());
        }

        let cindex = constraint.unwrap();
        
        if self.problem.constraints[cindex].count() == 0 {
            return None;
        }

        self.problem.cover_column(cindex);

        // pick an action for the constraint to satisfy
        let action_nodes = self.problem.constraints[cindex].iter();

        // Try that action, and return the solution to partial
        // problem, if possible.
        for action in action_nodes {
            let ri = row_index(&action).unwrap();
            self.partial_solution.push(ri);

            for c in iter_row(&action) {
                self.problem.cover_column(column_index(&c).unwrap())
            }

            let sol = self.first_solution_aux();


            for c in iter_row(&action).rev() {
                self.problem.uncover_column(column_index(&c).unwrap())
            }

            if let Some(x) = sol {
                return Some(x);
            }

            self.partial_solution.pop();
        }

        None
    }
}
