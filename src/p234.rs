use crate::common::*;
struct Solution;

impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut ptr = head.as_mut();

        let mut list = Vec::new();
        while let Some(node) = ptr {
            list.push(node.val);
            ptr = node.as_mut().next.as_mut();
        }

        let (mut l, mut r) = (0, list.len() - 1);
        while l < r {
            if list[l] == list[r] {
                l += 1;
                r -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let ans = Solution::is_palindrome(create_list(&[1, 2, 3]));
        assert_eq!(false, ans);
    }

    #[test]
    fn test_1() {
        let ans = Solution::is_palindrome(create_list(&[1, 2, 1]));
        assert_eq!(true, ans);
    }

    #[test]
    fn test_2() {
        let ans = Solution::is_palindrome(create_list(&[1, 2, 2, 1]));
        assert_eq!(true, ans);
    }
}
