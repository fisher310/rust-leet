use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        helper(&chars)
    }
}

fn helper(expr: &[char]) -> i32 {
    let mut par_stack: Vec<usize> = Vec::new();
    let mut par_map: HashMap<usize, usize> = HashMap::new();
    for (idx, c) in expr.iter().enumerate() {
        if *c == '(' {
            par_stack.push(idx);
        } else if *c == ')' {
            par_map.insert(par_stack.pop().unwrap(), idx);
        }
    }
    let mut value = 0_i32;
    let mut digit = false;
    let mut num_stack: VecDeque<i32> = VecDeque::new();
    let mut op_stack: VecDeque<char> = VecDeque::new();
    let mut minus = 1;
    let mut i = 0;
    while i < expr.len() {
        let c = expr[i];
        if c.is_digit(10) {
            value = value * 10 + c.to_digit(10).unwrap() as i32;
            digit = true;
        } else {
            if digit {
                num_stack.push_back(minus * value);
                value = 0;
                digit = false;
                minus = 1;
            }

            if c == '+' {
                op_stack.push_back('+');
            } else if c == '-' {
                minus = -1;
                op_stack.push_back('+');
                if num_stack.is_empty() {
                    num_stack.push_back(0);
                }
            } else if c == '(' {
                let end = *(par_map.get(&i).unwrap());
                num_stack.push_back(minus * helper(&expr[i + 1..end]));
                i = end;
                minus = 1;
            }
        }
        i += 1;
    }

    if digit {
        num_stack.push_back(minus * value);
    }

    while !op_stack.is_empty() {
        match op_stack.pop_front() {
            Some(_) => {
                let n1 = num_stack.pop_front().unwrap();
                let n2 = num_stack.pop_front().unwrap();
                num_stack.push_front(n1 + n2);
            }
            None => {}
        }
    }
    num_stack.pop_front().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let ans = Solution::calculate("1 + 1".to_string());
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_1() {
        let ans = Solution::calculate("2-1 + 2 ".to_string());
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_2() {
        let ans = Solution::calculate("(1+(4+5+2 )-3)+(6+8)".to_string());
        assert_eq!(ans, 23);
    }

    #[test]
    fn test_3() {
        let ans = Solution::calculate("1-(     -2)".to_string());
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_4() {
        let ans = Solution::calculate("-(1 -2)".to_string());
        assert_eq!(ans, 1);
    }
}
