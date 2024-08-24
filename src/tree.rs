#[derive(Clone)]
pub struct Node {
    pub hash: Vec<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub parent: Option<Box<Node>>,
}

impl Node {
    pub fn new(
        hash: Vec<u8>,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        parent: Option<Box<Node>>,
    ) -> Self {
        Node {
            hash,
            left,
            right,
            parent,
        }
    }

    pub fn duplicate(&self) -> Node {
        Node {
            hash: self.hash.clone(),
            left: self.left.as_ref().map(|left| Box::new(left.duplicate())),
            right: self.right.as_ref().map(|right| Box::new(right.duplicate())),
            parent: self
                .parent
                .as_ref()
                .map(|parent| Box::new(parent.duplicate())),
        }
    }

    fn hash(&self) -> &Vec<u8> {
        &self.hash
    }
}

pub struct MerkleTree {
    pub root: Option<Box<Node>>,
}

impl MerkleTree {
    pub fn new(root: Option<Box<Node>>) -> Self {
        MerkleTree { root }
    }

    pub fn root(&self) -> Option<&Node> {
        self.root.as_deref()
    }

    pub fn root_hash(&self) -> Option<&Vec<u8>> {
        self.root.as_ref().map(|node| node.hash())
    }
}
