use node::{Column, Row};


pub struct Problem {
    constraints: Vec<Column>,
    actions: Vec<Row>
}

impl Problem {
    pub fn new(num_constraints: usize) -> Problem {
        Problem { constraints: Vec::new(), actions: Vec::new() }
    }

    /// Add a new action, creating constrains
    pub fn add_action(&mut self, c: &[u32]) {

    }
}
