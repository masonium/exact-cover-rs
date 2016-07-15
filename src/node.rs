use std::cell::{RefCell};
use std::rc::{Rc, Weak};

pub type ColumnIndex = usize;
pub type RowIndex = usize;

/// `NodeExtra` contains the information specific to a node's
/// type. For instance, only constraint (column) header nodes require
/// a count of how many nodes are contained in the column. The type of
/// node (header, inner, or root) can be directly inferred by the enum
/// value.
#[derive(Debug)]
pub enum NodeExtra {
    Row(RowIndex), // The node is an inner node, representing part of an action.
    Count(usize),  // The node is a header for a constraints.
    Root           // Root node.
}


/// A `Node` represents multiple types in the DLX algorithm. Most nodes
/// are 'inner' nodes, essentially representing 1's in the sparse
/// matrix representation of the problem. Headers `Node`s contain a
/// current count of how many actions can be used to satisfy the
/// constaint.
///
/// Throughout the code, nodes are almost never dealt with
/// directly. Instead, nodes are handled either through `WeakNode`s or
/// `OwnedNode`s.
///
/// `Node`s exclusively store neighbor pointers as `WeakNode`s to
/// prevent memory leaks via cyclical references.
#[derive(Debug)]
pub struct Node {
    up: Weak<RefCell<Node>>,
    down: Weak<RefCell<Node>>,
    left: Weak<RefCell<Node>>,
    right: Weak<RefCell<Node>>,
    at_self: Weak<RefCell<Node>>,

    pub column: Option<ColumnIndex>,
    header: Weak<RefCell<Node>>,
    extra: NodeExtra
}


pub type OwnedNode = Rc<RefCell<Node>>;
pub type WeakNode = Weak<RefCell<Node>>;

impl Node {
    pub fn new_header(col: Option<usize>) -> OwnedNode {
        Self::new(col, None, NodeExtra::Count(0))
    }

    pub fn new_inner(header: &OwnedNode, row: usize) -> OwnedNode {
        Self::new(header.borrow().column, Some(&Rc::downgrade(&header)), NodeExtra::Row(row))
    }

    pub fn new_root() -> OwnedNode {
        Self::new(None, None, NodeExtra::Root)
    }

    /// Create new node that circularly points to itself. 
    fn new(col: Option<usize>, header: Option<&WeakNode>, e: NodeExtra) -> OwnedNode {
	let rc = Rc::new(RefCell::new(Node {
	    up: Weak::new(), down: Weak::new(), 
            left: Weak::new(), right: Weak::new(),
            at_self: Weak::new(),
            header: Weak::new(),
            column: col, extra: e
	}));

        {
            let mut nc = (*rc).borrow_mut();
            let w = Rc::downgrade(&rc);
	    nc.up = w.clone();
	    nc.down = w.clone();
	    nc.left = w.clone();
	    nc.right = w.clone();
            nc.at_self = w.clone();
            nc.header = match header {
                Some(n) => n.clone(),
                None => w.clone()
            }
        }
	rc
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

    /// Return the count for a header node, and None for other nodes.
    pub fn get_count(&self) -> Option<usize> {
        match self.extra {
            NodeExtra::Count(i) => Some(i),
            _ => None
        }
    }

    pub fn dec_count(&mut self) {
        let c = match self.extra {
            NodeExtra::Count(i) => i,
            _ => return
        };
        self.extra = NodeExtra::Count(c-1);
    }

    pub fn inc_count(&mut self) {
        let c = match self.extra {
            NodeExtra::Count(i) => i,
            _ => return
        };
        self.extra = NodeExtra::Count(c+1);
    }

    /// Return the associated row index for an inner node, and None
    /// for other nodes.
    pub fn get_row(&self) -> Option<usize> {
        match self.extra {
            NodeExtra::Row(i) => Some(i),
            _ => None
        }
    }

    /// Return the constraint header of this node.
    pub fn get_header(&self) -> WeakNode {
        self.header.clone()
    }

    /// Return true iff this is a header node and the column has
    /// already been chosen.
    pub fn is_already_chosen(&self) -> bool {
        if let NodeExtra::Count(_) = self.extra {
            let r = self.right.upgrade().unwrap();
            let l = r.borrow().left().upgrade().unwrap();
            let lc = l.borrow().column;
            lc != self.column
        } else {
            false
        }
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
pub fn get_header(n: &WeakNode) -> WeakNode {
    let s = n.upgrade().unwrap();
    let h = s.borrow().get_header();
    h.clone()
}

// impl fmt::Display for Node {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({:?}, {:?})", self.row, self.column)
//     }
// }

/// Prepend `node` to the left of `root.
pub fn prepend_left(root: &OwnedNode, node: &WeakNode) {
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

/// Prepend node above root.
pub fn prepend_up(root: &OwnedNode, node: &WeakNode) {
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
    nodes: Vec<Rc<RefCell<Node>>>,
    id: usize,
    action: Action
}

impl<Action: Copy> Row<Action> {
    pub fn new(nodes: Vec<OwnedNode>, action: Action, index: usize) -> Self {
        let l = nodes.len();
        for i in l..(2*l) {
            let mut n = nodes[i % l].borrow_mut();
            n.left = Rc::downgrade(&nodes[(i-1) % l]);
            n.right = Rc::downgrade(&nodes[(i+1) % l]);
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
