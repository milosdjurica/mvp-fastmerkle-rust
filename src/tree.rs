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

pub struct MerkleTree {
    root: Option<Box<Node>>,
}

impl MerkleTree {
    pub fn new(root: Option<Box<Node>>) -> Self {
        MerkleTree { root }
    }

    pub fn root(&self) -> Option<&Node> {
        self.root.as_deref()
    }

    // Get the root hash of the Merkle tree
    pub fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|node| node.hash())
    }
}
