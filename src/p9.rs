struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut arr = Vec::new();
        let mut i = x;
        while i != 0 {
            arr.push(i % 10);
            i /= 10;
        }

        let mut a = 0;
        let mut b = arr.len() - 1;
        while a < b {
            if arr[a] != arr[b] {
                return false;
            }
            a += 1;
            b -= 1;
        }

        true
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = Solution::is_palindrome(121);
        assert_eq!(true, ans);
    }

    #[test]
    fn test2() {
        let ans = Solution::is_palindrome(123);
        assert_eq!(false, ans);
    }

    #[test]
    fn test3() {
        let ans = Solution::is_palindrome(-121);
        assert_eq!(false, ans);
    }
    #[test]
    fn test4() {}
}
