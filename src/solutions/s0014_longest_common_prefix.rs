struct Solution;

impl Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        // * 以第一个字符串为基准
        let mut prefix = strs[0].clone();

        let mut i = 1;
        while i < strs.len() {
            let mut j = 0;
            while j < prefix.len() && j < strs[i].len() {
                // * 逐个字符比较
                if prefix.chars().nth(j).unwrap() != strs[i].chars().nth(j).unwrap() {
                    prefix = prefix[..j].to_string();
                    break;
                }

                j += 1;
            }

            i += 1;
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let cases: [(Vec<String>, String); 2] = [
            (
                vec![
                    "flower".to_string(),
                    "flow".to_string(),
                    "flight".to_string(),
                ],
                "fl".to_string(),
            ),
            (
                vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
                "".to_string(),
            ),
        ];

        for (strs, expected) in cases {
            let actual = Solution::longest_common_prefix(strs);

            assert_eq!(expected, actual);
        }
    }
}
