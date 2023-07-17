fn main() {
    assert_eq!(
        Solution::add_two_numbers(
            ListNode::bo(7, ListNode::bo(2, ListNode::bo(4, ListNode::bo(3, None)))),
            ListNode::bo(5, ListNode::bo(6, ListNode::bo(4, None)))
        ),
        ListNode::bo(7, ListNode::bo(8, ListNode::bo(0, ListNode::bo(7, None))))
    );
    assert_eq!(
        Solution::add_two_numbers(
            ListNode::bo(2, ListNode::bo(4, ListNode::bo(3, None))),
            ListNode::bo(5, ListNode::bo(6, ListNode::bo(4, None)))
        ),
        ListNode::bo(8, ListNode::bo(0, ListNode::bo(7, None)))
    );
    assert_eq!(
        Solution::add_two_numbers(ListNode::bo(0, None), ListNode::bo(0, None)),
        ListNode::bo(0, None)
    );
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn bo(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: val,
            next: next,
        }))
    }
}

struct Solution;

impl Solution {
    fn bln(val: i32, next: Option<Box<ListNode>>) -> Box<ListNode> {
        Box::new(ListNode {
            val: val,
            next: next,
        })
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut s1 = vec![];
        let mut s2 = vec![];

        let mut cur1 = l1;
        while let Some(node) = cur1 {
            s1.push(node.val);
            cur1 = node.next;
        }

        let mut cur2 = l2;
        while let Some(node) = cur2 {
            s2.push(node.val);
            cur2 = node.next;
        }

        let mut up = 0;
        let mut next = None;
        while !s1.is_empty() || !s2.is_empty() {
            let val = up
                + match (s1.pop(), s2.pop()) {
                    (None, None) => 0,
                    (None, Some(v2)) => v2,
                    (Some(v1), None) => v1,
                    (Some(v1), Some(v2)) => v1 + v2,
                };

            up = val / 10;
            next = Some(Solution::bln(val % 10, next));
        }

        if up > 0 {
            Some(Solution::bln(up, next))
        } else {
            next
        }
    }
}
