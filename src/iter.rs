use node::{WeakNode, OwnedNode};

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
        let ref weak_next = self.curr_right.borrow().right().clone();
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
        let ref weak_next = self.curr_left.borrow().left().clone();
        self.curr_left = weak_next.upgrade().unwrap();

        if self.curr_right.borrow().column != self.curr_left.borrow().column {
            Some(weak_next.clone())
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ColumnIterator {
    curr_up: OwnedNode,
    curr_down: OwnedNode
}


/// Iterate through the ndoes in a column, skipping the initial
/// (header) node.
impl ColumnIterator {
    pub fn new(c: &OwnedNode) -> ColumnIterator {
        ColumnIterator { curr_up: c.clone(),
                         curr_down: c.clone() }
    }
}

impl Iterator for ColumnIterator {
    type Item = WeakNode;

    fn next(&mut self) -> Option<WeakNode> {
        let ref weak_next = self.curr_down.borrow().down();
        self.curr_down = weak_next.upgrade().unwrap();

        if self.curr_down.borrow().get_row() != self.curr_up.borrow().get_row() {
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

        if self.curr_down.borrow().get_row() != self.curr_up.borrow().get_row() {
            Some(weak_next.clone())
        }
        else {
            None
        }
    }
}

/// Iterate down through the elements of a column.
pub fn iter_col(header: &OwnedNode) -> ColumnIterator {
    ColumnIterator::new(header)
}

/// Return an iterator to iterate through the elements in the row
/// occupied by `node`.
pub fn iter_row(node: &WeakNode ) -> RowIterator {
    RowIterator::new(&node)
}
