pub use column::{Column};

use node::{Row, WeakNode, OwnedNode, NodeContents, iter_row, column_index};
use node::{prepend_left};
use std::slice;
use std::rc::{Rc};


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
        {
            if self.constraints.len() < max_constraint + 1 {
                self.constraints.reserve(max_constraint + 1);
                for i in self.constraints.len() .. max_constraint + 1 {
                    let c = Column::new(i);
                    prepend_left(&mut self.root, &c.root());
                    self.constraints.push(c);
                }
            }
        }

        // Create and collect new nodes for each constraint.
        let nodes = c.iter().map(|x| {
            self.constraints[*x].append_new()
        }).collect();

        // create a row from those nodes
        let new_id  = self.actions.len();
        self.actions.push(Row::new(nodes, new_id));
    }

    // Count the number of inner cells in the entire matrix
    pub fn count_cells(&self) -> usize {
        return self.constraints.iter().map(|ref x| { x.get_count() }).fold(0, |x, y| { x + y })
    }

    /// Choose the column with the smallest count
    pub fn choose_column(&self) -> Option<usize> {
        iter_row(&Rc::downgrade(&self.root))
            .map( |node| self.get_column(&node) )
            .min_by_key( |ref c| c.count() )
            .map( |ref c| c.id())
    }

    fn get_column(&self, row_node: &WeakNode) -> &Column {
        let s = row_node.upgrade().unwrap();
        let ci = s.borrow().column.unwrap();
        &self.constraints[ci]
    }

    /// Require that a given action be part of the solution
    pub fn require_row(&mut self, action: usize) -> Result<(), String> {
        let iter = {
            let act: &Row = &(self.actions[action]);

            if let Some(c) = act.iter().map(|node| { self.get_column(&node) }).find(|c| { c.is_already_chosen() })
            {
                return Err(format!("Could not require row; Constraint {} already satisfied", c.id()));
            }
            act.iter()
        };

        for n in iter {
            let ci = self.get_column(&n).id();
            self.cover_column(ci);
        }

        Ok(())
    }

    /// Cover a column by remove each action that could remove that
    /// column, and remove the column from the header list.
    pub fn cover_column(&mut self, column_index: usize) {
        {
            let col = &mut self.constraints[column_index];
            col.cover_header();
        }

        let iter = {
            let col = &self.constraints[column_index];
            col.iter()
        };

        for r in iter {
            // For every node in the row (except the one from this
            // constraing), remove the node from its column and
            // decrement the corresponding count
            for n in iter_row(&r) {
                let sn = n.upgrade().unwrap();
                let ci = sn.borrow().column.unwrap();
                println!("looking at other column: {}", ci);
                sn.borrow_mut().remove_from_column();
                self.constraints[ci].dec_count();
            }
        }
    }

    /// Uncover a column, repairing all links in reverse order.
    pub fn uncover_column(&mut self, column_index: usize) {
        let mut col = &mut self.constraints[column_index];

        for r in col.iter().rev() {
            // For every node in the row (except the one from this
            // constraing), remove the node from its column and
            // decrement the corresponding count
            for n in iter_row(&r).rev() {
                let sn = n.upgrade().unwrap();
                sn.borrow_mut().reinsert_into_column();
                col.inc_count();
            }
        }

        col.uncover_header();
    }

    pub fn all_constraints(&self) -> slice::Iter<Column> {
        self.constraints.iter()
    }


    /// Testing function: assert the number of headers in the linked list is equal to the number of constraints.
    pub fn assert_header_counts(&self) {
        assert_eq!(self.constraints.len(), iter_row(&Rc::downgrade(&self.root)).count());
        assert_eq!(self.constraints.len(), iter_row(&Rc::downgrade(&self.root)).rev().count());
    }

    pub fn print_remaining_constraint_counts(&self) {
        for c in iter_row(&Rc::downgrade(&self.root)) {
            let n = c.upgrade().unwrap();
            let col = &self.constraints[n.borrow().column.unwrap()];
            println!("column {}: {}", col.id(), col.count());
        }
    }
}
