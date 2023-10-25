struct Solution;
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut ans = 0;
        for c in s.chars() {
            if c == 'L' {
                l += 1;
            } else {
                r += 1;
            }

            if l == r {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let ans = Solution::balanced_string_split("RLRRLLRLRL".to_string());
        assert_eq!(4, ans);
    }

    #[test]
    fn test1() {
        let ans = Solution::balanced_string_split("RLRRRLLRLL".to_string());
        assert_eq!(2, ans);
    }

    #[test]
    fn test2() {
        let ans = Solution::balanced_string_split("LLLLRRRR".to_string());
        assert_eq!(1, ans);
    }
}
