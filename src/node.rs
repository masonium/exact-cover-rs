use std::cell::{RefCell};
use std::rc::{Rc, Weak};
use std::fmt;
use std::mem;

pub type ColumnIndex = usize;
pub type RowIndex = usize;

#[derive(Debug)]
pub struct NodeContents {
    up: Weak<RefCell<NodeContents>>,
    down: Weak<RefCell<NodeContents>>,
    left: Weak<RefCell<NodeContents>>,
    right: Weak<RefCell<NodeContents>>,
    at_self: Weak<RefCell<NodeContents>>,

    pub column: Option<ColumnIndex>,
    pub row: Option<RowIndex>
}


pub type OwnedNode = Rc<RefCell<NodeContents>>;
pub type WeakNode = Weak<RefCell<NodeContents>>;

impl NodeContents {
    /// Create new node that circularly points to itself. 
    pub fn new() -> Rc<RefCell<NodeContents>> {
	let rc = Rc::new(RefCell::new(NodeContents {
	    up: Weak::new(), down: Weak::new(), 
            left: Weak::new(), right: Weak::new(),
            at_self: Weak::new(),
            column: None, row: None
	}));

        {
            let mut nc = (*rc).borrow_mut();
            let w = Rc::downgrade(&rc);
	    nc.up = w.clone();
	    nc.down = w.clone();
	    nc.left = w.clone();
	    nc.right = w.clone();
            nc.at_self = w.clone();
        }
	rc
    }
    pub fn prepend_node(&mut self, node: &WeakNode) {
        {
            let r = (*node).upgrade().unwrap();
            let mut n = r.borrow_mut();
            n.down = self.at_self.clone();
            n.up = self.up.clone();
        }

        {
            let prev_up_weak = mem::replace(&mut self.up, node.clone());
            let prev_up = prev_up_weak.upgrade().unwrap();
            prev_up.borrow_mut().down = node.clone();
        }
    }

    /// return the down link
    pub fn down(&self) -> WeakNode {
        self.down.clone()
    }

    /// return the up link
    pub fn up(&self) -> WeakNode {
        self.up.clone()
    }

    /// return the left link
    pub fn left(&self) -> WeakNode {
        self.left.clone()
    }

    /// return the right link
    pub fn right(&self) -> WeakNode {
        self.right.clone()
    }

    /// Remove a node from its column
    pub fn remove_from_column(&mut self) {
	let up_weak = self.up.clone();
	let down_weak = self.down.clone();
	let up = up_weak.upgrade().unwrap();
	up.borrow_mut().down = down_weak.clone();
	let down = down_weak.upgrade().unwrap();
	down.borrow_mut().up = up_weak;
    }

    /// Remove a node from its row
    pub fn remove_from_row(&mut self) {
	let l = self.left.clone();
	let r = self.right.clone();
	let lrc = self.left.upgrade().unwrap();
	(*lrc).borrow_mut().right = r;
	let rrc = self.right.upgrade().unwrap();
	(*rrc).borrow_mut().left = l;
    }

    /// Re-add a node to its column
    pub fn reinsert_into_column(&mut self) {
	let urc = self.up.upgrade().unwrap();
	(*urc).borrow_mut().down = self.at_self.clone();
	let drc = self.down.upgrade().unwrap();
	(*drc).borrow_mut().up = self.at_self.clone();
    }

    /// Re-add a node to its row
    pub fn reinsert_into_row(&mut self) {
	let lrc = self.left.upgrade().unwrap();
	(*lrc).borrow_mut().right = self.at_self.clone();
	let rrc = self.right.upgrade().unwrap();
	(*rrc).borrow_mut().left = self.at_self.clone();
    }
}

impl fmt::Display for NodeContents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.row, self.column)
    }
}

/// Prepend `node` to the left of `root.
pub fn prepend_left(root: &mut OwnedNode, node: &WeakNode) {
    let l = (*node).upgrade().unwrap();

    {
	let mut n = l.borrow_mut();
	n.right = Rc::downgrade(&root);
	n.left = root.borrow_mut().left.clone();
    }
    {
	let mut head = root.borrow_mut();
	head.left = (*node).clone();
    }
    {
	let pleft = l.borrow_mut();
	let prev_left  = pleft.left.upgrade().unwrap();
	prev_left.borrow_mut().right = (*node).clone();
    }

}

pub fn prepend_up(root: &mut OwnedNode, node: &WeakNode) {
    let u = (*node).upgrade().unwrap();

    {
	let mut n = u.borrow_mut();
	n.down = Rc::downgrade(&root);
	n.up = root.borrow_mut().up.clone();
    }
    {
	let mut head = root.borrow_mut();
	head.up = (*node).clone();
    }
    {
	let pup = u.borrow_mut();
	let prev_up  = pup.up.upgrade().unwrap();
	prev_up.borrow_mut().down = (*node).clone();
    }
}

#[derive(Debug)]
pub struct Row<Action: Copy> {
    nodes: Vec<Rc<RefCell<NodeContents>>>,
    id: usize,
    action: Action
}

impl<Action: Copy> Row<Action> {
    pub fn new(nodes: Vec<OwnedNode>, action: Action, index: usize) -> Self {
        let l = nodes.len();
        for i in l..(2*l) {
            let mut n = nodes[i % l].borrow_mut();
            n.left = Rc::downgrade(&nodes[(i+1) % l]);
            n.right = Rc::downgrade(&nodes[(i+1) % l]);
            n.row = Some(index);
        }
        Row {nodes: nodes, id: index, action: action }
    }

    pub fn first_node(&self) -> WeakNode {
        Rc::downgrade(&self.nodes[0])
    }

    pub fn iter(&self) -> FullRowIterator {
        FullRowIterator::new(&self)
    }
    pub fn action(&self) -> Action {
        self.action
    }
}


#[derive(Debug)]
pub struct RowIterator {
    curr_right: OwnedNode,
    curr_left: OwnedNode
}

/// Iterate through them to the right (in normal order)
/// until hitting the original node. Does not hit the first node.
impl RowIterator {
    fn new(node: &WeakNode) -> RowIterator {
        let st = node.upgrade().unwrap();
        RowIterator { curr_right: st.clone(), curr_left: st }
    }
}

impl Iterator for RowIterator {
    type Item = WeakNode;

    fn next(&mut self) -> Option<WeakNode> {
        let ref weak_next = self.curr_right.borrow().right.clone();
        self.curr_right = weak_next.upgrade().unwrap();

        if self.curr_right.borrow().column != self.curr_left.borrow().column {
            Some(weak_next.clone())
        }
        else {
            None
        }
    }
}

impl DoubleEndedIterator for RowIterator {
    fn next_back(&mut self) -> Option<WeakNode> {
        let ref weak_next = self.curr_left.borrow().left.clone();
        self.curr_left = weak_next.upgrade().unwrap();

        if self.curr_right.borrow().column != self.curr_left.borrow().column {
            Some(weak_next.clone())
        }
        else {
            None
        }
    }
}

/// Iterator for row nodes (not header rwos)
pub struct FullRowIterator {
    head_column: ColumnIndex,
    curr: OwnedNode,
    started: bool
}

impl FullRowIterator {
    pub fn new<A: Copy>(row: &Row<A>) -> FullRowIterator {
        FullRowIterator{ head_column: row.nodes[0].borrow().column.unwrap(), curr: row.nodes[0].clone(),
                         started: false }
    }
}

impl Iterator for FullRowIterator {
    type Item = WeakNode;

    fn next(&mut self) -> Option<WeakNode> {
        let ret = self.curr.clone();
        if self.started {
            if ret.borrow().column.unwrap() == self.head_column {
                return None
            } 
        }
        self.curr = { let r = self.curr.borrow().right(); r.upgrade().unwrap() };
        self.started = true;
        Some(Rc::downgrade(&ret))
    }
}

pub fn iter_row(node: &WeakNode ) -> RowIterator {
    RowIterator::new(&node)
}

pub fn column_index(node: &WeakNode) -> Option<ColumnIndex> {
    let s = node.upgrade().unwrap();
    let c = s.borrow().column;
    c
}

