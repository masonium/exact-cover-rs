use problem::{Problem, Constraint, Action};
use iter::{iter_col};
use cover::{try_cover_column, try_cover_row};

/// A `Solver` consumes a problem and computes solutions to the exact
/// cover problem.
pub struct Solver<A: Action, C: Constraint> {
    problem: Problem<A, C>,
    partial_solution: Vec<A>,
}

impl<A: Action, C: Constraint> Solver<A, C> {
    pub fn new(problem: Problem<A, C>) -> Solver<A, C> {
        Solver { problem: problem, partial_solution: Vec::new() }
    }

    pub fn problem(&self) -> &Problem<A, C> {
        &self.problem
    }

    /// Specify that an action must be present in the final solution.
    ///
    /// If no solution contains the set of required actions, then any
    /// solution-returning method will return no solution, even if
    /// another solution (that doesn't contain the require actions)
    /// would otherewise exits.
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

    /// Return a solution to the problem that includes any previously
    /// required actions (set via `require_actions()`), if one
    /// exists. 
    ///
    /// 'First' is an arbitrary qualifier; there are no guarantees
    /// that, for instance, the lexographically smallest action-set is
    /// returned. It is only guaranteed that, if at least one solution
    /// exists, a solution will be returned.
    pub fn first_solution(&self) -> Option<Vec<A>> {
        let mut sol: Vec<A> = Vec::new();
        if self.first_solution_aux(&mut sol){
            Some(sol)
        } else {
            None
        }
    }

    fn first_solution_aux(&self, solution: &mut Vec<A>) -> bool {
        let (_tc, action_nodes) = {
            let constraint = self.problem.choose_column();
            if let None = constraint {
                solution.extend_from_slice(&self.partial_solution);
                return true;
            }
            
            let con = constraint.unwrap();

            if con.borrow().get_count().unwrap() == 0 {
                return false;
            }

            // pick an action for the constraint to satisfy
            (try_cover_column(&con), iter_col(&con))
        };

        // Try that action, and return the solution to partial
        // problem, if possible.
        for action in action_nodes {
            let a = self.problem.get_action(&action);

            solution.push(a);

            let _cover = try_cover_row(&action);
            let sol = self.first_solution_aux(solution);

            if sol {
                return true;
            }

            solution.pop();
        }

        false
    }
}
