use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let trees_for_3 = vec![
        new_node(1, None, new_node(2, None, new_node(3, None, None))),
        new_node(1, None, new_node(3, new_node(2, None, None), None)),
        new_node(2, new_node(1, None, None), new_node(3, None, None)),
        new_node(3, new_node(1, None, new_node(2, None, None)), None),
        new_node(3, new_node(2, new_node(1, None, None), None), None),
    ];
    assert_eq!(generate_trees(3), trees_for_3);
    assert_eq!(generate_trees(1), vec![new_node(1, None, None)]);
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
}

/*
| n = 4
| [1,2,3,4]
|
|            1                               2                3                            4
|  [2,           3,              4]      [1]   [3,  4]   [1,  2]   [4]    [1,          2,          3]
|    [3,  4]  [2]  [4]   [2,   3]               4 3       2 1             [2  ,3]    1  3  [1,   2]
|      4 3                 3  2                                            3 2              2  1
|
| brute force
| O(n^n), O(1)
|
| cache
|
| [], [1], [2], [3], [4], [1, 2], [2, 3], [3, 4], [1,2,3], [2,3,4], [1,2,3,4]
| 1 + n + n - 1 + n - 2 + n - 3 + ... 1 =  n*(n+1)/2
|
| HashMap[set] = [trees]
| hm[[1,2]] = [1 -> 2, 2 <- 1]
|
| hm[[]] = [0]
| hm[[1]] = [1]
| ...
| hm[[1,2]] = [0 <- 1 -> 2, 1 <- 2 -> 0]
| hm[[2,3]] = [0 <- 2 -> 3, 2 <- 3 -> 0]
| hm[[3,4]] = [0 <- 3 -> 4, 3 -> 4 -> 0]
| hm[[1,2,3]] = [hm[0] <- 1 -> hm[[2,3]], hm[[1]] <- 2 -> hm[[3]], hm[[1,2]] <- 3 -> hm[[]]]
| ...
|
| O(n*(n+1)/2 = n^2), O(n*(n+1)/2 = n^2)
 */
pub fn new_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut hm = std::collections::HashMap::<&[i32], Vec<Option<Rc<RefCell<TreeNode>>>>>::new();

    let a = (1..=n).collect::<Vec<_>>();

    hm.insert(&[], vec![None]);

    for i in 1..=a.len() {
        for nodes in a.windows(i) {
            let mut trees: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

            for j in 0..nodes.len() {
                let left_trees = hm.get(&nodes[..j]).unwrap();
                let right_trees = hm.get(&nodes[j + 1..]).unwrap();

                for left in left_trees {
                    for right in right_trees {
                        trees.push(new_node(nodes[j], left.clone(), right.clone()))
                    }
                }
            }

            hm.insert(nodes, trees);
        }
    }

    hm.get(a.as_slice()).unwrap().clone()
}
