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

impl<T: std::cmp::PartialEq + Copy> BinaryTree<T> {
    pub fn replace(&mut self, to: Self) -> () {
        *self = to;
    }

    pub fn delete(&mut self) -> () {
        self.replace(Self::Nil)
    }

    pub fn search_depth_first(&self, v: T) -> bool {
        match self {
            BinaryTree::Nil => return false,
            BinaryTree::Node { val, left, right } => {
                if v == *val {
                    return true;
                }
                if left.search_depth_first(v) {
                    return true;
                }
                if right.search_depth_first(v) {
                    return true;
                }
                return false;
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_tree_1() -> BinaryTree<i32> {
        //       5
        //      /
        //     4
        //    /
        //   11
        //  /  \
        // 7    2
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
    }

    fn gen_tree_2() -> BinaryTree<i32> {
        //   8
        //  / \
        // 13  4
        //      \
        //       1
        bin_tree! {
            val: 8,
            left: bin_tree! { val: 13 },
            right: bin_tree! {
                val: 4,
                right: bin_tree! { val: 1 },
            },
        }
    }

    fn gen_tree_3() -> BinaryTree<i32> {
        // tree1.root.right + tree2:
        //       5
        //      / \
        //     4   8
        //    /   / \
        //   11  13  4
        //  /  \      \
        // 7    2      1
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
            right: bin_tree! {
                val: 8,
                left: bin_tree! { val: 13 },
                right: bin_tree! {
                    val: 4,
                    right: bin_tree!{ val: 1 },
                },
            },
        }
    }
    #[test]
    fn test_replace() {
        let mut tree1 = gen_tree_1();
        let tree2 = gen_tree_2();
        let tree3 = gen_tree_3();

        if let BinaryTree::Node { right, .. } = &mut tree1 {
            &right.replace(tree2);
        }
        assert_eq!(&tree1, &tree3);
    }

    #[test]
    fn test_delete() {
        let tree1 = gen_tree_1();
        let mut tree3 = gen_tree_3();

        if let BinaryTree::Node { right, .. } = &mut tree3 {
            (&mut (**right)).delete()
        }
        assert_eq!(tree3, tree1);
    }

    #[test]
    fn test_search_depth_first() {
        let tree3 = gen_tree_3();
        assert_eq!((tree3.search_depth_first(11)), true);
        assert_eq!((tree3.search_depth_first(3)), false);

        assert_eq!((BinaryTree::Nil.search_depth_first(11)), false);
    }
}