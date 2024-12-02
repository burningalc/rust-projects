use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// 26P10 = 19,275,223,968,000 = 19.3 trillion permutations at max
fn brute_force_solve(input: &str) -> Option<HashMap<char, u8>> {
    let (coefficient_char, leading_char, characters) = parse_equation(input);

    // check all possible solution
    let mut iteration_count = 0;
    for perm in (0..=9).permutations(characters.len()) {
        iteration_count += 1;
        if iteration_count % 100000 == 0 {
            println!("Iteration: {}", iteration_count);
        }
        if verify_solution(&coefficient_char, &leading_char, &characters, &perm) {
            // build the hashmap to return
            let mut solution = HashMap::new();
            for (index, key) in characters.iter().enumerate() {
                solution.insert(*key, perm[index]);
            }
            return Some(solution);
        }
    }

    None
}

pub fn verify_solution(
    coefficients: &Vec<i64>,
    leading_char: &HashSet<char>,
    characters: &Vec<char>,
    solution: &Vec<u8>,
) -> bool {
    // check none of the leading char is 0
    for (index, character) in characters.iter().enumerate() {
        if leading_char.contains(character) && *solution.get(index).unwrap() == 0 {
            return false;
        }
    }

    // get the sum for coefficient_char
    let mut sum: i64 = 0;
    for (index, s) in solution.iter().enumerate() {
        sum += coefficients[index] * *s as i64;
    }

    sum == 0
}

/// parse the equation into data structure
///
/// returns (coefficient_char, leading_char, solution)
pub fn parse_equation(input: &str) -> (Vec<i64>, HashSet<char>, Vec<char>) {
    let input_stripped = input.replace([' ', '\n'], "");
    let mut input_splitted = input_stripped.split("==");
    let lhs = input_splitted
        .next()
        .unwrap()
        .split('+')
        .map(|s| s.to_string())
        .collect();

    let rhs = input_splitted.next().unwrap();

    let coefficient_char = calculate_coefficient(&lhs, rhs.to_string());

    let mut concat_terms = lhs.clone();
    concat_terms.push(rhs.to_string());

    let leading_char = concat_terms
        .clone()
        .into_iter()
        .map(|term| term.chars().next().unwrap())
        .collect::<HashSet<_>>();

    let solution = coefficient_char.keys().cloned().collect();
    let coefficient = coefficient_char.values().cloned().collect();

    (coefficient, leading_char, solution)
}

pub fn calculate_coefficient(lhs: &Vec<String>, rhs: String) -> HashMap<char, i64> {
    let mut coefficient = HashMap::new();

    for term in lhs {
        let mut coefficient_multiplier = 1;
        for character in term.chars().rev() {
            coefficient
                .entry(character)
                .and_modify(|e| *e += coefficient_multiplier)
                .or_insert(coefficient_multiplier);
            coefficient_multiplier *= 10;
        }
    }

    let mut coefficient_multiplier = 1;
    for char in rhs.chars().rev() {
        // insert or modify
        coefficient
            .entry(char)
            .and_modify(|e| *e -= coefficient_multiplier)
            .or_insert(-coefficient_multiplier);
        coefficient_multiplier *= 10;
    }

    coefficient
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    brute_force_solve(input)
}
