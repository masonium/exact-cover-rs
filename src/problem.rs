use node::{WeakNode, OwnedNode, NodeContents, Row};
use iter::{iter_row};
use node::{prepend_left, prepend_up};
use std::hash::Hash;
use std::collections::{HashMap};
use std::rc::{Rc};
use cover::{cover_column};

pub trait Constraint : Clone + Hash + Eq {}
impl<T: Clone + Hash + Eq> Constraint for T {}

pub trait Action : Copy + Hash + Eq {}
impl<T: Copy + Hash + Eq> Action for T {}

pub struct Problem<A: Action, C: Constraint> {
    root: OwnedNode,
    pub constraints: Vec<OwnedNode>,
    pub actions: Vec<Row<A>>,
    constraint_map: HashMap<C, usize>,
    constraint_names: Vec<C>,
    action_map: HashMap<A, usize>
}

impl<A: Action, C: Constraint> Problem<A, C> {
    pub fn new() -> Problem<A, C> {
        Problem { constraints: Vec::new(), actions: Vec::new(),
                  root: NodeContents::new_root(),
                  constraint_map: HashMap::new(),
                  constraint_names: Vec::new(),
                  action_map: HashMap::new()
        }
    }

    /// Add a constraint if it doesn't already exist.
    pub fn add_constraint(&mut self, constraint: &C) {
        let curr_size = self.constraint_map.len();
        if !self.constraint_map.contains_key(constraint) {
            let c = NodeContents::new_header(Some(curr_size));
            prepend_left(&mut self.root, &Rc::downgrade(&c));
            self.constraints.push(c);
            self.constraint_map.insert(constraint.clone(), curr_size);
            self.constraint_names.insert(curr_size, constraint.clone());
        }
    }

    /// Add a new action, creating additional constraints on
    /// demand. Actions must not already have already existed.
    pub fn add_action(&mut self, a: A, clist: &[C]) {
        // Ignore actions that don't satisfy constraints
        if clist.is_empty() {
            return
        }

        // Ignore actions that are already present.
        if self.action_map.contains_key(&a) {
            return
        }

        // extend the constraint list to accomodate all constraints, if necessary
        for x in clist {
            self.add_constraint(x);
        }

        // create a row from those nodes
        let new_id  = self.actions.len();

        // Create and collect new nodes for each constraint.
        let nodes = clist.iter().map(|x| {
            let ci = *self.constraint_map.get(x).unwrap();
            let c = &self.constraints[ci];
            let n = NodeContents::new_inner(&c, new_id);
            prepend_up(c, &Rc::downgrade(&n));
            c.borrow_mut().inc_count();
            n
        }).collect();

        self.actions.push(Row::new(nodes, a, new_id));
        self.action_map.insert(a, new_id);
    }

    /// Choose the column with the smallest count
    pub fn choose_column(&self) -> Option<&OwnedNode> {
        iter_row(&Rc::downgrade(&self.root))
            .map( |ref node| self.get_column(&node) )
            .min_by_key( |ref c| c.borrow().get_count() )
    }

    /// Return the number of constraints currently in the problem.
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    /// Return the column associated with a node.
    fn get_column(&self, row_node: &WeakNode) -> &OwnedNode {
        let s = row_node.upgrade().unwrap();
        let ci = s.borrow().column.unwrap();
        &self.constraints[ci]
    }

    /// Return the row associated with a node.
    fn get_row(&self, row_node: &WeakNode) -> &Row<A> {
        let s = row_node.upgrade().unwrap();
        let ri = s.borrow().get_row().unwrap();
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

            if let Some(c) = act.iter().map(|node| { self.get_column(&node) }).find(|c| { c.borrow().is_already_chosen() })
            {
                return Err(format!("Could not require row; C {} already satisfied", c.borrow().column.unwrap()));
            }
            act.iter()
        };

        for n in iter {
            //let ci = self.get_column(&n).borrow().column.unwrap();
            cover_column(self.get_column(&n));
        }

        Ok(())
    }
}
