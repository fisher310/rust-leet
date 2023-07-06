struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();

        let pairs = vec![('(', ')'), ('[', ']'), ('{', '}')];

        let mut stack: Vec<char> = Vec::new();

        for c in chars {
            for (left, right) in pairs.iter() {
                if c == *left {
                    stack.push(c);
                } else if c == *right {
                    match stack.pop() {
                        None => return false,
                        Some(v) => {
                            if v != *left {
                                return false;
                            }
                        }
                    }
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        assert_eq!(true, Solution::is_valid("()".to_string()));
    }
}
