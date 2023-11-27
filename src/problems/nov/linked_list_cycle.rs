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
        
    
    false
}: