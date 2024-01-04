impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let mut char_index = HashMap::new();
        let mut left = 0;
        let mut max_len = 0;

        for (right, char) in s.chars().enumerate() {
            if let Some(prev_index) = char_index.get(&char) {
                //如果当前字符已经出现过，那么更新左边界
                left = left.max(*prev_index + 1);
            }
            //更新当前字符的位置
            char_index.insert(char, right);
            //更新最大长度
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
/**
 * 这个算法比另一个快了许多，而空间复杂度相同，故而更加优秀。
 * 主要是因为这个算法只需要遍历一次字符串，而另一个算法需要遍历两次。
 * 且使用HashMap而非HashSet，所以不需要时常增减元素。
 */
