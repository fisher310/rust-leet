struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars = s.chars().collect::<Vec<_>>();

        let mut start: usize = 0;
        let mut end = 0;
        for i in 0..chars.len() - 1 {
            let res1 = expand(&chars, i, i);
            let res2 = expand(&chars, i, i + 1);
            if let Some((a, b)) = res1 {
                if b - a > end - start {
                    start = a;
                    end = b;
                }
            }

            if let Some((a, b)) = res2 {
                if b - a > end - start {
                    start = a;
                    end = b;
                }
            }
        }

        s[start as usize..(end + 1) as usize].to_string()
    }
}

fn expand(chars: &Vec<char>, mut i: usize, mut j: usize) -> Option<(usize, usize)> {
    let mut has = false;
    let mut added = false;
    loop {
        if chars[i] == chars[j] {
            has = true;
            if i == 0 || j == chars.len() - 1 {
                added = false;
                break;
            }
        } else {
            break;
        }
        added = true;
        i -= 1;
        j += 1;
    }

    if has {
        if added {
            Some((i + 1, j - 1))
        } else {
            Some((i, j))
        }
    } else {
        None
    }
}

mod tests {

    use super::*;

    #[test]
    fn test() {
        let ans = Solution::longest_palindrome("babad".to_string());
        assert_eq!("bab", ans);
    }

    #[test]
    fn test2() {
        let ans = Solution::longest_palindrome("cbbd".to_string());
        assert_eq!("bb", ans);
    }

    #[test]
    fn test3() {
        let ans = Solution::longest_palindrome("abcba".to_string());
        assert_eq!("abcba", ans);
    }
}
