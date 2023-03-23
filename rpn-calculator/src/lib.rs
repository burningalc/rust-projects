#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut num_stack = Vec::new();

    for expr in inputs {
        println!("{:?} {:?}", num_stack, expr);
        match expr {
            CalculatorInput::Value(n) => num_stack.push(*n),
            CalculatorInput::Add => eval_operation(&mut num_stack, |a, b| a + b)?,
            CalculatorInput::Subtract => eval_operation(&mut num_stack, |a, b| b - a)?,
            CalculatorInput::Multiply => eval_operation(&mut num_stack, |a, b| a * b)?,
            CalculatorInput::Divide => eval_operation(&mut num_stack, |a, b| b / a)?,
        }
    }

    if num_stack.len() != 1 {
        return None;
    }
    num_stack.pop()
}

fn eval_operation<F>(stack: &mut Vec<i32>, op: F) -> Option<()>
where
    F: FnOnce(i32, i32) -> i32,
{
    let a = stack.pop()?;
    let b = stack.pop()?;
    let result = op(a, b);
    stack.push(result);
    Some(())
}
