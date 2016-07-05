use node::{Column, Row, OwnedNode, NodeContents};
use node::{prepend_left};
use std::rc::Rc;
use std::slice;

pub struct Problem {
    root: OwnedNode,
    pub constraints: Vec<Column>,
    pub actions: Vec<Row>
}

impl Problem {
    pub fn new() -> Problem {
        Problem { constraints: Vec::new(), actions: Vec::new(), root: NodeContents::new() }
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
                let c = Column::new(i);
                prepend_left(&mut self.root, &c.root());
                self.constraints.push( c );
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

    // Count the number of inner cells in the entire matrix
    pub fn count_cells(&self) -> usize {
        return self.constraints.iter().map(|ref x| { x.get_count() }).fold(0, |x, y| { x + y })
    }

    /// Choose a column to cover from among those available.
    pub fn choose_column(&self) -> Option<usize> {
        let ref x =  self.root.borrow().right;
        let r2 = x.upgrade().unwrap();
        let rc = r2.borrow().column;
        rc
    }

    // Choose a row by removing all other actions that match at least one constraint.
    // pub fn choose_row(&self, row_index: usize) -> bool {
    //     let constraints = self.actions[row_index].iter();

    //     for c in constraints {
    //         // get the actual column
    //         let col = &self.constraints[c.upgrade().unwrap().borrow().column.unwrap()];

    //         // cover the column
    //         self.cover_column(col);
    //     }
    // }

    fn cover_row(&self, row_index: usize) {
    }

    /// Cover a column by remove each action that could remove that
    /// column, and remove the column from the header list.
    pub fn cover_column(&self, column_index: usize)  {
        let col = &self.constraints[column_index];

        col.cover_header();
        for n in col.iter() {
            ///
        }
    }


    pub fn all_constraints(&self) -> slice::Iter<Column> {
        self.constraints.iter()
    }
}
