use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new();
        for (i, &element) in nums.iter().enumerate() {
            let complement = target - element;
            if let Some(&index) = num_map.get(&complement) {
                return vec![index as i32, i as i32];
            }
            num_map.insert(element, i);
        }
        vec![]
    }
}
/*
这种算法的时间效率更高，因为只需要遍历一次数组，而不是两次。
*/
