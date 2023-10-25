struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n < 0 {
            panic!("n should not be negative");
        }
        match n {
            0 => 1,
            n => {
                let mut ans = 10;
                let mut sum = 9;
                let mut base = 9;
                for i in 1..n {
                    ans += {
                        sum *= base;
                        sum
                    };
                    base -= 1;
                }

                ans
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let ans = Solution::count_numbers_with_unique_digits(2);
        assert_eq!(ans, 91);
    }

    #[test]
    fn test1() {
        let ans = Solution::count_numbers_with_unique_digits(0);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test2() {
        let ans = Solution::count_numbers_with_unique_digits(3);
        assert_eq!(ans, 739);
    }
}
