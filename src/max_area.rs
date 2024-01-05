/*
11. 盛最多水的容器

给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。

找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

返回容器可以储存的最大水量。

说明：你不能倾斜容器。

示例 1：

输入：[1,8,6,2,5,4,8,3,7]
输出：49 
解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。

示例 2：

输入：height = [1,1]
输出：1
 

提示：

n == height.length
2 <= n <= 105
0 <= height[i] <= 104
*/
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            let h_left = height[left];
            let h_right = height[right];
            let d = (right - left) as i32;
            let h = h_left.min(h_right);

            let area = d * h;
            max_area = max_area.max(area);

            if h_left < h_right{
                left += 1;
            }else{
                right -= 1;
            }
        }
        max_area
    }
}


/**
 * 在这个问题中，v = min(height[i], height[j]) * (j - i)。
 * 可以使用双指针，从容器两端开始向中间迭代，每次迭代时，将指向较小值的指针向中间移动一步。
 * 每次迭代时，都可以计算出一个容量，并保留较大容量。
 * 且当迭代遇到的height值小于等于上一次迭代时的height值时，可以直接跳过。
 * 最后直到两个指针相遇，即可得到最大容量。
 */


/*
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0, height.len() - 1);
        let mut res = 0;
        while low < high {
            res = res.max(height[low].min(height[high]) * (high - low) as i32);
            if height[low] < height[high] {
                low += 1;
            } else {
                high -= 1;
            }
        }
        res
    }
}
这个算法时间效率更高。
*/