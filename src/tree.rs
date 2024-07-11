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

    fn duplicate(&self) -> Node {
        Node {
            hash: self.hash.clone(),
            left: self.left.as_ref().map(|left| Box::new(left.duplicate())),
            right: self.right.as_ref().map(|right| Box::new(right.duplicate())),
            parent: self.parent,
        }
    }

    fn hash(&self) -> Vec<u8> {
        self.hash.clone()
    }
}
