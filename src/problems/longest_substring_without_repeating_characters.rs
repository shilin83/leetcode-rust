use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut chars, mut chars_map, mut max_len, mut left) = (
            s.chars().collect::<Vec<char>>(),
            HashMap::<char, usize>::with_capacity(s.len()),
            0,
            0,
        );

        for (right, c) in chars.iter().enumerate() {
            // * 如果哈希表中存在当前元素，则移动滑动窗口左边界到当前元素的下一个位置
            if let Some(&idx) = chars_map.get(&c) {
                if idx >= left {
                    left = idx + 1;
                }
            }

            // * 将当前元素及其索引存入哈希表
            chars_map.insert(*c, right);

            // * 更新滑动窗口最大长度
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
