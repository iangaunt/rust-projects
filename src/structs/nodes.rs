#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode { val, left: None, right: None }
    }
    
    pub fn set_left(&mut self, node: TreeNode<T>) {
        self.left = Some(Box::new(node));
    }

    pub fn set_right(&mut self, node: TreeNode<T>) {
        self.right = Some(Box::new(node));
    }

    pub fn build_tree(&mut self, vec: Vec<T>) {
        let mut queue: Vec<Box<TreeNode<T>>> = Vec::new();
    }
}