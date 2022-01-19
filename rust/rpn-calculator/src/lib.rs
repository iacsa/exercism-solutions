#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for input in inputs {
        // handle the simple case first: values
        if let CalculatorInput::Value(v) = input {
            stack.push(*v);
            continue;
        }

        // common handling for all operations
        let operand1 = stack.pop()?;
        let operand2 = stack.pop()?;

        // handle specific operations
        stack.push(match input {
            CalculatorInput::Add => operand2 + operand1,
            CalculatorInput::Subtract => operand2 - operand1,
            CalculatorInput::Multiply => operand2 * operand1,
            CalculatorInput::Divide => operand2 / operand1,
            _ => unreachable!(), // covered by the earlier 'if let ...'
        });
    }

    // only return a value if there is nothing else left on the stack
    stack.pop().filter(|_| stack.is_empty())
}
