pub struct Node {
    hash: Vec<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    parent: Option<*mut Node>,
}

impl Node {
    pub fn new(
        hash: Vec<u8>,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        parent: Option<*mut Node>,
    ) -> Self {
        Node {
            hash,
            left,
            right,
            parent,
        }
    }

    fn duplicate(&self) {}
    fn hash(&self) {}
}
