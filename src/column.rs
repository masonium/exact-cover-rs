use node::{NodeContents, OwnedNode, WeakNode};
use node::{prepend_up};
use std::rc::Rc;

#[derive(Debug)]
pub struct ColumnIterator {
    curr_up: OwnedNode,
    curr_down: OwnedNode
}

/// Iterate through the ndoes in a column, skipping the initial
/// (header) node.
impl ColumnIterator {
    pub fn new(c: &Column) -> ColumnIterator {
        ColumnIterator { curr_up: c.head.clone(),
                         curr_down: c.head.clone() }
    }
}

impl Iterator for ColumnIterator {
    type Item = WeakNode;

    fn next(&mut self) -> Option<WeakNode> {
        let ref weak_next = self.curr_down.borrow().down();
        self.curr_down = weak_next.upgrade().unwrap();

        if self.curr_down.borrow().row != self.curr_up.borrow().row {
            Some(weak_next.clone())
        }
        else {
            None
        }
    }
}

impl DoubleEndedIterator for ColumnIterator {
    fn next_back(&mut self) -> Option<WeakNode> {
        let ref weak_next = self.curr_up.borrow().up();
        self.curr_up = weak_next.upgrade().unwrap();

        if self.curr_down.borrow().row != self.curr_up.borrow().row {
            Some(weak_next.clone())
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Column {
    head: OwnedNode,
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

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn root(&self) -> WeakNode {
        Rc::downgrade(&self.head)
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Create a new node append to the end of the column, and return.
    pub fn append_new(&mut self) -> OwnedNode {
        let n = NodeContents::new();
        self.append( &n);
        n.clone()
    }


    fn append(&mut self, node: &OwnedNode ) {
        {
            node.borrow_mut().column = Some(self.id);
        }

        prepend_up(&mut self.head, &Rc::downgrade(&node));
        self.count += 1;
    }

    /// Return true iff this column has already been satisfied as a
    /// constraint.
    pub fn is_already_chosen(&self) -> bool {
        let wr = self.head.borrow().right();
        let r = wr.upgrade().unwrap();
        let l = r.borrow().left().upgrade().unwrap();
        let lc = l.borrow().column;
        match lc {
            Some(x) if x == self.id => false,
            _ => true
        }
    }

    /// Cover the header node of the column, removing it from the
    /// constraint list.
    pub fn cover_header(&self) {
        self.head.borrow_mut().remove_from_row();
    }

    /// Uncover the header node of the column, adding it back to the
    /// constraint list.
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


#[test]
fn create_column() {
    let mut h = Column::new(0);
    h.append_new();
}
