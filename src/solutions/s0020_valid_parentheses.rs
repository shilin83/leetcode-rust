struct Solution;

impl Solution {
    fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                // * 遇到左括号，将对应的右括号入栈
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                // * 遇到右括号，检查是否与栈顶元素匹配
                // * 其他字符都是非法的
                _ => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let cases: [(String, bool); 4] = [
            ("()".to_string(), true),
            ("()[]{}".to_string(), true),
            ("(]".to_string(), false),
            ("([])".to_string(), true),
        ];

        for (s, expected) in cases {
            let actual = Solution::is_valid(s);

            assert_eq!(expected, actual);
        }
    }
}
