use crate::parser;

#[derive(Debug)]
pub struct Program {
    pub function: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub identifier: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Return(Val),
    Unary(UnaryOperator, Val, Val),        // op, src, dst
    Binary(BinaryOperator, Val, Val, Val), // op, src1, src2, dst
    Copy(Val, Val),                        // src, dst
    Jump(String),                          // identifier
    JumpIfZero(Val, String),               // condition, target
    JumpIfNotZero(Val, String),            // condition, target
    Label(String),                         // identifier
}

#[derive(Debug, Clone)]
pub enum Val {
    Constant(i64),
    Var(String),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Complement,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
    EqualTo,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

pub fn generate_tacky(program: parser::Program) -> Program {
    let mut tacky_program = Program {
        function: FunctionDefinition {
            identifier: program.function.identifier,
            instructions: Vec::new(),
        },
    };

    let instructions = &mut tacky_program.function.instructions;
    match program.function.statement {
        parser::Statement::Return(expression) => {
            let val = emit_tacky_value(expression, instructions);
            instructions.push(Instruction::Return(val));
        }
    }

    tacky_program
}

fn emit_tacky_value(expression: parser::Expression, instructions: &mut Vec<Instruction>) -> Val {
    match expression {
        parser::Expression::Constant(value) => Val::Constant(value),
        parser::Expression::Unary(operator, expression) => {
            let src = emit_tacky_value(*expression, instructions);
            let dst = Val::Var(make_temp_name());
            let operator = match operator {
                parser::UnaryOperator::Negate => UnaryOperator::Negate,
                parser::UnaryOperator::Complement => UnaryOperator::Complement,
                parser::UnaryOperator::Not => UnaryOperator::Not,
            };
            instructions.push(Instruction::Unary(operator, src, dst.clone()));
            dst
        }
        parser::Expression::Binary(parser::BinaryOperator::LAnd, operand1, operand2) => {
            let result = Val::Var(make_temp_name());
            let false_label = make_label_name("false");
            let end_label = make_label_name("and_end");

            let evaluation1 = emit_tacky_value(*operand1, instructions);
            instructions.push(Instruction::JumpIfZero(evaluation1, false_label.clone()));
            let evaluation2 = emit_tacky_value(*operand2, instructions);
            instructions.push(Instruction::JumpIfZero(evaluation2, false_label.clone()));

            instructions.push(Instruction::Copy(Val::Constant(1), result.clone()));
            instructions.push(Instruction::Jump(end_label.clone()));
            instructions.push(Instruction::Label(false_label));
            instructions.push(Instruction::Copy(Val::Constant(0), result.clone()));
            instructions.push(Instruction::Label(end_label));

            result
        }
        parser::Expression::Binary(parser::BinaryOperator::LOr, operand1, operand2) => {
            let result = Val::Var(make_temp_name());
            let true_label = make_label_name("true");
            let end_label = make_label_name("or_end");

            let evaluation1 = emit_tacky_value(*operand1, instructions);
            instructions.push(Instruction::JumpIfNotZero(evaluation1, true_label.clone()));
            let evaluation2 = emit_tacky_value(*operand2, instructions);
            instructions.push(Instruction::JumpIfNotZero(evaluation2, true_label.clone()));

            instructions.push(Instruction::Copy(Val::Constant(0), result.clone()));
            instructions.push(Instruction::Jump(end_label.clone()));
            instructions.push(Instruction::Label(true_label));
            instructions.push(Instruction::Copy(Val::Constant(1), result.clone()));
            instructions.push(Instruction::Label(end_label));

            result
        }
        parser::Expression::Binary(operator, operand1, operand2) => {
            let src1 = emit_tacky_value(*operand1, instructions);
            let src2 = emit_tacky_value(*operand2, instructions);
            let dst = Val::Var(make_temp_name());
            let operator = match operator {
                parser::BinaryOperator::Add => BinaryOperator::Add,
                parser::BinaryOperator::Subtract => BinaryOperator::Subtract,
                parser::BinaryOperator::Multiply => BinaryOperator::Multiply,
                parser::BinaryOperator::Divide => BinaryOperator::Divide,
                parser::BinaryOperator::Modulo => BinaryOperator::Remainder,
                parser::BinaryOperator::Xor => BinaryOperator::Xor,
                parser::BinaryOperator::And => BinaryOperator::And,
                parser::BinaryOperator::Or => BinaryOperator::Or,
                parser::BinaryOperator::LeftShift => BinaryOperator::LeftShift,
                parser::BinaryOperator::RightShift => BinaryOperator::RightShift,
                parser::BinaryOperator::EqualTo => BinaryOperator::EqualTo,
                parser::BinaryOperator::NotEqualTo => BinaryOperator::NotEqual,
                parser::BinaryOperator::LessThan => BinaryOperator::LessThan,
                parser::BinaryOperator::LessOrEqual => BinaryOperator::LessOrEqual,
                parser::BinaryOperator::GreaterThan => BinaryOperator::GreaterThan,
                parser::BinaryOperator::GreaterOrEqual => BinaryOperator::GreaterOrEqual,
                _ => panic!("Shouldn't reach here")
            };
            instructions.push(Instruction::Binary(operator, src1, src2, dst.clone()));
            dst
        }
    }
}

static mut TEMP_COUNTER: i64 = -1;

fn make_temp_name() -> String {
    format!("temp.{}", unsafe {
        TEMP_COUNTER += 1;
        TEMP_COUNTER
    })
}

static mut LABEL_COUNTER: i64 = -1;

fn make_label_name(prefix: &str) -> String {
    format!("label_{}.{}", prefix, unsafe {
        LABEL_COUNTER += 1;
        LABEL_COUNTER
    })
}
