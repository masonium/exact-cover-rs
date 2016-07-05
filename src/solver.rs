use problem::Problem;
use node::{WeakNode};

pub struct Solver {
    pub problem: Problem
}

impl Solver {
    /// Choose a column to cover from among those available.
    pub fn first_solution(&mut self) -> Option<Vec<usize>> {
        //let col: Vec<usize> = Vec::new();
        loop {
            let constraint = self.problem.choose_column();
            if let None = constraint {
                println!("found no valid constraints.");
                break;
            }

            let cindex = constraint.unwrap();
            println!("using constraint {}", cindex);
            let ref c = self.problem.constraints[cindex];

            self.problem.cover_column(cindex);

            // pick an action for the constraint to satisfy
            let actions = c.iter().filter_map(|ref x| { 
                let n = x.upgrade().unwrap();
                n.borrow().row });
            
            for action in actions {
                let r = &self.problem.actions[action];
                let columns = r.iter().map(|ref x| {
                    let n = x.upgrade().unwrap();
                    n.borrow().column.unwrap()
                });

            }
            break;
        }

        None
    }
}
