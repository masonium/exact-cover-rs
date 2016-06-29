extern crate dancing_links;

use dancing_links::{ColumnHeader, NodeContents};

#[test]
fn it_works() {
    let mut h = ColumnHeader::new();
    let mut n = NodeContents::new();
    h.append_node(&mut n);
}
