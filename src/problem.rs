use node::{Column, Row};
use std::rc::Rc;

#[derive(Default)]
pub struct Problem {
    constraints: Vec<Column>,
    actions: Vec<Row>
}

impl Problem {
    pub fn new() -> Problem {
        Problem { constraints: Vec::new(), actions: Vec::new() }
    }

    /// Add a new action, creating additional constraints on demand
    pub fn add_action(&mut self, c: &[usize]) {
        // Ignore actions that don't satisfy constraints
        if c.is_empty() {
            return
        }

        let max_constraint = *c.iter().max().unwrap() as usize;

        // extend the constraint list to accomodate all constraints
        if self.constraints.len() < max_constraint + 1 {
            self.constraints.reserve(max_constraint + 1);
            for i in self.constraints.len() .. max_constraint + 1 {
                self.constraints.push( Column::new(i) )
            }
        }
        let mut row = Row::new(self.actions.len());

        for col in c {
            // Add a new node for each constraint, and attach it to the corresponding column
            let n = row.append_new();
            self.constraints[*col].append(&mut Rc::downgrade(&n));
        }

        self.actions.push(row);
    }

    pub fn count_cells(&self) -> usize {
        return self.constraints.iter().map(|ref x| { x.get_count() }).fold(0, |x, y| { x + y })
    }
}
