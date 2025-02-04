use std::collections::HashMap;

struct Solution;

impl Solution {
    fn roman_to_int(s: String) -> i32 {
        // * 创建罗马数字到整数的映射
        let (hash_table, chars) = (
            HashMap::<char, i32>::from([
                ('I', 1),
                ('V', 5),
                ('X', 10),
                ('L', 50),
                ('C', 100),
                ('D', 500),
                ('M', 1000),
            ]),
            s.chars().collect::<Vec<char>>(),
        );

        let (mut result, mut prev) = (0, 0);

        // * 从右向左遍历，处理特殊规则（如IV = 5-1 = 4）
        for c in chars.iter().rev() {
            let curr = hash_table.get(c).unwrap();
            // * 如果当前值小于前一个值，说明是特殊情况（如IV）
            if curr < &prev {
                result -= curr;
            } else {
                result += curr;
            }

            prev = *curr;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let cases: [(String, i32); 5] = [
            ("III".to_string(), 3),
            ("IV".to_string(), 4),
            ("IX".to_string(), 9),
            ("LVIII".to_string(), 58),
            ("MCMXCIV".to_string(), 1994),
        ];

        for (s, expected) in cases {
            let actual = Solution::roman_to_int(s);

            assert_eq!(expected, actual);
        }
    }
}
