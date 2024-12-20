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
    Unary(UnaryOperator, Val, Val),
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
            let val = emit_tacky(expression, instructions);
            instructions.push(Instruction::Return(val));
        }
    }

    tacky_program
}

fn emit_tacky(expression: parser::Expression, instructions: &mut Vec<Instruction>) -> Val {
    match expression {
        parser::Expression::Constant(value) => Val::Constant(value),
        parser::Expression::Unary(operator, expression) => {
            let src = emit_tacky(*expression, instructions);
            let dst = Val::Var(make_temp_name());
            let operator = match operator {
                parser::UnaryOperator::Negate => UnaryOperator::Negate,
                parser::UnaryOperator::Complement => UnaryOperator::Complement,
            };
            instructions.push(Instruction::Unary(operator, src, dst.clone()));
            dst
        }
    }
}

static mut TEMP_COUNTER: i64 = -1;

fn make_temp_name() -> String {
    format!("temp.{}", unsafe { TEMP_COUNTER += 1; TEMP_COUNTER })
}