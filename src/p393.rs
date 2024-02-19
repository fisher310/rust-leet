struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        for i in data.iter() {
            print!("{:#b},", i);
        }
        println!();


        let mut i = 0;

        while i < data.len() {
            let mut j = 0;
            let mut m = data[i];
            while m & 0b10000000 != 0 {
                j += 1;
                m <<= 1;
            }
            if j == 1 || j > 4 {
                return false;
            }
            for _k in 0..j - 1 {
                i += 1;
                if i >= data.len() {
                    return false;
                }
                let n = data[i];
                if n >> 6 != 0b10 {
                    return false;
                }
            }
            i += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_0() {
        let res = Solution::valid_utf8(vec![197, 130, 1]);
        assert!(res);
    }

    #[test]
    fn test_1() {
        let res = Solution::valid_utf8(vec![235, 140, 4]);
        assert!(!res);
    }

    #[test]
    fn test_3() {
        let res = Solution::valid_utf8(vec![255]);
        assert!(!res);
    }

    #[test]
    fn test_4() {
        let res = Solution::valid_utf8(vec![145]);
        assert!(!res);
    }

    #[test]
    fn test_5() {
        let res = Solution::valid_utf8(vec![250, 145, 145, 145, 145]);
        assert!(!res);
    }
}
