struct Solution;

impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let n = arr.len();
        for i in (0..n).step_by(1).rev() {
            let mut max_index = i;
            for j in 0..=i {
                if arr[j] > arr[max_index] {
                    max_index = j;
                }
            }

            if max_index == i {
                continue;
            }

            arr[0..=max_index].reverse();
            arr[0..=i].reverse();
            res.push(max_index as i32 + 1);
            res.push(i as i32 + 1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let ans = Solution::pancake_sort(vec![3, 2, 4, 1]);
        println!("{:?}", ans);
    }

    #[test]
    fn test02() {
        let ans = Solution::pancake_sort(vec![1, 2, 3]);
        println!("{:?}", ans);
    }
}
