use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut cache : HashSet<char> = HashSet::new();
        let ch = s.chars().collect::<Vec<_>>();

        let mut i = 0;
        let mut j = 0;
        let n = s.len();
        let mut ans = 0;
        while i < n && j < n {
            let c = ch[j];
            if !cache.contains(&c) {
                cache.insert(c);
                j+=1;
                ans = if ans < j - i {j - i} else {ans};
            } else {
                cache.remove(&ch[i]);
                i+=1;
            }
            
        }
        ans as i32
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let n = Solution::length_of_longest_substring("pwwkew".to_string());
        assert_eq!(3, n, "pwwkew");
        let n = Solution::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(3, n, "abcabcbb");
        let n = Solution::length_of_longest_substring("bb".to_string());
        assert_eq!(1, n, "bb");
        let n = Solution::length_of_longest_substring(" ".to_string());
        assert_eq!(1, n, "blank");
        let n = Solution::length_of_longest_substring("abcdef".to_string());
        assert_eq!(6, n, "abcdef");
        let n = Solution::length_of_longest_substring("aab".to_string());
        assert_eq!(2, n, "aab");
        let n = Solution::length_of_longest_substring("adb".to_string());
        assert_eq!(3, n, "adb");
        let n = Solution::length_of_longest_substring("cdd".to_string());
        assert_eq!(2, n, "cdd");
        
    }
}
