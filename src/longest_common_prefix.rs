/*
14. 最长公共前缀
已解答
简单
相关标签
相关企业
编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 ""。



示例 1：

输入：strs = ["flower","flow","flight"]
输出："fl"
示例 2：

输入：strs = ["dog","racecar","car"]
输出：""
解释：输入不存在公共前缀。


提示：

1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] 仅由小写英文字母组成
*/
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let first_str = &strs[0];

        for (i, char_i) in first_str.chars().enumerate() {
            for other_str in &strs[1..] {
                if let Some(char_other) = other_str.chars().nth(i) {
                    if char_other != char_i {
                        return result;
                    }
                } else {
                    return result;
                }
            }
            result.push(char_i);
        }
        result
    }
}
/**
 * 时间复杂度为O(n*m)，n为字符串数组长度，m为最长公共前缀长度。
 * 空间复杂度为O(1)。
 * 切入点在于拆分字符串数组，然后逐个比较。
 */