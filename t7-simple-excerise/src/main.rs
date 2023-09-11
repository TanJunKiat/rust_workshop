enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct MathExpression {
    operation: MathOperation,
    operand1: f64,
    operand2: f64,
}

impl MathExpression {
    fn evaluate(&self) -> Result<f64,String> { // defining Ok returns f64 and Err returns string
        match self.operation {
            MathOperation::Add => {
                return Result::Ok(self.operand1 + self.operand2);
            }
            MathOperation::Subtract => {
                return Result::Ok(self.operand1 - self.operand2);
            }
            MathOperation::Multiply => {
                return Result::Ok(self.operand1 * self.operand2);
            }
            MathOperation::Divide => {
                return Result::Ok(self.operand1 / self.operand2);
            }
        }
    }
}

fn main() {
    // Generate expressions for 5 + 3
    let eqn1 = MathExpression {
        operation: MathOperation::Add,
        operand1: 5.0,
        operand2: 3.0,
    };
    match eqn1.evaluate() {
        Ok(success) => println!("Answer is {}",success),
        _=>{}
    }

    // Generate expressions for 32 - 20
    let eqn2 = MathExpression {
        operation: MathOperation::Subtract,
        operand1: 32.0,
        operand2: 20.0,
    };
    match eqn2.evaluate() {
        Ok(success) => println!("Answer is {}",success),
        _=>{}
    }

    // Generate expressions for 25 * 12
    let eqn3 = MathExpression {
        operation: MathOperation::Multiply,
        operand1: 25.0,
        operand2: 12.0,
    };
    match eqn3.evaluate() {
        Ok(success) => println!("Answer is {}",success),
        _=>{}
    }

    // Generate expressions for 120 / 15
    let eqn4 = MathExpression {
        operation: MathOperation::Divide,
        operand1: 120.0,
        operand2: 15.0,
    };
    match eqn4.evaluate() {
        Ok(success) => println!("Answer is {}",success),
        _=>{}
    }
}