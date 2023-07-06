use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);
impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }
    fn increment(&mut self, word: &str) {
        let key = word.to_string();

        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }
    fn display(self, n: u64) {
        for (key, value) in self.0.iter() {
            if *value > n {
                println!("{}: {}", key, value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let arguments: Vec<String> =
            vec!["/Users/lihailong/spaces/algorithm/rust-leet/src/word_counter.rs".to_string()]; // env::args().collect();
        let filename = arguments[0].clone();
        println!("Processing file: {}", filename);
        let file = File::open(filename).expect("Could not open file");
        let reader = BufReader::new(file);
        let mut word_counter = WordCounter::new();
        for line in reader.lines() {
            let line = line.expect("Could not read line");
            let words = line.split(" ");
            for word in words {
                if word == "" {
                    continue;
                } else {
                    word_counter.increment(word);
                }
            }
        }
        word_counter.display(2);
    }
}
