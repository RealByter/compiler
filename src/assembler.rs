use crate::tacker;
use std::collections::HashMap;

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
    Unary(UnaryOperator, Operand),
    AllocateStack(i64),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(i64),
    Reg(Reg),
    Pseudo(String),
    Stack(i64),
}

#[derive(Debug, Clone)]
pub enum Reg {
    AX,
    R10D,
}

pub fn tacky_to_assembly(orig_instructions: Vec<tacker::Instruction>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for instruction in orig_instructions {
        match instruction {
            tacker::Instruction::Return(val) => {
                instructions.push(Instruction::Mov(val_to_operand(val), Operand::Reg(Reg::AX)));
                instructions.push(Instruction::Ret);
            }
            tacker::Instruction::Unary(operator, src, dst) => {
                instructions.push(Instruction::Mov(
                    val_to_operand(src),
                    val_to_operand(dst.clone()),
                ));
                instructions.push(Instruction::Unary(
                    match operator {
                        tacker::UnaryOperator::Negate => UnaryOperator::Neg,
                        tacker::UnaryOperator::Complement => UnaryOperator::Not,
                    },
                    val_to_operand(dst),
                ));
            }
        }
    }

    instructions
}

fn get_identifier_offset(identifiers: &mut HashMap<String, i64>, stack_size: &mut i64, name: &str) -> i64 {
    if !identifiers.contains_key(name) {
        *stack_size += 4;
        identifiers.insert(name.to_string(), -*stack_size);
    }
    identifiers[name]
}

fn replace_pseudo_operands(instructions: &mut Vec<Instruction>) -> i64 {
    let mut identifiers: HashMap<String, i64> = HashMap::new();
    let mut stack_size = 0;

    for instruction in instructions.iter_mut() {
        match instruction {
            Instruction::Mov(src, dst) => {
                let new_src = match src {
                    Operand::Pseudo(name) => {
                        Operand::Stack(get_identifier_offset(&mut identifiers, &mut stack_size, name))
                    }
                    _ => src.clone(),
                };

                let new_dst = match dst {
                    Operand::Pseudo(name) => {
                        Operand::Stack(get_identifier_offset(&mut identifiers, &mut stack_size, name))
                    }
                    _ => dst.clone(),
                };

                *instruction = Instruction::Mov(new_src, new_dst);
            }
            Instruction::Unary(op, Operand::Pseudo(name)) => {
                *instruction = Instruction::Unary(
                    op.clone(),
                    Operand::Stack(get_identifier_offset(&mut identifiers, &mut stack_size, name)),
                );
            }
            _ => {}
        }
    }

    stack_size
}

fn allocate_stack_and_fix_mov(orig_instructions: Vec<Instruction>, stack_size: i64) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    instructions.push(Instruction::AllocateStack(stack_size));

    for instruction in orig_instructions {
        match instruction {
            Instruction::Mov(Operand::Stack(src), Operand::Stack(dst)) => {
                instructions.push(Instruction::Mov(Operand::Stack(src), Operand::Reg(Reg::R10D)));
                instructions.push(Instruction::Mov(Operand::Reg(Reg::R10D), Operand::Stack(dst)));
            }
            _ => instructions.push(instruction),
        }
    }

    instructions
}

pub fn assemble(program: tacker::Program) -> Program {
    let mut instructions = tacky_to_assembly(program.function.instructions);
    let stack_size = replace_pseudo_operands(&mut instructions);
    let instructions = allocate_stack_and_fix_mov(instructions, stack_size);

    Program {
        function: FunctionDefinition {
            name: program.function.identifier,
            instructions: instructions,
        },
    }
}

fn val_to_operand(val: tacker::Val) -> Operand {
    match val {
        tacker::Val::Constant(value) => Operand::Imm(value),
        tacker::Val::Var(name) => Operand::Pseudo(name),
    }
}
