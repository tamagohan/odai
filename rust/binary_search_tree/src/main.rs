#[derive(Debug, PartialEq)]
pub struct BinarySearchTree<T: std::cmp::Ord + std::fmt::Debug>(BinarySearchTreeInner<T>);

#[derive(Debug, PartialEq)]
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
                    Self::find_nil_to_add(left, &val)
                } else {
                    Self::find_nil_to_add(right, &val)
                }
            }
        }
    }

    pub fn contains(&self, val: &T) -> bool {
        Self::contains_imp(&self.0, val)
    }

    fn contains_imp(tree: &BinarySearchTreeInner<T>, val: &T) -> bool {
        match tree {
            BinarySearchTreeInner::Nil => false,
            BinarySearchTreeInner::Node {
                val: cur_val,
                left,
                right,
            } => {
                if cur_val == val {
                    true
                } else {
                    BinarySearchTree::contains_imp(left, val)
                        || BinarySearchTree::contains_imp(right, val)
                }
            }
        }
    }
}

fn main() {}

#[test]
fn test_add_to_leaf() {
    use BinarySearchTreeInner::*;
    let mut tree = BinarySearchTree::<i32>::new();

    // 5
    tree.add(5);
    assert_eq!(
        tree,
        BinarySearchTree(Node {
            val: 5,
            left: Box::new(Nil),
            right: Box::new(Nil)
        })
    );

    //   5
    //  /
    // 3
    tree.add(3);
    assert_eq!(
        tree,
        BinarySearchTree(Node {
            val: 5,
            left: Box::new(Node {
                val: 3,
                left: Box::new(Nil),
                right: Box::new(Nil)
            }),
            right: Box::new(Nil)
        })
    );

    //     5
    //    /
    //   3
    //  /
    // 3
    tree.add(3);
    assert_eq!(
        tree,
        BinarySearchTree(Node {
            val: 5,
            left: Box::new(Node {
                val: 3,
                left: Box::new(Node {
                    val: 3,
                    left: Box::new(Nil),
                    right: Box::new(Nil)
                }),
                right: Box::new(Nil)
            }),
            right: Box::new(Nil)
        })
    );

    //     5
    //    /
    //   3
    //  / \
    // 3   4
    tree.add(4);
    assert_eq!(
        tree,
        BinarySearchTree(Node {
            val: 5,
            left: Box::new(Node {
                val: 3,
                left: Box::new(Node {
                    val: 3,
                    left: Box::new(Nil),
                    right: Box::new(Nil)
                }),
                right: Box::new(Node {
                    val: 4,
                    left: Box::new(Nil),
                    right: Box::new(Nil)
                })
            }),
            right: Box::new(Nil)
        })
    );
}

#[test]
fn test_in_same_order() {
    let mut tree1 = BinarySearchTree::<i32>::new();
    tree1.add(1);
    tree1.add(2);

    let mut tree2 = BinarySearchTree::<i32>::new();
    tree2.add(1);
    tree2.add(2);

    assert_eq!(tree1, tree2);
}

#[test]
fn test_in_different_order1() {
    let mut tree1 = BinarySearchTree::<i32>::new();
    tree1.add(1);
    tree1.add(2);

    let mut tree2 = BinarySearchTree::<i32>::new();
    tree2.add(2);
    tree2.add(1);

    assert_ne!(tree1, tree2);
}

#[test]
fn test_in_different_order2() {
    let mut tree1 = BinarySearchTree::<i32>::new();
    tree1.add(5);
    tree1.add(3);
    tree1.add(7);

    let mut tree2 = BinarySearchTree::<i32>::new();
    tree2.add(5);
    tree2.add(7);
    tree2.add(3);

    assert_eq!(tree1, tree2);
}

#[test]
fn test_contains1() {
    let mut tree = BinarySearchTree::<i32>::new();
    // tree only has nil node
    assert!(!tree.contains(&1));

    // tree is below.
    //     5
    //    / \
    //   3   7
    tree.add(5);
    tree.add(3);
    tree.add(7);
    assert!(tree.contains(&5));
    assert!(!tree.contains(&1));
}

#[test]
fn test_contains2() {
    use BinarySearchTreeInner::*;
    // tree is below.
    //     5
    //    / \
    //   5   5
    //        \
    //         7
    let tree = BinarySearchTree(Node {
        val: 5,
        left: Box::new(Node {
            val: 5,
            left: Box::new(Nil),
            right: Box::new(Nil),
        }),
        right: Box::new(Node {
            val: 5,
            left: Box::new(Nil),
            right: Box::new(Node {
                val: 7,
                left: Box::new(Nil),
                right: Box::new(Nil),
            }),
        }),
    });
    assert!(tree.contains(&5));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&3));
}
