pub use column::{Column};

use node::{Row, WeakNode, OwnedNode, NodeContents, iter_row, column_index};
use node::{prepend_left};
use std::hash::Hash;
use std::collections::{HashMap};
use std::slice;
use std::rc::{Rc};
use std::fmt::Debug;

pub trait Constraint : Debug + Clone + Hash + Eq {}
impl<T: Debug + Clone + Hash + Eq> Constraint for T {}

pub trait Action : Debug + Copy + Hash + Eq {}
impl<T: Debug + Copy + Hash + Eq> Action for T {}

pub struct Problem<A: Action, C: Constraint> {
    root: OwnedNode,
    pub constraints: Vec<Column<C>>,
    pub actions: Vec<Row<A>>,
    constraint_map: HashMap<C, usize>,
    action_map: HashMap<A, usize>,
}


impl<A: Action, C: Constraint> Problem<A, C> {
    pub fn new() -> Problem<A, C> {
        Problem { constraints: Vec::new(), actions: Vec::new(),
                  root: NodeContents::new(),
                  constraint_map: HashMap::new(),
                  action_map: HashMap::new()
        }
    }

    /// Add a new action, creating additional constraints on demand
    pub fn add_action(&mut self, a: A, clist: &[C]) {
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
    pub fn choose_column(&self) -> Option<&Column<C>> {
        iter_row(&Rc::downgrade(&self.root))
            .map( |ref node| self.get_column(&node) )
            .min_by_key( |ref c| c.count() )
    }

    /// Return the column associated with a node.
    fn get_column(&self, row_node: &WeakNode) -> &Column<C> {
        let s = row_node.upgrade().unwrap();
        let ci = s.borrow().column.unwrap();
        &self.constraints[ci]
    }

    fn get_column_mut(&mut self, row_node: &WeakNode) -> &mut Column<C> {
        let s = row_node.upgrade().unwrap();
        let ci = s.borrow().column.unwrap();
        &mut self.constraints[ci]
    }


    /// Return the row associated with a node.
    fn get_row(&self, row_node: &WeakNode) -> &Row<A> {
        let s = row_node.upgrade().unwrap();
        let ri = s.borrow().row.unwrap();
        &self.actions[ri]
    }

    /// Return the action associated with a node.
    pub fn get_action(&self, row_node: &WeakNode) -> A {
        self.get_row(row_node).action()
    }


    /// Require that a given action be part of the solution
    pub fn require_row(&mut self, action: A) -> Result<(), String> {
        let iter = {
            let act = &(self.actions.get(*self.action_map.get(&action).unwrap()).unwrap());

            if let Some(c) = act.iter().map(|node| { self.get_column(&node) }).find(|c| { c.is_already_chosen() })
            {
                return Err(format!("Could not require row; C {} already satisfied", c.id()));
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
            // constraint), remove the node from its column and
            // decrement the corresponding count
            for n in iter_row(&r) {
                let sn = n.upgrade().unwrap();
                let ci = sn.borrow().column.unwrap();

                sn.borrow_mut().remove_from_column();
                self.constraints[ci].dec_count();
            }
        }

        self.assert_header_counts();
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

        self.assert_header_counts();
    }

    pub fn all_constraints(&self) -> slice::Iter<Column<C>> {
        self.constraints.iter()
    }


    /// Testing function: assert the number of headers in the linked list is equal to the number of constraints.
    pub fn assert_header_counts(&self) {
        for x in iter_row(&Rc::downgrade(&self.root)) {
            let c = self.get_column(&x);
            assert_eq!(c.iter().count(), c.count());
        }
    }

    pub fn remaining_header_counts(&self) {
        print!("Headers: ");
        for x in iter_row(&Rc::downgrade(&self.root)) {
            let c = self.get_column(&x);
            print!("({:?}, {}); ", c.constraint(), c.count());
        }
        println!("");
    }
}


#[test]
fn row_rev_iterator() {
    let mut p  = Problem::new();
    p.add_action(0, &[0, 1, 2, 3]);
    let r = &p.actions[0];
    let mut iter = iter_row(&r.first_node()).rev();

    assert_eq!(column_index(&iter.next().unwrap()).unwrap(), 3);
    assert_eq!(column_index(&iter.next().unwrap()).unwrap(), 2);
    assert_eq!(column_index(&iter.next().unwrap()).unwrap(), 1);
}

#[test]
fn row_normal_iterator() {
    let mut p = Problem::new();
    p.add_action(0, &[0, 1, 2, 3]);
    let r = &p.actions[0];
    let mut iter = iter_row(&r.first_node());

    assert_eq!(column_index(&iter.next().unwrap()).unwrap(), 1);
    assert_eq!(column_index(&iter.next().unwrap()).unwrap(), 2);
    assert_eq!(column_index(&iter.next().unwrap()).unwrap(), 3);
}

#[test]
fn column_normal_iterator() {
    let mut p = Problem::new();
    p.add_action(0, &[0]);
    p.add_action(1, &[0]);
    p.add_action(2, &[0]);
    let c = &p.constraints[0];
    let mut iter = c.iter();

    assert_eq!(p.get_action(&iter.next().unwrap()), 0);
    assert_eq!(p.get_action(&iter.next().unwrap()), 1);
    assert_eq!(p.get_action(&iter.next().unwrap()), 2);
}

#[test]
fn column_rev_iterator() {
    let mut p = Problem::new();
    p.add_action(0, &[0]);
    p.add_action(1, &[0]);
    p.add_action(2, &[0]);
    let c = &p.constraints[0];
    let mut iter = c.iter().rev();

    assert_eq!(p.get_action(&iter.next().unwrap()), 2);
    assert_eq!(p.get_action(&iter.next().unwrap()), 1);
    assert_eq!(p.get_action(&iter.next().unwrap()), 0);
}
