struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut p = time_series[0] + duration - 1;
        let mut pre = time_series[0];
        let mut ans = 0;
        for t in &time_series[1..] {
            let t = *t;
            if p < t {
                // not in p
                ans += duration;
                p = t + duration - 1;
                pre = t;
            } else {
                // in p
                ans += t - pre;
                p = t + duration - 1;
                pre = t;
            }
        }

        if p > 0 {
            ans + duration
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        let ans = Solution::find_poisoned_duration(vec![1, 4], 2);
        assert_eq!(4, ans);
    }
    #[test]
    fn test1() {
        let ans = Solution::find_poisoned_duration(vec![1, 2], 2);
        assert_eq!(3, ans);
    }

    #[test]
    fn test2 () {
        let ans = Solution::find_poisoned_duration(vec![0, 1], 0);
        assert_eq!(0, ans);
        
    }
}
