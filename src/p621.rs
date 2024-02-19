use std::collections::{BinaryHeap, HashMap};

struct Solution;

#[derive(Debug)]
struct Task {
    name: char,
    count: u32,
    time: i32,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.time.partial_cmp(&self.time) {
            Some(ord) => match ord {
                std::cmp::Ordering::Equal => self.count.partial_cmp(&other.count),
                _ => Some(ord),
            },
            None => None,
        }
    }
}

impl Eq for Task {}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .time
            .cmp(&self.time)
            .then_with(|| self.count.cmp(&other.count))
    }
}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut map: HashMap<char, u32> = HashMap::new();
        for task in tasks {
            if map.contains_key(&task) {
                let count = map.get(&task).unwrap();
                map.insert(task, count + 1);
            } else {
                map.insert(task, 1);
            }
        }

        let mut pq: BinaryHeap<Task> = BinaryHeap::new();

        for (name, count) in map {
            pq.push(Task {
                name,
                count,
                time: 0,
            });
        }

        let mut ans = 0_i32;
        while !pq.is_empty() {
            let task = pq.peek().unwrap();
            let tm = task.time;
            if tm <= ans {
                let mut task = pq.pop().unwrap();
                task.count -= 1;
                if task.count > 0 {
                    task.time += n + 1;
                    pq.push(task);
                }
                ans += 1;
            } else {
                ans += tm - ans;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        // ["A","A","A","B","B","B"], n = 2, ans = 8
        assert_eq!(
            8,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)
        );
    }

    #[test]
    fn test_1() {
        // ["A","A","A","B","B","B"], n = 0
        assert_eq!(
            6,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0)
        );
    }

    #[test]
    fn test_3() {
        //  ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
        assert_eq!(
            16,
            Solution::least_interval(
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                2
            )
        )
    }
}
