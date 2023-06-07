struct Solution;
impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let m = logs.len();

        let mut ans = logs[0][0];
        let mut time = logs[0][1];
        for i in 1..m {
            let t = logs[i][1] - logs[i - 1][1];
            if t > time {
                ans = logs[i][0];
                time = t;
            } else if t == time {
                ans = ans.min(logs[i][0]);
            }
        }

        ans
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let x = Solution::hardest_worker(10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]);
        assert_eq!(1, x, "1");
    }

    #[test]
    fn test2() {
        let x = Solution::hardest_worker(26, vec![vec![1, 1], vec![3, 7], vec![2, 12]]);
        assert_eq!(3, x);
    }
    #[test]
    fn test3() {
        let x = Solution::hardest_worker(2, vec![vec![0,10], vec![1,20]]);
        assert_eq!(0, x);
    }
}
