#[macro_export]
macro_rules! bin_tree {
    ( val: $val:expr, left: $left:expr, right: $right:expr $(,)? ) => {
        BinaryTree::Node {
            val: $val,
            left: Box::new($left),
            right: Box::new($right),
        }
    };

    ( val: $val:expr, right: $right:expr $(,)? ) => {
        bin_tree! {
            val: $val,
            left: bin_tree!(),
            right: $right,
        }
    };

    ( val: $val:expr, left: $left:expr $(,)? ) => {
        bin_tree! {
            val: $val,
            left: $left,
            right: bin_tree!(),
        }
    };

    ( val: $val:expr $(,)? ) => {
        bin_tree!(val: $val, left: bin_tree!(), right: bin_tree!(),)
    };

    () => {
        BinaryTree::Nil
    };
}

#[derive(Debug, PartialEq)]
pub enum BinaryTree<T> {
    Nil,
    Node {
        val: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> BinaryTree<T> {
    pub fn replace(&mut self, to: Self) -> () {
        *self = to;
    }

    pub fn delete(&mut self) -> () {
        self.replace(Self::Nil)
    }
}

fn main() {
    let tree = bin_tree! {
        val: 5,
        left: bin_tree! {
            val: 4,
            left: bin_tree!{ val: 11 },
        },
        right: bin_tree!{ val: 8 },
    };
    println!("{:?}", tree);
}

#[test]
fn test_replace() {
    // tree1:
    //       5
    //      /
    //     4
    //    /
    //   11
    //  /  \
    // 7    2
    //
    // tree2:
    //         8
    //        / \
    //       13  4
    //            \
    //             1
    //
    // tree3 = tree1.root.right + tree2:
    //       5
    //      / \
    //     4   8
    //    /   / \
    //   11  13  4
    //  /  \      \
    // 7    2      1
    //

    let mut tree1 = bin_tree! {
        val: 5,
        left: bin_tree! {
            val: 4,
            left: bin_tree! {
                val: 11,
                left: bin_tree! { val: 7 },
                right: bin_tree! { val: 2 },
            },
        },
    };

    let tree2 = bin_tree! {
        val: 8,
        left: bin_tree! { val: 13 },
        right: bin_tree! {
            val: 4,
            right: bin_tree! { val: 1 },
        },
    };

    let tree3 = bin_tree! {
        val: 5,
        left: bin_tree! {
            val: 4,
            left: bin_tree! {
                val: 11,
                left: bin_tree! { val: 7 },
                right: bin_tree! { val: 2 },
            },
        },
        right: bin_tree! {
            val: 8,
            left: bin_tree! { val: 13 },
            right: bin_tree! {
                val: 4,
                right: bin_tree!{ val: 1 },
            },
        },
    };
    if let BinaryTree::Node { right, .. } = &mut tree1 {
        &right.replace(tree2);
    }
    assert_eq!(&tree1, &tree3);

    if let BinaryTree::Node { right, .. } = &mut tree1 {
        (&mut (**right)).delete()
    }
    assert_eq!(
        tree1,
        bin_tree! {
            val: 5,
            left: bin_tree! {
                val: 4,
                left: bin_tree! {
                    val: 11,
                    left: bin_tree! { val: 7 },
                    right: bin_tree! { val: 2 },
                },
            },
        }
    )
}
