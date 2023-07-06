#[derive(Debug)]
pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut pt = PascalsTriangle { rows: vec![] };
        for i in 0..row_count {
            if i == 0 {
                pt.rows.push(vec![1]);
            } else if i == 1 {
                pt.rows.push(vec![1, 1]);
            } else {
                let pre = &pt.rows[(i - 1) as usize];
                let mut row = vec![1];
                for j in 1..pre.len() {
                    row.push(pre[j] + pre[j - 1]);
                }
                row.push(1);
                pt.rows.push(row);
            }
        }

        pt
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::pascals_triangle::*;

    #[test]
    fn test_01() {
        let pt = PascalsTriangle::new(5);
        let mut ans = pt.rows();
        ans[0].push(1111);
        println!("{:?}", pt);
        println!("{:?}", ans)
    }
    #[test]
    fn no_rows() {
        let pt = PascalsTriangle::new(0);
        let expected: Vec<Vec<u32>> = Vec::new();
        assert_eq!(expected, pt.rows());
    }
    #[test]
    fn one_row() {
        let pt = PascalsTriangle::new(1);
        let expected: Vec<Vec<u32>> = vec![vec![1]];
        assert_eq!(expected, pt.rows());
    }
    #[test]
    fn two_rows() {
        let pt = PascalsTriangle::new(2);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
        assert_eq!(expected, pt.rows());
    }
    #[test]
    fn three_rows() {
        let pt = PascalsTriangle::new(3);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
        assert_eq!(expected, pt.rows());
    }
    #[test]
    fn last_of_four_rows() {
        let pt = PascalsTriangle::new(4);
        let expected: Vec<u32> = vec![1, 3, 3, 1];
        assert_eq!(Some(expected), pt.rows().pop());
    }
    #[test]
    fn five_rows() {
        let pt = PascalsTriangle::new(5);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(expected, pt.rows());
    }
    #[test]
    fn six_rows() {
        let pt = PascalsTriangle::new(6);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
        ];
        assert_eq!(expected, pt.rows());
    }
    #[test]
    fn seven_rows() {
        let pt = PascalsTriangle::new(7);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
        ];
        assert_eq!(expected, pt.rows());
    }
    #[test]
    fn ten_rows() {
        let pt = PascalsTriangle::new(10);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
            vec![1, 7, 21, 35, 35, 21, 7, 1],
            vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
        ];
        assert_eq!(expected, pt.rows());
    }
}
