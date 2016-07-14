use node::{OwnedNode, WeakNode};
use problem::{Problem, Constraint, Action};
use iter::{iter_col, ColumnIterator};
use cover::{try_cover_column, cover_column, uncover_column, try_cover_row, cover_row, uncover_row};

/// A `Solver` consumes a problem and computes solutions to the exact
/// cover problem.
pub struct Solver<A: Action, C: Constraint> {
    problem: Problem<A, C>,
    partial_solution: Vec<A>,
}

struct FrameState {
    iter: ColumnIterator,
    column: OwnedNode,
    row: Option<WeakNode>
}

impl Drop for FrameState {
    fn drop(&mut self) {
        // uncover the column
        uncover_column(&self.column);

        // uncover the row, if there is one
        if let Some(ref node) = self.row {
            uncover_row(node);
        }
    }
}

pub struct SolutionIterator<A: Action, C: Constraint> {
    problem: Problem<A, C>,
    partial: Vec<A>,
    current_solution: Vec<A>,
    iter_stack: Vec< FrameState >,
    running: bool
}

impl <A: Action, C: Constraint> SolutionIterator<A, C> {
    pub fn new(problem: Problem<A, C>) -> SolutionIterator<A, C> {
        Self::from_solver(Solver::new(problem))
    }

    pub fn from_solver(solver: Solver<A, C>) -> SolutionIterator<A, C> {
        SolutionIterator { problem: solver.problem, 
                           current_solution: solver.partial_solution.clone(),
                           partial: solver.partial_solution, 
                           iter_stack: Vec::new(),
                           running: false}
    }

    // If init returns a solution, that's the only solution
    fn init(&mut self) -> Option<Vec<A>> {
        let c = self.problem.choose_column();
        match c {
            None =>  Some(self.partial.clone()),
            Some(c) => {
                if c.borrow().get_count().unwrap() > 0 {
                    cover_column(&c);
                    self.iter_stack.push(FrameState { iter: iter_col(&c), 
                                                      column: c.clone(),
                                                      row: None })
                }
                None
            }
        }
    }
}

impl<A: Action, C: Constraint> Iterator for SolutionIterator<A, C>  {
    type Item = Vec<A>;


    fn next(&mut self) -> Option<Vec<A>> {
        if !self.running {
            self.init();
            self.running = true;
        }

        let mut s = None;

        // At each step, take the next action in the iterator. Try to push a new state onto the column.
        while !self.iter_stack.is_empty() {
            let mut r = self.iter_stack.pop().unwrap();

            // Take the next action.
            if let Some(action_node) =  r.iter.next() {
                let a = self.problem.get_action(&action_node);
                {
                    let sa = action_node.upgrade().unwrap();
                    println!("trying action {}", sa.borrow().get_row().unwrap());
                }

                // put our frame back on the stack
                self.iter_stack.push(r);

                // add the action the current solution
                self.current_solution.push(a);

                cover_row(&action_node);

                // Choose a new constraint
                let c = self.problem.choose_column();

                match c {
                    // If there's no column to choose, we've found a result.
                    None => {
                        println!("found result.");
                        s = Some(self.current_solution.clone());

                        // We need to uncover the row ourselves,
                        // since we don't FrameState to do it for
                        // us.
                        uncover_row(&action_node);
                    },
                    // Otherwise, check to see if there are still options left.
                    Some(c) => {
                        println!("new column.");
                        // If there are, push a new frame.
                        let num_actions = c.borrow().get_count().unwrap();
                        if num_actions > 0 {
                            cover_column(&c);
                            self.iter_stack.push(FrameState { iter: iter_col(&c), 
                                                              column: c.clone(),
                                                              row: Some(action_node) });
                        } else {
                            // otherwise, we still need to uncover the row.
                            uncover_row(&action_node);
                        }
                    }
                }
            }
            // if we found a solution during the iteration, return it
            if s.is_some() {
                return s;
            }
        }
        None
    }
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
            Ok(_) => {
                self.partial_solution.push(action);
                Ok(())
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

impl<A: Action, C: Constraint> IntoIterator for Solver<A, C> {
    type Item = Vec<A>;
    type IntoIter = SolutionIterator<A, C>;

    fn into_iter(self) -> Self::IntoIter {
        SolutionIterator::from_solver(self)
    }
}

