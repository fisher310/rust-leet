struct Solution;
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for value in position {
            if value % 2 == 0 {
                a += 1;
            } else {
                b += 1;
            }
        }

        a.min(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let ans = Solution::min_cost_to_move_chips(vec![1, 2, 3]);
        assert_eq!(1, ans);
    }

    #[test]
    fn test_2() {
        let ans = Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]);
        assert_eq!(2, ans);
    }

    #[test]
    fn test_1() {
        let ans = Solution::min_cost_to_move_chips(vec![1, 10000]);
        assert_eq!(1, ans);
    }
}
