use node::{OwnedNode};
use iter::{iter_row, iter_col};

/// Structure to temporarily cover a column. The column is
/// automatically covered when the instance is dropped.
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

pub fn try_cover_column(col: &OwnedNode) -> TempCoverColumn {
    TempCoverColumn::new(col)
}

pub fn cover_column(col: &OwnedNode) {
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
