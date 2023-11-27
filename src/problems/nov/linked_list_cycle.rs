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

pub fn linked_list_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut fast = head.clone();
    let mut slow = head.clone();

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        fast = fast.unwrap().next.unwrap().next;
        slow = slow.unwrap().next;
        if fast == slow {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // Returns false for a linked list with a cycle of length 1
    #[test]
    fn test_no_cycle_linked_list() {
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut current = &mut head;
        for i in 2..=5 {
            let node = current.as_mut().unwrap();
            node.next = Some(Box::new(ListNode::new(i)));
            current = &mut node.next;
        }
        assert_eq!(linked_list_cycle(head), false);
    }

    #[test]
    fn test_empty_linked_list() {
        let head = None;
        assert_eq!(linked_list_cycle(head), false);
    }
}
