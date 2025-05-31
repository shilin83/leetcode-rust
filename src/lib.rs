mod problems;
mod structures;

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

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
