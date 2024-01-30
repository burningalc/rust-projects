pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut values = vec![];

        // decide if token is number or arithmetic operators
        for token in tokens.iter() {
            match token.as_str() {
                "+" => {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    values.push(v1 + v2);
                }
                "-" => {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    values.push(v2 - v1);
                }
                "*" => {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    values.push(v1 * v2);
                }
                "/" => {
                    let v1 = values.pop().unwrap();
                    let v2 = values.pop().unwrap();
                    values.push(v2 / v1);
                }
                _ => values.push(token.parse::<i32>().unwrap()),
            }
        }

        values.pop().unwrap()
    }
}
