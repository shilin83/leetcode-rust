#[derive(Debug, Eq, PartialEq)]
pub struct ListNode {
    pub val: u8,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: u8) -> Self {
        ListNode { next: None, val }
    }
}

pub fn int_2_list(nums: Vec<u8>) -> Option<Box<ListNode>> {
    let length = nums.len();

    let mut head: Option<Box<ListNode>> = None;

    if length != 0 {
        for &num in nums.iter().rev() {
            let mut node = ListNode::new(num);
            node.next = head;
            head = Some(Box::new(node));
        }
    }

    head
}
