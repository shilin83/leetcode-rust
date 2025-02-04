struct Solution;

impl Solution {
    fn is_palindrome(x: i32) -> bool {
        // * 负数或最后一位是 0 且不是 0 本身的数字不是回文数
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let (mut num, mut reverted) = (x, 0);

        // * 当原始数字大于反转后的数字时, 说明还没处理到一半
        while num > reverted {
            reverted = reverted * 10 + num % 10;
            num /= 10;
        }

        // * 当数字长度为偶数时，1221 -> x = 12, reversed = 12
        // * 当数字长度为奇数时，12321 -> x = 12, reversed = 123
        num == reverted || num == reverted / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let cases: [(i32, bool); 3] = [(121, true), (-121, false), (10, false)];

        for (x, expected) in cases {
            let actual = Solution::is_palindrome(x);

            assert_eq!(expected, actual);
        }
    }
}
