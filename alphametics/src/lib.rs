use std::collections::{BTreeMap, HashMap};

// struct Puzzle {
//     solution: HashMap<char, u8>, // k: char, v: (digit, leading)
//     used_digit: Vec<u8>,
//     leading_char: Vec<char>,
//     lhs_coefficient_char: HashMap<char, u8>,
//     rhs_coefficient_char: String,
// }

// impl Puzzle {
//     fn new(input: &str) -> Puzzle {
//         let (lhs, rhs) = parse_equation(input);
//         Puzzle {
//             solution: HashMap::new(),
//             lhs,
//             rhs,
//         }
//     }
// }

// 26^10 = 141,167,095,653,376 = 141.1 trillion combinations
fn brute_force_solve(input: &str) -> Option<HashMap<char, u8>> {
    let (equation, result) = parse_equation(input);
    let mut solution = HashMap::new();

    // insert all characters found in the equation and result into the solution
    for term in equation.iter().chain(std::iter::once(&result)) {
        for character in term.chars() {
            solution.entry(character).or_insert(0);
        }
    }

    // check all possible solutions
    let mut iterationCount = 0;
    loop {
        // check if the current solution is valid
        if verify_solution(&equation, &result, &solution) {
            return Some(solution);
        }

        if solution.values().all(|&digit| digit == 9) {
            return None;
        }

        solution = next_solution(&solution);
        iterationCount += 1;

        if iterationCount % 10000 == 0 {
            println!("Iteration: {}", iterationCount);
        }
    }
}

pub fn verify_solution(equation: &Vec<String>, result: &str, solution: &HashMap<char, u8>) -> bool {
    // let transformed_equation = equation
    //     .iter()
    //     .map(|term| transform_to_numbers(term, solution).unwrap())
    //     .collect::<Vec<u32>>();
    // let transformed_result = transform_to_numbers(result, solution).unwrap();

    let mut transformed_equation: Vec<u32> = Vec::new();
    for term in equation {
        let transformed_term = transform_to_numbers(term, solution);
        match transformed_term {
            Some(number) => transformed_equation.push(number),
            None => return false,
        }
    }
    let transformed_result = match transform_to_numbers(result, solution) {
        Some(number) => number,
        None => return false,
    };

    let sum: u32 = transformed_equation.iter().sum();

    // check if all the number in solution is unique
    let mut unique_digits = BTreeMap::new();
    for digit in solution.values() {
        unique_digits.insert(digit, 0);
    }

    unique_digits.len() == solution.len() && sum == transformed_result
}

pub fn parse_equation(input: &str) -> (Vec<String>, String) {
    let input_stripped = input.replace([' ', '\n'], "");
    let mut input_splitted = input_stripped.split("==");
    let equation = input_splitted
        .next()
        .unwrap()
        .split('+')
        .map(|s| s.to_string())
        .collect();
    let result = input_splitted.next().unwrap();

    (equation, result.to_string())
}

pub fn transform_to_numbers(term: &str, solution: &HashMap<char, u8>) -> Option<u32> {
    let mut transformed_number = "".to_string();
    // add each digit to the transformed number
    for element in term.chars() {
        let digit = solution.get(&element).unwrap();
        transformed_number += &digit.to_string();
    }

    // check if the number has leading zero
    match transformed_number.len() > 1 && transformed_number.starts_with('0') {
        true => None,
        false => Some(transformed_number.parse::<u32>().unwrap()),
    }
}

pub fn next_solution(solution: &HashMap<char, u8>) -> HashMap<char, u8> {
    let mut sorted_solution = solution.iter().collect::<Vec<(&char, &u8)>>();

    sorted_solution.sort_by_key(|pair| pair.0);

    let mut new_solution = solution.clone();

    for (char, digit) in sorted_solution.iter().rev() {
        // if digit is smaller than 9, increment it
        if **digit < 9 {
            new_solution.insert(**char, **digit + 1);
            return new_solution;
        }
        new_solution.insert(**char, 0);
    }

    new_solution
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    brute_force_solve(input)
}
