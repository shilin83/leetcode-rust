use crate::structures::ListNode;

struct Solution;

impl Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let (mut current, mut p1, mut p2, mut carry) = (&mut head, l1, l2, 0_u8);

        // * 遍历两个链表，计算每个节点的和，并与当前进位值相加
        // * 遍历结束后，如果还有进位值，则在链表末尾添加新的节点
        while p1.is_some() || p2.is_some() || carry != 0 {
            let sum =
                (p1.as_ref().map_or(0, |n| n.val)) + (p2.as_ref().map_or(0, |n| n.val)) + carry;

            p1 = p1.and_then(|n| n.next);
            p2 = p2.and_then(|n| n.next);

            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            current = &mut current.as_mut().unwrap().next;
            carry = sum / 10;
        }

        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::structures::int_2_list;

    #[test]
    fn test_add_two_numbers() {
        let cases: [(Vec<u8>, Vec<u8>, Vec<u8>); 3] = [
            (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
            (vec![0], vec![0], vec![0]),
            (
                vec![9, 9, 9, 9, 9, 9, 9],
                vec![9, 9, 9, 9],
                vec![8, 9, 9, 9, 0, 0, 0, 1],
            ),
        ];

        for (nums1, nums2, expected) in cases {
            assert_eq!(
                int_2_list(expected),
                Solution::add_two_numbers(int_2_list(nums1), int_2_list(nums2)),
            )
        }
    }
}
