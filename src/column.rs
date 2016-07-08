use node::{NodeContents, OwnedNode, WeakNode};
use node::{prepend_up};
use std::rc::Rc;
use iter::{iter_row};

#[derive(Debug)]
pub struct ColumnIterator {
    curr_up: OwnedNode,
    curr_down: OwnedNode
}

pub fn iter_col(header: &OwnedNode) -> ColumnIterator {
    ColumnIterator::new(header)
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

pub fn try_cover_column(col: &OwnedNode) -> TempCoverColumn {
    TempCoverColumn::new(col)
}

pub fn cover_column(col: &OwnedNode) {
//    println!("Covering c{}", column_index);
    col.borrow_mut().remove_from_row();

    for r in iter_col(col) {
        // For every node in the row (except the one from this
        // constraint), remove the node from its column and
        // decrement the corresponding count
        for n in iter_row(&r) {
            let sn = n.upgrade().unwrap();
            //let ci = sn.borrow().column.unwrap();

            sn.borrow_mut().remove_from_column();
            let header = sn.borrow().get_header().upgrade().unwrap();
            header.borrow_mut().dec_count();
        }
    }
}

pub fn uncover_column(col: &OwnedNode) {
//    println!("Covering c{}", column_index);

    for r in iter_col(col).rev() {
        // For every node in the row (except the one from this
        // constraint), reinsert node into its column and
        // increment that column's count.
        for n in iter_row(&r).rev() {
            let sn = n.upgrade().unwrap();
            //let ci = sn.borrow().column.unwrap();

            sn.borrow_mut().reinsert_into_column();
            let header = sn.borrow().get_header().upgrade().unwrap();
            header.borrow_mut().inc_count();
        }
    }

    col.borrow_mut().reinsert_into_row();

}

pub struct TempCoverColumn<'a> {
    node: &'a OwnedNode,
}

impl<'a> TempCoverColumn<'a> {
    pub fn new(node: &'a OwnedNode) -> TempCoverColumn<'a> {
        cover_column(node);
        TempCoverColumn { node: node }
    }
}

impl<'a> Drop for TempCoverColumn<'a> {
    fn drop(&mut self) {
        uncover_column(&self.node);
    }
}
