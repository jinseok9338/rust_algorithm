// https://leetcode.com/problems/add-two-numbers/
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
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut dummy_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut dummy_head;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() {
        let mut sum = carry;
        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }
        carry = sum / 10;

        if let Some(node) = tail {
            node.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = &mut node.next;
        }
    }

    if carry > 0 {
        if let Some(node) = tail {
            node.next = Some(Box::new(ListNode::new(carry)));
        }
    }
    dummy_head.unwrap().next
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(8))),
            })),
        }));
        assert_eq!(add_two_numbers(l1, l2), expected);
    }
}
