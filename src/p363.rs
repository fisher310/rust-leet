struct Solution;

use std::collections::BTreeSet;
use std::ops::Bound::Included;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = -1000000;

        // upper bound
        for i in 0..m {
            let mut sum = vec![0; n];
            // lower bound
            for j in i..m {
                for k in 0..n {
                    sum[k] += matrix[j][k];
                }
                let mut sum_set = BTreeSet::new();
                sum_set.insert(0);
                let mut s = 0;
                for v in sum.iter() {
                    s += v;
                    let mut it = sum_set.range((Included(&(s - k)), Included(&i32::MAX)));
                    if let Some(value) = it.next() {
                        ans = ans.max(s - *value);
                    }
                    sum_set.insert(s);
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use core::num;
    use std::sync::Mutex;

    use crate::solutions;

    use super::*;
    #[test]
    fn test_0() {
        // [[1,0,1],[0,-2,3]], k = 2
        let ans = Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_1() {
        let ans = Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3);
        assert_eq!(3, ans);
    }

    #[test]
    fn test_3() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();
        let evens = numbers.filter(|x| x % 2 == 0);
        let even_square = evens.clone().map(|x| x * x);
        let result = even_square.clone().collect::<Vec<_>>();
        println!("{:?}", result);
        println!("{:?}\n{:?}", evens, even_square);
    }

    #[test]
    fn test_4() {
        let value = Mutex::new(32);
        *value.lock().unwrap() += 1;
        println!("{:?}", value);
        dbg!(value);
    }

    #[test]
    fn test_5() {
        union hello_int {
            a: u32,
            b: f32,
            c: f64,
        }

        let x = hello_int { a: 11112128 };
        unsafe {
            println!("{:?}", x.b);
            println!("{:?}", x.c);
        }
    }
}
