use std::collections::{HashMap, HashSet};

pub struct Alphametic {
    addends: Vec<Vec<char>>,
    sum: Vec<char>,
}

impl Alphametic {
    fn new(input: &str) -> Self {
        let mut expr = input.split("==");
        let addends = expr
            .next()
            .unwrap()
            .split("+")
            .map(|s| s.trim())
            .map(|s| s.chars().rev().collect())
            .collect();

        let sum = expr.next().unwrap().chars().collect();

        Alphametic { addends, sum }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let expr = input.split("==").collect::<Vec<&str>>();
    let adders: Vec<Vec<char>> = expr[0]
        .split("+")
        .map(|s| s.trim())
        .map(|s| s.chars().rev().collect())
        .collect();
    let mut dist_set = HashSet::new();
    for adder in adders.iter() {
        for ch in adder.iter() {
            dist_set.insert(*ch);
        }
    }

    let sum = expr[1].trim().chars().rev().collect::<Vec<char>>();

    for ch in sum.iter() {
        dist_set.insert(*ch);
    }
    let mut map: HashMap<char, u8> = HashMap::new();
    let mut rev_map: HashMap<u8, char> = HashMap::new();
    if helper(&dist_set, &adders, &sum, &mut map, &mut rev_map) {
        return Some(map);
    }

    None
}

fn helper(
    dist_set: &HashSet<char>,
    adders: &Vec<Vec<char>>,
    sum: &Vec<char>,
    map: &mut HashMap<char, u8>,
    rev_map: &mut HashMap<u8, char>,
) -> bool {
    for ch in dist_set {
        if !map.contains_key(ch) {
            for i in 0..9 {
                if !rev_map.contains_key(&i) {
                    map.insert(*ch, i);
                    rev_map.insert(i, *ch);
                    if helper(dist_set, adders, sum, map, rev_map) {
                        return true;
                    } else {
                        map.remove(ch);
                        rev_map.remove(&i);
                    }
                }
            }
        }
    }

    if map[&sum[sum.len() - 1]] == 0 {
        return false;
    }

    for adder in adders {
        if map[&adder[adder.len() - 1]] == 0 {
            return false;
        }
    }

    let max_len = adders.iter().map(|a| a.len()).max().unwrap();
    let mut rem = 0;
    let mut ans: Vec<char> = Vec::with_capacity(max_len + 1);
    for i in 0..max_len {
        let mut ss = 0;
        for adder in adders {
            if adder.len() > i {
                ss += map[&adder[i]] as i32;
            }
        }
        ss += rem;
        let rx = (ss % 10) as u8;
        if !rev_map.contains_key(&rx) {
            return false;
        }
        ans.push(rev_map[&rx]);
        rem = ss / 10;
    }
    while rem > 0 {
        ans.push(rev_map[&((rem % 10) as u8)]);
        rem /= 10;
    }

    if ans.len() != sum.len() {
        return false;
    }

    for i in 0..ans.len() {
        if ans[i] != sum[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphamettics as alphametics;

    use std::collections::HashMap;
    fn assert_alphametic_solution_eq(puzzle: &str, solution: &[(char, u8)]) {
        let answer = alphametics::solve(puzzle);
        let solution: HashMap<char, u8> = solution.iter().cloned().collect();
        assert_eq!(answer, Some(solution));
    }
    #[test]
    fn test_with_three_letters() {
        assert_alphametic_solution_eq("I + BB == ILL", &[('I', 1), ('B', 9), ('L', 0)]);
    }
    #[test]
    #[ignore]
    fn test_must_have_unique_value_for_each_letter() {
        let answer = alphametics::solve("A == B");
        assert_eq!(answer, None);
    }
    #[test]
    #[ignore]
    fn test_leading_zero_solution_is_invalid() {
        let answer = alphametics::solve("ACA + DD == BD");
        assert_eq!(answer, None);
    }
    #[test]
    #[ignore]
    fn test_sum_must_be_wide_enough() {
        let answer = alphametics::solve("ABC + DEF == GH");
        assert_eq!(answer, None);
    }
    #[test]
    #[ignore]
    fn puzzle_with_two_digits_final_carry() {
        assert_alphametic_solution_eq(
            "A + A + A + A + A + A + A + A + A + A + A + B == BCC",
            &[('A', 9), ('B', 1), ('C', 0)],
        );
    }
    #[test]
    #[ignore]
    fn test_puzzle_with_four_letters() {
        assert_alphametic_solution_eq("AS + A == MOM", &[('A', 9), ('S', 2), ('M', 1), ('O', 0)]);
    }
    #[test]
    #[ignore]
    fn test_puzzle_with_six_letters() {
        assert_alphametic_solution_eq(
            "NO + NO + TOO == LATE",
            &[('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)],
        );
    }
    #[test]
    #[ignore]
    fn test_puzzle_with_seven_letters() {
        assert_alphametic_solution_eq(
            "HE + SEES + THE == LIGHT",
            &[
                ('E', 4),
                ('G', 2),
                ('H', 5),
                ('I', 0),
                ('L', 1),
                ('S', 9),
                ('T', 7),
            ],
        );
    }
    #[test]
    #[ignore]
    fn test_puzzle_with_eight_letters() {
        assert_alphametic_solution_eq(
            "SEND + MORE == MONEY",
            &[
                ('S', 9),
                ('E', 5),
                ('N', 6),
                ('D', 7),
                ('M', 1),
                ('O', 0),
                ('R', 8),
                ('Y', 2),
            ],
        );
    }
    #[test]
    #[ignore]
    fn test_puzzle_with_ten_letters() {
        assert_alphametic_solution_eq(
            "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE",
            &[
                ('A', 5),
                ('D', 3),
                ('E', 4),
                ('F', 7),
                ('G', 8),
                ('N', 0),
                ('O', 2),
                ('R', 1),
                ('S', 6),
                ('T', 9),
            ],
        );
    }
    #[test]
    #[ignore]
    fn test_puzzle_with_ten_letters_and_199_addends() {
        assert_alphametic_solution_eq(
        "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES",
        &[
            ('A', 1),
            ('E', 0),
            ('F', 5),
            ('H', 8),
            ('I', 7),
            ('L', 2),
            ('O', 6),
            ('R', 3),
            ('S', 4),
            ('T', 9),
        ],
    );
    }
}
