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
    Binary(BinaryOperator, Operand, Operand),
    Idiv(Operand),
    Cdq,
    AllocateStack(i64),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mult,
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
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
    DX,
    R10,
    R11,
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
            tacker::Instruction::Binary(
                op @ (tacker::BinaryOperator::Add
                | tacker::BinaryOperator::Subtract
                | tacker::BinaryOperator::Multiply
                | tacker::BinaryOperator::And
                | tacker::BinaryOperator::Or
                | tacker::BinaryOperator::Xor
                | tacker::BinaryOperator::LeftShift
                | tacker::BinaryOperator::RightShift),
                src1,
                src2,
                dst,
            ) => {
                instructions.push(Instruction::Mov(
                    val_to_operand(src1),
                    val_to_operand(dst.clone()),
                ));
                instructions.push(Instruction::Binary(
                    match op {
                        tacker::BinaryOperator::Add => BinaryOperator::Add,
                        tacker::BinaryOperator::Subtract => BinaryOperator::Sub,
                        tacker::BinaryOperator::Multiply => BinaryOperator::Mult,
                        tacker::BinaryOperator::And => BinaryOperator::And,
                        tacker::BinaryOperator::Or => BinaryOperator::Or,
                        tacker::BinaryOperator::Xor => BinaryOperator::Xor,
                        tacker::BinaryOperator::LeftShift => BinaryOperator::LeftShift,
                        tacker::BinaryOperator::RightShift => BinaryOperator::RightShift,
                        _ => panic!("Checked only for these"),
                    },
                    val_to_operand(src2),
                    val_to_operand(dst),
                ));
            }
            tacker::Instruction::Binary(tacker::BinaryOperator::Divide, src1, src2, dst) => {
                instructions.push(Instruction::Mov(
                    val_to_operand(src1),
                    Operand::Reg(Reg::AX),
                ));
                instructions.push(Instruction::Cdq);
                instructions.push(Instruction::Idiv(val_to_operand(src2)));
                instructions.push(Instruction::Mov(Operand::Reg(Reg::AX), val_to_operand(dst)));
            }
            tacker::Instruction::Binary(tacker::BinaryOperator::Remainder, src1, src2, dst) => {
                instructions.push(Instruction::Mov(
                    val_to_operand(src1),
                    Operand::Reg(Reg::AX),
                ));
                instructions.push(Instruction::Cdq);
                instructions.push(Instruction::Idiv(val_to_operand(src2)));
                instructions.push(Instruction::Mov(Operand::Reg(Reg::DX), val_to_operand(dst)));
            }
        }
    }

    instructions
}

fn get_identifier_offset(
    identifiers: &mut HashMap<String, i64>,
    stack_size: &mut i64,
    name: &str,
) -> i64 {
    if !identifiers.contains_key(name) {
        *stack_size += 4;
        identifiers.insert(name.to_string(), -*stack_size);
    }
    identifiers[name]
}

fn replace_psuedo_operand_if_needed(
    operand: Operand,
    identifiers: &mut HashMap<String, i64>,
    stack_size: &mut i64,
) -> Operand {
    match operand {
        Operand::Pseudo(name) => {
            Operand::Stack(get_identifier_offset(identifiers, stack_size, &name))
        }
        _ => operand,
    }
}

fn replace_pseudo_operands(instructions: &mut Vec<Instruction>) -> i64 {
    let mut identifiers: HashMap<String, i64> = HashMap::new();
    let mut stack_size = 0;

    for instruction in instructions.iter_mut() {
        match instruction {
            Instruction::Mov(src, dst) => {
                *instruction = Instruction::Mov(
                    replace_psuedo_operand_if_needed(
                        src.clone(),
                        &mut identifiers,
                        &mut stack_size,
                    ),
                    replace_psuedo_operand_if_needed(
                        dst.clone(),
                        &mut identifiers,
                        &mut stack_size,
                    ),
                );
            }
            Instruction::Unary(op, Operand::Pseudo(name)) => {
                *instruction = Instruction::Unary(
                    op.clone(),
                    Operand::Stack(get_identifier_offset(
                        &mut identifiers,
                        &mut stack_size,
                        name,
                    )),
                );
            }
            Instruction::Binary(op, src, dst) => {
                *instruction = Instruction::Binary(
                    op.clone(),
                    replace_psuedo_operand_if_needed(
                        src.clone(),
                        &mut identifiers,
                        &mut stack_size,
                    ),
                    replace_psuedo_operand_if_needed(
                        dst.clone(),
                        &mut identifiers,
                        &mut stack_size,
                    ),
                );
            }
            Instruction::Idiv(Operand::Pseudo(name)) => {
                *instruction = Instruction::Idiv(Operand::Stack(get_identifier_offset(
                    &mut identifiers,
                    &mut stack_size,
                    name,
                )));
            }
            _ => {}
        }
    }

    stack_size
}

fn fix_up(orig_instructions: Vec<Instruction>, stack_size: i64) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    instructions.push(Instruction::AllocateStack(stack_size));

    for instruction in orig_instructions {
        match instruction {
            // Can't move from memory address to memory address
            Instruction::Mov(Operand::Stack(src), Operand::Stack(dst)) => {
                instructions.push(Instruction::Mov(
                    Operand::Stack(src),
                    Operand::Reg(Reg::R10),
                ));
                instructions.push(Instruction::Mov(
                    Operand::Reg(Reg::R10),
                    Operand::Stack(dst),
                ));
            }
            // Can't divide by an immediate value
            Instruction::Idiv(Operand::Imm(value)) => {
                instructions.push(Instruction::Mov(
                    Operand::Imm(value),
                    Operand::Reg(Reg::R10),
                ));
                instructions.push(Instruction::Idiv(Operand::Reg(Reg::R10)));
            }
            // Can't use memory addresses as both the src and destination
            Instruction::Binary(
                op @ (BinaryOperator::Add | BinaryOperator::Sub),
                Operand::Stack(src),
                Operand::Stack(dst),
            ) => {
                instructions.push(Instruction::Mov(
                    Operand::Stack(src),
                    Operand::Reg(Reg::R10),
                ));
                instructions.push(Instruction::Binary(
                    op,
                    Operand::Reg(Reg::R10),
                    Operand::Stack(dst),
                ));
            }
            // Can't use a memory address as its destination
            Instruction::Binary(BinaryOperator::Mult, src, Operand::Stack(dst)) => {
                instructions.push(Instruction::Mov(
                    Operand::Stack(dst),
                    Operand::Reg(Reg::R11),
                ));
                instructions.push(Instruction::Binary(
                    BinaryOperator::Mult,
                    src,
                    Operand::Reg(Reg::R11),
                ));
                instructions.push(Instruction::Mov(
                    Operand::Reg(Reg::R11),
                    Operand::Stack(dst),
                ));
            }
            _ => instructions.push(instruction),
        }
    }

    instructions
}

pub fn assemble(program: tacker::Program) -> Program {
    let mut instructions = tacky_to_assembly(program.function.instructions);
    let stack_size = replace_pseudo_operands(&mut instructions);
    let instructions = fix_up(instructions, stack_size);

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
