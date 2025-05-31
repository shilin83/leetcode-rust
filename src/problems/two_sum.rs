use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut nums_vec, mut nums_map) = (
            Vec::<i32>::with_capacity(2),
            HashMap::<i32, usize>::with_capacity(nums.len()),
        );

        for (i, num) in nums.iter().enumerate() {
            // * 计算目标值与当前元素的差值
            let complement: i32 = target - num;

            // * 如果哈希表中存在差值，则返回差值与当前元素的索引
            if let Some(&j) = nums_map.get(&complement) {
                nums_vec.push(j as i32);
                nums_vec.push(i as i32);
                break;
            } else {
                // * 否则，将当前元素及其索引存入哈希表
                nums_map.insert(*num, i);
            }
        }

        nums_vec
    }
}
