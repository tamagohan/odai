pub struct BinarySearchTree<T>(BinarySearchTreeInner<T>);

pub enum BinarySearchTreeInner<T> {
    Nil,
    Node {
        val: T,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self(BinarySearchTreeInner::Nil)
    }
}

fn main() {
    println!("Hello, world!");
}
