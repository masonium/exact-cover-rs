use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct NodeContents {
    up: Weak<RefCell<NodeContents>>,
    down: Weak<RefCell<NodeContents>>,
    left: Weak<RefCell<NodeContents>>,
    right: Weak<RefCell<NodeContents>>
}

#[derive(Debug)]
pub struct Column {
    head: Rc<RefCell<NodeContents>>,
    count: usize
}

pub struct Row {
    head: Rc<RefCell<NodeContents>>,
    nodes: Vec<Rc<RefCell<NodeContents>>>
}

impl NodeContents {
    pub fn new() -> Rc<RefCell<NodeContents>> {
        let rc = Rc::new(RefCell::new(NodeContents {
            up: Weak::new(), down: Weak::new(), left: Weak::new(), right: Weak::new()
        }));
        (*rc).borrow_mut().up = Rc::downgrade(&rc);
        (*rc).borrow_mut().down = Rc::downgrade(&rc);
        rc
    }

    pub fn remove_from_column(&mut self) {
        let l = self.up.clone();
        let r = self.down.clone();
        let lrc = self.up.upgrade().unwrap();
        (*lrc).borrow_mut().down = r;
        let rrc = self.up.upgrade().unwrap();
        (*rrc).borrow_mut().up = l;
    }
}

impl Column {
    pub fn new() -> Column {
        Column { head: NodeContents::new(),  count: 0 }
    }

    // fn node_ref(&self) -> Weak<RefCell<NodeContents>> {
    //     Rc::downgrade(&self.node)
    // }

    pub fn append_new(&mut self) -> Rc<RefCell<NodeContents>> {
        let mut n = NodeContents::new();
        self.append( &mut n );
        n
    }

    pub fn append(&mut self, node: &mut Rc<RefCell<NodeContents>> ) {
        {
            let mut n = (*node).borrow_mut();
            n.down = Rc::downgrade(&self.head);
            n.up = self.head.borrow_mut().up.clone();
        }
        {
            let mut head = self.head.borrow_mut();
            head.up = Rc::downgrade(node);
        }
        {
            let pup = (*node).borrow_mut();
            let prev_up  = pup.up.upgrade().unwrap();
            prev_up.borrow_mut().down = Rc::downgrade(node);
        }
        self.count += 1
    }
}
