use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // let trees_for_3 = vec![
    //     new_node(1, None, new_node(2, None, new_node(3, None, None))),
    //     new_node(1, None, new_node(3, new_node(2, None, None), None)),
    //     new_node(2, new_node(1, None, None), new_node(3, None, None)),
    //     new_node(3, new_node(1, None, new_node(2, None, None)), None),
    //     new_node(3, new_node(2, new_node(1, None, None), None), None),
    // ];
    // assert_eq!(generate_trees(3), trees_for_3);
    // assert_eq!(generate_trees(1), vec![new_node(1, None, None)]);

    // [1] [0, 2] [0, 0, 0, 3]
    assert_eq!(
        TreeNode::from(vec![1, 0, 2, 0, 0, 0, 3]),
        new_node(1, None, new_node(2, None, new_node(3, None, None)))
    );
    assert_eq!(
        TreeNode::from(vec![2, 1, 3]),
        new_node(2, new_node(1, None, None), new_node(3, None, None))
    );
    assert_eq!(
        TreeNode::from(vec![3, 2, 0, 1, 0, 0, 0]),
        new_node(3, new_node(2, new_node(1, None, None), None), None)
    );
    assert_eq!(
        TreeNode::from(vec![2, 1, 3, 0, 4]),
        new_node(
            2,
            new_node(1, None, new_node(4, None, None)),
            new_node(3, None, None)
        )
    );

    assert_eq!(TreeNode::from(vec![3]), new_node(3, None, None));
    assert_eq!(rob(TreeNode::from(vec![2, 1, 3, 0, 4])), 7);
    assert_eq!(rob(TreeNode::from(vec![3])), 3);
    assert_eq!(rob(TreeNode::from(vec![3, 2, 3, 0, 3, 0, 1])), 7);
    assert_eq!(rob(TreeNode::from(vec![3, 4, 5, 1, 3, 0, 1])), 9);
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from(xs: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if xs.is_empty() {
            None
        } else {
            let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![None; xs.len()];

            let e = (xs.len() as f64).log2().ceil() as u32;

            if e == 0 {
                new_node(xs[0], None, None)
            } else {
                for i in 0..e {
                    for j in (2_usize.pow(i) - 1)..(2_usize.pow(i + 1) - 1) {
                        if j < xs.len() && xs[j] > 0 {
                            nodes[j] = new_node(xs[j], None, None);

                            if j > 0 {
                                let pi = (j as f64 / 2.0).ceil() as usize - 1;

                                if j % 2 == 0 {
                                    nodes[pi] = nodes[pi].clone().map(|x| {
                                        x.borrow_mut().right = nodes[j].clone();
                                        x
                                    });
                                } else {
                                    nodes[pi] = nodes[pi].clone().map(|x| {
                                        x.borrow_mut().left = nodes[j].clone();
                                        x
                                    });
                                }
                            }
                        }
                    }
                }

                nodes[0].clone()
            }
        }
    }
}

pub fn new_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

/*
 *
 * [3,2,3,null,3,null,1]
 *                                      (with root, w/o root)
 *                                         3;(+3 + left[1] + right[1], max(left) + max(right))
 *  2;(+3 + left[1] + right[1], max(left) + max(right))    3;(+3 + left[1] + right[1], max(left) + max(right))
 *                                    3;(3, 0)                 1;(1, 0)
 *
 * O(n), O(logn + 2)
 */
pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn go(r: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = r {
            let left = go(&node.borrow().left);
            let right = go(&node.borrow().right);

            (
                node.borrow().val + left.1 + right.1,
                left.1.max(left.0) + right.0.max(right.1),
            )
        } else {
            (0, 0)
        }
    }

    let res = go(&root);

    res.0.max(res.1)
}
