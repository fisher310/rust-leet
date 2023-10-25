struct Solution;
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        if trips.is_empty() {
            return true;
        }
        let mut trips = trips;
        trips.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut nums = 0;
        let mut bus: Vec<&Vec<i32>> = Vec::new();
        for i in 0..trips.len() {
            bus.sort_by(|a, b| b[2].cmp(&a[2]));
            while !bus.is_empty() && bus[bus.len() - 1][2] <= trips[i][1] {
                nums -= bus[bus.len() - 1][0];
                bus.pop();
            }
            nums += trips[i][0];
            if nums > capacity {
                return false;
            }
            bus.push(&trips[i]);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::common;
    use crate::p1094::Solution;

    #[test]
    fn test0() {
        let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let ans = Solution::car_pooling(trips, 4);
        assert_eq!(ans, false);
    }
    #[test]
    fn test2() {
        let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let ans = Solution::car_pooling(trips, 5);
        assert_eq!(ans, true);
    }

    #[test]
    fn test3() {
        let trips = vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]];
        let ans = Solution::car_pooling(trips, 11);
        assert_eq!(ans, true);
    }
}
