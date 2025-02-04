use std::collections::HashMap;

struct Solution;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_table = HashMap::<i32, i32>::with_capacity(nums.len());

        for (key, value) in nums.iter().enumerate() {
            // * 计算目标值与当前元素的差值
            let diff = target - value;

            // * 如果哈希表中存在差值，则返回差值与当前元素的索引
            if let Some(idx) = hash_table.get(&diff) {
                return vec![*idx, key as i32];
            }

            // * 将当前元素及其索引存入哈希表
            hash_table.insert(*value, key as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let cases: [(Vec<i32>, i32, Vec<i32>); 4] = [
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
            (vec![1, 2], 4, vec![]),
        ];

        for (nums, target, expected) in cases {
            let actual = Solution::two_sum(nums, target);

            assert_eq!(expected, actual);
        }
    }
}
