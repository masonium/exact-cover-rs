extern crate dancing_links;

use dancing_links::node::{Column};
use dancing_links::problem::{Problem};

#[test]
fn create_column() {
    let mut h = Column::new(0);
    let n = h.append_new();
}

#[test]
fn create_problem() {
    let mut p = Problem::new();
    p.add_action(&[0, 1, 2]);
    assert_eq!(p.count_cells(), 3);
    p.add_action(&[3, 4]);
    assert_eq!(p.count_cells(), 5);
    p.add_action(&[2, 4]);
    assert_eq!(p.count_cells(), 7);
}
