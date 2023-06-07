
struct Solution;

use std::collections::HashMap;
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if map.contains_key(&(target - n)) {
                return vec![*map.get(&(target-n)).unwrap(), i as i32]
            }
            map.insert(n, i as i32);
        }
        vec![]
    }
}