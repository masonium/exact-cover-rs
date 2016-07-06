use std::cell::{RefCell};
use std::rc::{Rc, Weak};
use std::fmt;

pub type ColumnIndex = usize;
pub type RowIndex = usize;

#[derive(Debug)]
pub struct NodeContents {
    up: Weak<RefCell<NodeContents>>,
    down: Weak<RefCell<NodeContents>>,
    left: Weak<RefCell<NodeContents>>,
    pub right: Weak<RefCell<NodeContents>>,
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

    /// Remove a node from its column
    pub fn remove_from_column(&mut self) {
	let l = self.up.clone();
	let r = self.down.clone();
	let lrc = self.up.upgrade().unwrap();
	(*lrc).borrow_mut().down = r;
	let rrc = self.up.upgrade().unwrap();
	(*rrc).borrow_mut().up = l;
    }

    /// Remove a node from its row
    pub fn remove_from_row(&mut self) {
	let l = self.left.clone();
	let r = self.right.clone();
	let lrc = self.left.upgrade().unwrap();
	(*lrc).borrow_mut().right = r;
	let rrc = self.left.upgrade().unwrap();
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

#[derive(Debug)]
pub struct ColumnIterator {
    head: WeakNode,
    count: usize,
    curr: usize
}

impl ColumnIterator {
    pub fn new(c: &Column) -> ColumnIterator {
        ColumnIterator { head: Rc::downgrade(&c.head), count: c.count, curr: 0 }
    }
}

impl Iterator for ColumnIterator {
    type Item = WeakNode;

    fn next(&mut self) -> Option<WeakNode> {
        if self.count > self.curr {
            None
        } else {
            self.curr += 1;
            let o = self.head.upgrade().unwrap();
            let ref next = o.borrow().down;
            self.head = next.clone();
            Some(self.head.clone())
        }
       
    }
}

#[derive(Debug)]
pub struct Column {
    head: Rc<RefCell<NodeContents>>,
    count: usize,
    id: usize,
}

impl Column {
    pub fn new(index: usize) -> Self {
        let nc = NodeContents::new();
        {
            let mut c = nc.borrow_mut();
            c.column = Some(index);
        }
	Column { head: nc, count: 0, id: index }
    }


    pub fn root(&self) -> WeakNode {
        Rc::downgrade(&self.head)
    }

    pub fn from_node(node: OwnedNode, index: usize) -> Self {
        Column { head: node, count: 0, id: index }
    }

    pub fn append_new(&mut self) -> OwnedNode {
	let n = NodeContents::new();
	self.append( &mut Rc::downgrade(&n) );
	n
    }

    pub fn append(&mut self, node: &Weak<RefCell<NodeContents>> ) {
        let r = (*node).upgrade().unwrap();
        r.borrow_mut().column = Some(self.id);

	{
            let mut n = r.borrow_mut();
	    n.down = Rc::downgrade(&self.head);
	    n.up = self.head.borrow_mut().up.clone();
	}
	{
	    let mut head = self.head.borrow_mut();
	    head.up = node.clone();
	}
	{            
            let n = r.borrow_mut();
	    let prev_up  = n.up.upgrade().unwrap();
	    prev_up.borrow_mut().down = node.clone();
	}

	self.count += 1
    }

    /// cover the header node of the column
    pub fn cover_header(&self) {
        self.head.borrow_mut().remove_from_row();
    }

    /// uncover teh header node of the column
    pub fn uncover_header(&self) {
        self.head.borrow_mut().reinsert_into_row();
    }

    pub fn iter(&self) -> ColumnIterator {
        ColumnIterator::new(&self)
    }

    pub fn get_count(&self) -> usize {
        return self.count
    }

    pub fn dec_count(&mut self) {
        self.count -= 1
    }
    pub fn inc_count(&mut self) {
        self.count += 1
    }
}


#[derive(Debug)]
pub struct Row {
    nodes: Vec<Rc<RefCell<NodeContents>>>,
    id: usize
}

impl Row {
    pub fn new(nodes: Vec<OwnedNode>, index: usize) -> Self {
        let l = nodes.len();
        for i in l..(2*l) {
            let mut n = nodes[i % l].borrow_mut();
            n.left = Rc::downgrade(&nodes[(i+1) % l]);
            n.right = Rc::downgrade(&nodes[(i+1) % l]);
            n.row = Some(index);
        }
        Row {nodes: nodes, id: index }
    }
}


#[derive(Debug)]
pub struct RowIterator {
    head: OwnedNode,
    curr_right: OwnedNode,
    curr_left: OwnedNode
}

/// Starting with the current node, iterate through them to the right
/// until hitting the same node.
impl RowIterator {
    fn new(node: &WeakNode, reverse: bool) -> RowIterator {
        let st = node.upgrade().unwrap();
        RowIterator { head: st.clone(), curr_right: st, curr_left: st, reverse: reverse }
    }
}

impl Iterator for RowIterator {
    type Item = WeakNode;

    fn next(&mut self) -> Option<WeakNode> {
        let ref weak_next = self.curr.borrow().right.clone()
        self.curr = weak_next.upgrade().unwrap();

        if self.curr.borrow().column != self.head.borrow().column {
            Some(weak_next.clone())
        }
        else {
            None
        }
    }
}

pub fn iter_row(node: &WeakNode, reverse: bool ) -> RowIterator {
    RowIterator::new(&node, reverse)
}

pub fn column_index(node: &WeakNode) -> Option<ColumnIndex> {
    let s = node.upgrade().unwrap();
    let c = s.borrow().column;
    c
}

pub fn row_index(node: &WeakNode) -> Option<RowIndex> {
    let s = node.upgrade().unwrap();
    let r = s.borrow().row;
    r
}
