use std::cell::RefCell;
use std::rc::{Rc, Weak};

type ColumnIndex = usize;
type RowIndex = usize;

#[derive(Debug)]
pub struct NodeContents {
    up: Weak<RefCell<NodeContents>>,
    down: Weak<RefCell<NodeContents>>,
    left: Weak<RefCell<NodeContents>>,
    right: Weak<RefCell<NodeContents>>,

    column: Option<ColumnIndex>,
    row: Option<RowIndex>
}

impl NodeContents {
    /// Create new node that circularly points to itself. 
    pub fn new() -> Rc<RefCell<NodeContents>> {
	let rc = Rc::new(RefCell::new(NodeContents {
	    up: Weak::new(), down: Weak::new(), 
            left: Weak::new(), right: Weak::new(),
            column: None, row: None
	}));

        {
            let mut nc = (*rc).borrow_mut();
            let w = Rc::downgrade(&rc);
	    nc.up = w.clone();
	    nc.down = w.clone();
	    nc.left = w.clone();
	    nc.right = w.clone();
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
}

#[derive(Debug)]
pub struct Column {
    head: Rc<RefCell<NodeContents>>,
    count: usize,
    id: usize,
}

impl Column {
    pub fn new(index: usize) -> Self {
	Column { head: NodeContents::new(), count: 0, id: index }
    }

    pub fn append_new(&mut self) -> Rc<RefCell<NodeContents>> {
	let n = NodeContents::new();
	self.append( &mut Rc::downgrade(&n) );
	n
    }

    pub fn append(&mut self, node: &mut Weak<RefCell<NodeContents>> ) {
        let r = (*node).upgrade().unwrap();

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

    pub fn get_count(&self) -> usize {
        return self.count
    }
}


#[derive(Debug)]
pub struct Row {
    head: Rc<RefCell<NodeContents>>,
    nodes: Vec<Rc<RefCell<NodeContents>>>,
    id: usize
}

impl Row {
    pub fn new(index: usize) -> Self {
        Row { head: NodeContents::new(), nodes: vec![], id: index }
    }

    /// Add an existing node as to the linked list formed by this row
    fn append(&mut self, node: &mut Rc<RefCell<NodeContents>> ) {
        
        self.nodes.push(node.clone());
	{
	    let mut n = (*node).borrow_mut();
            n.row = Some(self.id);
	    n.right = Rc::downgrade(&self.head);
	    n.left = self.head.borrow_mut().left.clone();
	}
	{
	    let mut head = self.head.borrow_mut();
	    head.left = Rc::downgrade(node);
	}
	{
	    let pleft = (*node).borrow_mut();
	    let prev_left  = pleft.left.upgrade().unwrap();
	    prev_left.borrow_mut().right = Rc::downgrade(node);
	}
    }
    
    /// Create a new node for this row and return it
    pub fn append_new(&mut self) -> Rc<RefCell<NodeContents>> {
        let mut v = NodeContents::new();
        self.append(&mut v);
        v
    }

    /// Remove 
    pub fn cover(&mut self) {

    }
}
