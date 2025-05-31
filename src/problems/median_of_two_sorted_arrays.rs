pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // * 确保 nums1 的长度小于等于 nums2 的长度
        let (m, n) = (nums1.len(), nums2.len());
        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        // * left, right 分别表示 nums1 的左右边界
        let (mut left, mut right) = (0, m);

        // * 在较短的数组中进行二分查找
        while left <= right {
            // * i, j 分别表示 nums1 和 nums2 分割点
            let i: usize = (left + right) / 2;
            let j: usize = (m + n + 1) / 2 - i;

            // * nums1:  ……………… nums1[i-1] | nums1[i] ……………………
            // * nums2:  ……………… nums2[j-1] | nums2[j] ……………………
            // * 获取分界线左右两侧的最大值和最小值
            let (max_left1, min_right1, max_left2, min_right2) = (
                if i == 0 { i32::MIN } else { nums1[i - 1] },
                if i == m { i32::MAX } else { nums1[i] },
                if j == 0 { i32::MIN } else { nums2[j - 1] },
                if j == n { i32::MAX } else { nums2[j] },
            );

            // * 检查分割是否正确
            if max_left1 <= min_right2 && max_left2 <= min_right1 {
                return if ((m + n) % 2 == 0) {
                    // * 偶数个元素，返回中间两个数的平均值
                    (max_left1.max(max_left2) + min_right1.min(min_right2)) as f64 / 2.0
                } else {
                    // * 奇数个元素，返回左半部分的最大值
                    max_left1.max(max_left2) as f64
                };
            } else if max_left1 > min_right2 {
                // * nums1 中的分界线划多了，要向左边移动
                right = i - 1;
            } else {
                // * nums1 中的分界线划少了，要向右边移动
                left = i + 1;
            }
        }

        0.0
    }
}
