#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        let res = match input {
            CalculatorInput::Value(elem) => *elem,
            _ => {
                match (stack.pop(), stack.pop()) {
                    (Some(right), Some(left)) => {
                       match input {
                           CalculatorInput::Add => left+right,
                           CalculatorInput::Subtract => left-right,
                           CalculatorInput::Multiply => left*right,
                           CalculatorInput::Divide => left/right,
                           CalculatorInput::Value(_) => panic!("asd")
                       }
                    },
                    _ => return None,
                }
            }
        };
        stack.push(res)
    };

    if stack.len() == 1 { stack.pop() } else { None }
}
