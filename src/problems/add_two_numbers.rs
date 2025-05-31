use crate::structures::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // * 创建虚拟头节点作为结果链表的起点
        let mut head: ListNode = ListNode::new(0);
        let (mut current, mut n1, mut n2, mut carry) = (&mut head, l1, l2, 0_u8);

        while n1.is_some() || n2.is_some() || carry != 0 {
            // * 计算两个链表当前节点值与进位值的和
            let sum: u8 =
                (n1.as_ref().map_or(0, |n| n.val)) + (n2.as_ref().map_or(0, |n| n.val)) + carry;
            n1 = n1.and_then(|n| n.next);
            n2 = n2.and_then(|n| n.next);

            // * 创建新节点并添加到结果链表中
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut()?;

            // * 更新进位值
            carry = sum / 10;
        }

        head.next
    }
}
