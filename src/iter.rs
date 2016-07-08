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

/// Return an iterator to iterate through the elements in the row
/// occupied by `node`.
pub fn iter_row(node: &WeakNode ) -> RowIterator {
    RowIterator::new(&node)
}
