/*
3. 无重复字符的最长子串

示例 1:

输入: s = "abcabcbb"
输出: 3 
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
示例 2:

输入: s = "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
示例 3:

输入: s = "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 

提示：

0 <= s.length <= 5 * 104
s 由英文字母、数字、符号和空格组成
*/

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let (mut left, mut right) = (0, 0);
        let mut max_len = 0;

        while right < s.len(){
            if !set.contains(&s.chars().nth(right).unwrap()){
                set.insert(s.chars().nth(right).unwrap());
                right += 1;
                max_len = max_len.max(right - left); 
            }else{
                set.remove(&s.chars().nth(left).unwrap());
                left += 1;
            }
        }
        max_len as i32
    }
}
/**
 * 这种方法时间效率太低
 */