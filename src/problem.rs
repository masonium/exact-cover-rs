pub use column::{Column};

use node::{Row, WeakNode, OwnedNode, NodeContents, iter_row, column_index};
use node::{prepend_left};
use std::hash::Hash;
use std::collections::{HashMap};
use std::slice;
use std::rc::{Rc};
use std::fmt::Debug;


pub struct Problem<Action: Debug + Copy + Eq + Hash, Constraint: Debug + Clone + Hash + Eq> {
    root: OwnedNode,
    pub constraints: Vec<Column<Constraint>>,
    pub actions: Vec<Row<Action>>,
    constraint_map: HashMap<Constraint, usize>,
    action_map: HashMap<Action, usize>,
}


impl<Action: Debug + Copy + Eq + Hash, Constraint: Debug + Clone + Hash + Eq> Problem<Action, Constraint> {
    pub fn new() -> Problem<Action, Constraint> {
        Problem { constraints: Vec::new(), actions: Vec::new(),
                  root: NodeContents::new(),
                  constraint_map: HashMap::new(),
                  action_map: HashMap::new()
        }
    }

    /// Add a new action, creating additional constraints on demand
    pub fn add_action(&mut self, a: Action, clist: &[Constraint]) {
        // Ignore actions that don't satisfy constraints
        if clist.is_empty() {
            return
        }

        // extend the constraint list to accomodate all constraints, if necessary
        for x in clist {
            let curr_size = self.constraint_map.len();
            if !self.constraint_map.contains_key(x) {
                let c = Column::new(x.clone(), curr_size);
                prepend_left(&mut self.root, &c.root());
                self.constraints.push(c);
                self.constraint_map.insert(x.clone(), curr_size);
            }
        }

        // Create and collect new nodes for each constraint.
        let nodes = clist.iter().map(|x| {
            self.constraints[*self.constraint_map.get(x).unwrap()].append_new()
        }).collect();

        // create a row from those nodes
        let new_id  = self.actions.len();
        self.actions.push(Row::new(nodes, a, new_id));
        self.action_map.insert(a, new_id);
    }

    // Count the number of inner cells in the entire matrix
    pub fn count_cells(&self) -> usize {
        return self.constraints.iter().map(|ref x| { x.get_count() }).fold(0, |x, y| { x + y })
    }

    /// Choose the column with the smallest count
    pub fn choose_column(&self) -> Option<&Column<Constraint>> {
        iter_row(&Rc::downgrade(&self.root))
            .map( |ref node| self.get_column(&node) )
            .min_by_key( |ref c| c.count() )
    }

    /// Return the column associated with a node.
    fn get_column(&self, row_node: &WeakNode) -> &Column<Constraint> {
        let s = row_node.upgrade().unwrap();
        let ci = s.borrow().column.unwrap();
        &self.constraints[ci]
    }

    fn get_column_mut(&mut self, row_node: &WeakNode) -> &mut Column<Constraint> {
        let s = row_node.upgrade().unwrap();
        let ci = s.borrow().column.unwrap();
        &mut self.constraints[ci]
    }


    /// Return the row associated with a node.
    fn get_row(&self, row_node: &WeakNode) -> &Row<Action> {
        let s = row_node.upgrade().unwrap();
        let ri = s.borrow().row.unwrap();
        &self.actions[ri]
    }

    /// Return the action associated with a node.
    pub fn get_action(&self, row_node: &WeakNode) -> Action {
        self.get_row(row_node).action()
    }


    /// Require that a given action be part of the solution
    pub fn require_row(&mut self, action: Action) -> Result<(), String> {
        let iter = {
            let act = &(self.actions.get(*self.action_map.get(&action).unwrap()).unwrap());

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
            println!("Looking at action {:?}", self.get_action(&r));
            // For every node in the row (except the one from this
            // constraint), remove the node from its column and
            // decrement the corresponding count
            for n in iter_row(&r) {
                let sn = n.upgrade().unwrap();
                let ci = sn.borrow().column.unwrap();
                //println!("Decrementing {:?}", self.get_column(&n).constraint());

                sn.borrow_mut().remove_from_column();
                self.constraints[ci].dec_count();
            }
        }
    }

    /// Uncover a column, repairing all links in reverse order.
    pub fn uncover_column(&mut self, column_index: usize) {
        let iter = self.constraints[column_index].iter();

        for r in iter.rev() {
            // For every node in the row (except the one from this
            // constraing), remove the node from its column and
            // decrement the corresponding count
            for n in iter_row(&r).rev() {
                let sn = n.upgrade().unwrap();

                sn.borrow_mut().reinsert_into_column();
                self.get_column_mut(&n).inc_count();
            }
        }

        self.constraints[column_index].uncover_header();
    }

    pub fn all_constraints(&self) -> slice::Iter<Column<Constraint>> {
        self.constraints.iter()
    }


    /// Testing function: assert the number of headers in the linked list is equal to the number of constraints.
    pub fn assert_header_counts(&self) {
        assert_eq!(self.constraints.len(), iter_row(&Rc::downgrade(&self.root)).count());
        assert_eq!(self.constraints.len(), iter_row(&Rc::downgrade(&self.root)).rev().count());
    }

    pub fn remaining_header_counts(&self) {
        for x in iter_row(&Rc::downgrade(&self.root)) {
            let c = self.get_column(&x);
            print!("({:?}, {}); ", c.constraint(), c.count());
        }
        println!("");
    }
}
