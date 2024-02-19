use std::{collections::BinaryHeap, vec};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let m = minefield.len() as i32;
    let n = minefield[0].chars().collect::<Vec<char>>().len() as i32;
    let x: Vec<i32> = vec![-1, 0, 1, 1, 1, 0, -1, -1];
    let y: Vec<i32> = vec![-1, -1, -1, 0, 1, 1, 1, 0];
    let mut ans = Vec::new();

    for i in 0..m {
        let mut line = String::new();
        for j in 0..n {
            if minefield[i as usize].chars().nth(j as usize).unwrap() == '*' {
                line.push('*');
                continue;
            }
            let mut num = 0;
            for k in 0..8 {
                let a = i as i32 + x[k];
                let b = j as i32 + y[k];
                if a >= 0
                    && a < m
                    && b >= 0
                    && b < n
                    && minefield[a as usize].chars().nth(b as usize).unwrap() == '*'
                {
                    num += 1;
                }
            }
            if num == 0 {
                line.push('.');
            } else {
                line.push_str(&num.to_string());
            }
        }
        ans.push(line);
    }

    ans
}

/// https://exercism.org/tracks/rust/exercises/luhn/edit
pub fn is_valid(code: &str) -> bool {
    let mut chs = code.chars().filter(|c| *c != ' ').collect::<Vec<char>>();
    if chs.is_empty() || chs.len() < 2 {
        return false;
    }
    chs.reverse();
    let len = chs.len();
    let mut idx = 1;

    while idx < len {
        let ch = chs[idx];
        if ch < '0' || ch > '9' {
            return false;
        }
        let value = ch as u32 - '0' as u32;
        let value = value * 2;
        let value = if value > 9 { value - 9 } else { value };
        chs[idx] = char::from_digit(value, 10).unwrap();
        idx += 2;
    }

    println!("{:?}", chs);
    let mut sum = 0;
    for ch in chs {
        if ch < '0' || ch > '9' {
            return false;
        }
        sum += ch as u32 - '0' as u32;
    }

    sum % 10 == 0
}

///
/// has 3 as a factor, add 'Pling' to the result.
/// has 5 as a factor, add 'Plang' to the result.
/// has 7 as a factor, add 'Plong' to the result.
pub fn raindrops(n: u32) -> String {
    let mut ans = String::new();
    if n % 3 == 0 {
        ans.push_str("Pling");
    }
    if n % 5 == 0 {
        ans.push_str("Plang");
    }
    if n % 7 == 0 {
        ans.push_str("Plong");
    }

    if ans.len() == 0 {
        n.to_string()
    } else {
        ans
    }
}

pub fn reply(message: &str) -> &str {
    let question = message.trim().ends_with("?");
    let chars = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();
    let captial = chars.len() > 0 && chars.iter().all(|c| c.is_uppercase());
    let blank = message.is_empty() || message.chars().all(|c| c.is_whitespace());
    if question && captial {
        "Calm down, I know what I'm doing!"
    } else if question {
        "Sure."
    } else if blank {
        "Fine. Be that way!"
    } else if captial {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let open = vec!['(', '[', '{'];
    let close = vec![')', ']', '}'];
    for c in string.chars() {
        for (_, left) in open.iter().enumerate() {
            if c == *left {
                stack.push(c);
                continue;
            }
        }
        for (idx, right) in close.iter().enumerate() {
            if c == *right {
                match stack.pop() {
                    Some(v) => {
                        if v != open[idx] {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
    }

    stack.is_empty()
}

/// https://exercism.org/tracks/rust/exercises/high-scores/edit
#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut v_scores = Vec::new();
        for s in scores {
            v_scores.push(*s);
        }
        HighScores { scores: v_scores }
    }

    pub fn scores(&self) -> &[u32] {
        // unimplemented!("Return all the scores as a slice")
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let last = self.scores.last();
        match last {
            Some(v) => Some(*v),
            None => None,
        }
        // unimplemented!("Return the latest (last) score")
    }

    pub fn personal_best(&self) -> Option<u32> {
        // unimplemented!("Return the highest score")
        let max = self.scores.iter().max();
        match max {
            Some(v) => Some(*v),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // unimplemented!("Return 3 highest scores")
        let mut heap = BinaryHeap::from_iter(self.scores.iter());
        let mut ans = Vec::with_capacity(3);
        for _ in 0..3 {
            let v = heap.pop();
            match v {
                Some(i) => ans.push(*i),
                None => {}
            }
        }
        ans
    }
}

pub fn abbreviate(phrase: &str) -> String {
    let mut ans = String::new();
    let phrase: String = phrase
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace() || *c == '-')
        .collect();

    let worlds = phrase
        .split(" ")
        .map(|s| s.trim())
        .flat_map(|s| s.split('-'))
        .collect::<Vec<&str>>();
    for word in worlds {
        if word.is_empty() {
            continue;
        }
        if word.chars().all(|c| c.is_uppercase()) {
            let c = word.chars().nth(0);
            match c {
                Some(v) => {
                    ans.push(v);
                }
                None => {}
            }
        } else {
            ans.push(word.chars().nth(0).unwrap().to_ascii_uppercase());
            let sw = word.chars().skip(1).collect::<Vec<char>>();
            if !sw.is_empty() {
                for c in sw {
                    if c.is_uppercase() {
                        ans.push(c);
                    }
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_0() {
        let ans = annotate(&[&"·*·*·", &"··*··", &"··*··", &"·····"]);
        for x in ans {
            println!("{}", x);
        }
    }

    #[test]
    fn test_1() {
        let ch = '9';
        let ans = ch as u32 - '0' as u32;
        println!("{}", ans);
    }

    #[test]
    fn test_2() {
        let ans = is_valid("4539 3195 0343 6467");
        assert_eq!(true, ans);
    }

    #[test]
    fn test_3() {
        let ans = is_valid("8273 1232 7352 0569");
        assert_eq!(false, ans);
    }

    #[test]
    fn test_4() {
        let ans = is_valid("91%95");
        assert_eq!(false, ans);
    }

    #[test]
    fn test_5() {
        let ans = raindrops(28);
        assert_eq!("Plong", ans);
    }

    #[test]
    fn test_6() {
        let ans = raindrops(30);
        assert_eq!("PlingPlang", ans);
    }

    #[test]
    fn test_7() {
        let ans = reply("WHAT'S GOING ON?");
        assert_eq!("Calm down, I know what I'm doing!", ans);
    }

    #[test]
    fn test_8() {
        assert_eq!(reply("WATCH OUT!"), "Whoa, chill out!");
    }

    #[test]
    fn test_9() {
        assert_eq!(reply("										"), "Fine. Be that way!");
    }

    #[test]
    fn test_10() {
        assert_eq!(reply("1, 2, 3"), "Whatever.");
    }

    #[test]
    fn test_11() {
        assert_eq!(
            abbreviate("Complementary metal-oxide semiconductor"),
            "CMOS"
        );
        assert_eq!(abbreviate("Ruby on Rails"), "ROR");
        assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
        assert_eq!(abbreviate(""), "");
        assert_eq!(abbreviate("Portable Network Graphics"), "PNG");
        assert_eq!(abbreviate("First In, First Out"), "FIFO");
        assert_eq!(abbreviate("GNU Image Manipulation Program"), "GIMP");
        assert_eq!(abbreviate("PHP: Hypertext Preprocessor"), "PHP");
    }
}
