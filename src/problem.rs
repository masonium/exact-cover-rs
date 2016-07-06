use node::{Column, Row, OwnedNode, NodeContents, iter_row};
use node::{prepend_left};
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

        // extend the constraint list to accomodate all constraints, if necessary
        if self.constraints.len() < max_constraint + 1 {
            self.constraints.reserve(max_constraint + 1);
            for i in self.constraints.len() .. max_constraint + 1 {
                let c = Column::new(i);
                prepend_left(&mut self.root, &c.root());
                self.constraints.push(c);
            }
        }

        // Create and collect new nodes for each constraint.
        let nodes = c.iter().map(|x| self.constraints[*x].append_new()).collect();

        // create a row from those nodes
        let new_id  = self.actions.len();
        self.actions.push(Row::new(nodes, new_id));
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

    /// Cover a column by remove each action that could remove that
    /// column, and remove the column from the header list.
    pub fn cover_column(&mut self, column_index: usize) {
        let mut col = &mut self.constraints[column_index];

        col.cover_header();

        for r in col.iter() {
            // For every node in the row (except the one from this
            // constraing), remove the node from its column and
            // decrement the corresponding count
            for n in iter_row(&r, false).skip(1) {
                let sn = n.upgrade().unwrap();
                sn.borrow_mut().remove_from_column();
                col.dec_count();
            }
        }
    }

    /// Uncover a column, repairing all links in reverse order.
    pub fn uncover_column(&mut self, column_index: usize) {
        let mut col = &mut self.constraints[column_index];

        // for r in col.iter() {
        //     // For every node in the row (except the one from this
        //     // constraing), remove the node from its column and
        //     // decrement the corresponding count
        //     for n in iter_row(&r, false).skip(1) {
        //         let sn = n.upgrade().unwrap();
        //         sn.borrow_mut().remove_from_column();
        //         col.dec_count();
        //     }
        // }

        col.uncover_header();
    }

    pub fn all_constraints(&self) -> slice::Iter<Column> {
        self.constraints.iter()
    }
}
