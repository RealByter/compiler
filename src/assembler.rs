use crate::parser;

#[derive(Debug)]
pub struct Program {
    pub function: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i64),
    Register,
}

pub fn assemble(program: parser::Program) -> Program {
    Program {
        function: FunctionDefinition {
            name: program.function.identifier,
            instructions: match program.function.statement {
                parser::Statement::Return(expression) => vec![
                    Instruction::Mov(
                        match expression {
                            parser::Expression::Constant(value) => Operand::Imm(value),
                            _ => unimplemented!(),
                        },
                        Operand::Register,
                    ),
                    Instruction::Ret,
                ],
            },
        },
    }
}
