use problem::Problem;
use node::{iter_row, column_index};
use std::hash::Hash;
use std::fmt::Debug;

pub struct Solver<A: Debug + Copy + Eq + Hash, C: Debug + Clone + Hash + Eq> {
    problem: Problem<A, C>,
    partial_solution: Vec<A>
}

impl<A: Debug + Copy + Eq + Hash, C: Debug + Clone + Hash + Eq> Solver<A, C> {
    pub fn new(problem: Problem<A, C>) -> Solver<A, C> {
        Solver { problem: problem, partial_solution: Vec::new() }
    }

    pub fn require_action(&mut self, action: A) -> Result<(), String> {
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

    /// Return the first solution. .
    pub fn first_solution(&mut self) -> Option<Vec<A>> {
        self.problem.remaining_header_counts();
        self.first_solution_aux()
    }

    fn first_solution_aux(&mut self) -> Option<Vec<A>> {
        let cindex = {
            let constraint = self.problem.choose_column();
            if let None = constraint {
                return Some(self.partial_solution.clone());
            }
            
            constraint.unwrap().id()
        }; 
       

        if self.problem.constraints[cindex].count() == 0 {
            let con = self.problem.constraints[cindex].constraint();
            println!("No actions for {:?}", con);
            return None;
        }

        self.problem.cover_column(cindex);
        println!("Covering {:?}", self.problem.constraints[cindex].constraint());
        self.problem.remaining_header_counts();

        // pick an action for the constraint to satisfy
        let action_nodes = self.problem.constraints[cindex].iter();

        // Try that action, and return the solution to partial
        // problem, if possible.
        for action in action_nodes {
            let a = self.problem.get_action(&action);
            println!("Trying {:?}", a);

            self.partial_solution.push(a);

            for c in iter_row(&action) {
                self.problem.cover_column(column_index(&c).unwrap())
            }
            self.problem.remaining_header_counts();

            let sol = self.first_solution_aux();

            for c in iter_row(&action).rev() {
                self.problem.uncover_column(column_index(&c).unwrap())
            }
            self.problem.remaining_header_counts();
            if let Some(x) = sol {
                return Some(x);
            }

            self.partial_solution.pop();
        }

        None
    }
}
