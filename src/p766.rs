use crate::common::*;

struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        let len = m.min(n);

        for i in 0..len {
            let p = (i, 0);
            if !Self::helper(&matrix, p, len) {
                return false;
            }
            let p = (0, i);
            if !Self::helper(&matrix, p, len) {
                return false;
            }
        }

        true
    }

    fn helper(matrix: &Vec<Vec<i32>>, mut p: (usize, usize), len: usize) -> bool {
        let value = matrix[p.0][p.1];
        while p.0 < len && p.1 < len {
            if matrix[p.0][p.1] == value {
                p.0 += 1;
                p.1 += 1;
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
        // [[1,2,3,4],[5,1,2,3],[9,5,1,2]]
        let ans = Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2],
        ]);
        assert_eq!(true, ans);
    }

    #[test]
    fn test_1() {
        // [[1,2], [2,2]]
        let ans = Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]);
        assert_eq!(false, ans);
    }
}
