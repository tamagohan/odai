#[derive(Debug)]
pub struct BinarySearchTree<T: std::cmp::Ord + std::fmt::Debug>(BinarySearchTreeInner<T>);

#[derive(Debug)]
enum BinarySearchTreeInner<T> {
    Nil,
    Node {
        val: T,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<T: std::cmp::Ord + std::fmt::Debug> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self(BinarySearchTreeInner::Nil)
    }

    pub fn add(&mut self, val: T) {
        let nil = Self::find_nil_to_add(&mut self.0, &val);
        *nil = BinarySearchTreeInner::Node {
            val: val,
            left: Box::new(BinarySearchTreeInner::Nil),
            right: Box::new(BinarySearchTreeInner::Nil),
        };
    }

    fn find_nil_to_add<'t, 'v>(
        cur_node: &'t mut BinarySearchTreeInner<T>,
        val: &'v T,
    ) -> &'t mut BinarySearchTreeInner<T> {
        match cur_node {
            BinarySearchTreeInner::Nil => cur_node,
            BinarySearchTreeInner::Node {
                val: cur_v,
                left,
                right,
            } => {
                if val <= cur_v {
                    Self::find_nil_to_add(right, &val)
                } else {
                    Self::find_nil_to_add(left, &val)
                }
            }
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::<i32>::new();
    tree.add(5);
    println!("{:?}", tree);
}
