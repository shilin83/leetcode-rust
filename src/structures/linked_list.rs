pub struct ListNode {
    pub val: u8,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: u8) -> Self {
        ListNode { val, next: None }
    }
}

pub fn to_list(nums: Vec<u8>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;

    for num in nums {
        let mut node: ListNode = ListNode::new(num);
        node.next = head;
        head = Some(Box::new(node));
    }

    head
}

pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<u8> {
    let (mut result, mut current) = (vec![], head);

    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }

    result
}
