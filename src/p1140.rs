struct Solution;
///
/// X = max(X, M)
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let ans = Solution::stone_game_ii(vec![2, 7, 9, 4, 4]);
        assert_eq!(10, ans, "first test case.");
    }

    #[test]
    fn test1() {
        let ans = Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]);
        assert_eq!(104, ans, "second test case");
    }
}
