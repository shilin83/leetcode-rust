mod problems;
mod structures;

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_two_numbers() {
        use crate::problems::add_two_numbers::Solution;
        use crate::structures::linked_list::*;

        let cases: [(Vec<u8>, Vec<u8>, Vec<u8>); 3] = [
            (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
            (vec![0], vec![0], vec![0]),
            (
                vec![9, 9, 9, 9, 9, 9, 9],
                vec![9, 9, 9, 9],
                vec![8, 9, 9, 9, 0, 0, 0, 1],
            ),
        ];

        for (l1, l2, expected) in cases {
            let actual: Option<Box<ListNode>> = Solution::add_two_numbers(to_list(l1), to_list(l2));

            assert_eq!(to_vec(actual), expected);
        }
    }

    #[test]
    fn test_two_sum() {
        use crate::problems::two_sum::Solution;

        let cases: [(Vec<i32>, i32, Vec<i32>); 3] = [
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];

        for (nums, target, expected) in cases {
            let actual: Vec<i32> = Solution::two_sum(nums, target);

            assert_eq!(actual, expected);
        }
    }
}
