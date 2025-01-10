use crate::tacker;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionDefinition>,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
    Unary(UnaryOperator, Operand),
    Binary(BinaryOperator, Operand, Operand),
    Cmp(Operand, Operand),
    Idiv(Operand),
    Cdq,
    Jmp(String),
    JmpCC(CondCode, String),
    SetCC(CondCode, Operand),
    Label(String),
    AllocateStack(i64),
    Ret,
    DeallocateStack(i64),
    Push(Operand),
    Call(String),
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
    CX,
    DX,
    DI,
    SI,
    R8,
    R9,
    R10,
    R11,
}

#[derive(Debug, Clone)]
pub enum CondCode {
    E,
    NE,
    G,
    GE,
    L,
    LE,
}

pub fn tacky_function_to_assembly(function: tacker::FunctionDefinition) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let arg_registers = [Reg::DI, Reg::SI, Reg::DX, Reg::CX, Reg::R8, Reg::R9];
    let mut register_args = function.params;
    let stack_args = if register_args.len() >= 6 {
        register_args.split_off(6)
    } else {
        Vec::new()
    };
    for (i, param) in register_args.iter().enumerate() {
        if i < arg_registers.len() {
            instructions.push(Instruction::Mov(
                Operand::Reg(arg_registers[i].clone()),
                Operand::Pseudo(param.clone()),
            ));
        }
    }
    for (i, param) in stack_args.iter().enumerate() {
        instructions.push(Instruction::Mov(
            Operand::Stack((i as i64 + 1) * 8),
            Operand::Pseudo(param.clone()),
        ));
    }
    for instruction in function.instructions {
        match instruction {
            tacker::Instruction::Return(val) => {
                println!("return");
                instructions.push(Instruction::Mov(val_to_operand(val), Operand::Reg(Reg::AX)));
                instructions.push(Instruction::Ret);
            }
            tacker::Instruction::Jump(target) => {
                instructions.push(Instruction::Jmp(target));
            }
            tacker::Instruction::Copy(src, dst) => {
                println!("copy");
                println!("{:#?} {:#?}", src, dst);
                println!("{:#?} {:#?}", val_to_operand(src.clone()), val_to_operand(dst.clone()));
                instructions.push(Instruction::Mov(val_to_operand(src), val_to_operand(dst)));
            }
            tacker::Instruction::Label(identifier) => {
                instructions.push(Instruction::Label(identifier));
            }
            tacker::Instruction::Unary(
                operator @ (tacker::UnaryOperator::Complement | tacker::UnaryOperator::Negate),
                src,
                dst,
            ) => {
                instructions.push(Instruction::Mov(
                    val_to_operand(src),
                    val_to_operand(dst.clone()),
                ));
                instructions.push(Instruction::Unary(
                    match operator {
                        tacker::UnaryOperator::Negate => UnaryOperator::Neg,
                        tacker::UnaryOperator::Complement => UnaryOperator::Not,
                        _ => {
                            panic!("Only checked for these")
                        }
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
                println!("binary");
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
            tacker::Instruction::JumpIfZero(val, target) => {
                instructions.push(Instruction::Cmp(Operand::Imm(0), val_to_operand(val)));
                instructions.push(Instruction::JmpCC(CondCode::E, target));
            }
            tacker::Instruction::JumpIfNotZero(val, target) => {
                instructions.push(Instruction::Cmp(Operand::Imm(0), val_to_operand(val)));
                instructions.push(Instruction::JmpCC(CondCode::NE, target));
            }
            tacker::Instruction::JumpIfEqual(val1, val2, target) => {
                instructions.push(Instruction::Cmp(val_to_operand(val1), val_to_operand(val2)));
                instructions.push(Instruction::JmpCC(CondCode::E, target));
            }
            tacker::Instruction::Unary(tacker::UnaryOperator::Not, src, dst) => {
                instructions.push(Instruction::Cmp(Operand::Imm(0), val_to_operand(src)));
                instructions.push(Instruction::Mov(
                    Operand::Imm(0),
                    val_to_operand(dst.clone()),
                ));
                instructions.push(Instruction::SetCC(CondCode::E, val_to_operand(dst)));
            }
            tacker::Instruction::Binary(
                operator @ (tacker::BinaryOperator::EqualTo
                | tacker::BinaryOperator::NotEqual
                | tacker::BinaryOperator::LessThan
                | tacker::BinaryOperator::LessOrEqual
                | tacker::BinaryOperator::GreaterThan
                | tacker::BinaryOperator::GreaterOrEqual),
                src1,
                src2,
                dst,
            ) => {
                instructions.push(Instruction::Cmp(val_to_operand(src2), val_to_operand(src1)));
                instructions.push(Instruction::Mov(
                    Operand::Imm(0),
                    val_to_operand(dst.clone()),
                ));
                instructions.push(Instruction::SetCC(
                    match operator {
                        tacker::BinaryOperator::EqualTo => CondCode::E,
                        tacker::BinaryOperator::NotEqual => CondCode::NE,
                        tacker::BinaryOperator::LessThan => CondCode::L,
                        tacker::BinaryOperator::LessOrEqual => CondCode::LE,
                        tacker::BinaryOperator::GreaterThan => CondCode::G,
                        tacker::BinaryOperator::GreaterOrEqual => CondCode::GE,
                        _ => {
                            panic!("Only checked for these")
                        }
                    },
                    val_to_operand(dst),
                ));
            }
            tacker::Instruction::FunctionCall(func_name, args, dest) => {
                let arg_registers = [Reg::DI, Reg::SI, Reg::DX, Reg::CX, Reg::R8, Reg::R9];

                let mut register_args = args;
                let stack_args = if register_args.len() >= 6 {
                    register_args.split_off(6)
                } else {
                    Vec::new()
                };

                let stack_padding = if (stack_args.len() & 1) == 1 { 8 } else { 0 };

                if stack_padding != 0 {
                    instructions.push(Instruction::AllocateStack(stack_padding));
                }

                let mut reg_index = 0;
                for tacky_arg in register_args {
                    let reg = &arg_registers[reg_index];
                    let assembly_arg = val_to_operand(tacky_arg);
                    instructions.push(Instruction::Mov(assembly_arg, Operand::Reg(reg.clone())));
                    reg_index += 1;
                }

                let stack_args_len = stack_args.len() as i64;
                for tacky_arg in stack_args {
                    let assembly_arg = val_to_operand(tacky_arg);
                    match assembly_arg {
                        Operand::Imm(_) | Operand::Reg(_) => {
                            instructions.push(Instruction::Push(assembly_arg));
                        }
                        _ => {
                            instructions
                                .push(Instruction::Mov(assembly_arg, Operand::Reg(Reg::AX)));
                            instructions.push(Instruction::Push(Operand::Reg(Reg::AX)));
                        }
                    }
                }

                instructions.push(Instruction::Call(func_name));

                let bytes_to_remove: i64 = 8 * stack_args_len + stack_padding;
                if bytes_to_remove != 0 {
                    instructions.push(Instruction::DeallocateStack(bytes_to_remove));
                }

                let assembly_dest = val_to_operand(dest);
                instructions.push(Instruction::Mov(Operand::Reg(Reg::AX), assembly_dest));
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
        println!("{:?}", instruction);
        match instruction {
            Instruction::Mov(src, dst) => {
                println!("changing mov");
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
                println!("changing bin");
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
            Instruction::Cmp(operand1, operand2) => {
                *instruction = Instruction::Cmp(
                    replace_psuedo_operand_if_needed(
                        operand1.clone(),
                        &mut identifiers,
                        &mut stack_size,
                    ),
                    replace_psuedo_operand_if_needed(
                        operand2.clone(),
                        &mut identifiers,
                        &mut stack_size,
                    ),
                );
            }
            Instruction::SetCC(cc, Operand::Pseudo(name)) => {
                *instruction = Instruction::SetCC(
                    cc.clone(),
                    Operand::Stack(get_identifier_offset(
                        &mut identifiers,
                        &mut stack_size,
                        name,
                    )),
                );
            }
            Instruction::Push(Operand::Pseudo(name)) => {
                *instruction = Instruction::Push(Operand::Stack(get_identifier_offset(
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
    instructions.push(Instruction::AllocateStack(((stack_size + 15) / 16) * 16));

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
            Instruction::Cmp(Operand::Stack(offset1), Operand::Stack(offset2)) => {
                instructions.push(Instruction::Mov(
                    Operand::Stack(offset1),
                    Operand::Reg(Reg::R10),
                ));
                instructions.push(Instruction::Cmp(
                    Operand::Reg(Reg::R10),
                    Operand::Stack(offset2),
                ));
            }
            Instruction::Cmp(operand1, Operand::Imm(constant2)) => {
                instructions.push(Instruction::Mov(
                    Operand::Imm(constant2),
                    Operand::Reg(Reg::R11),
                ));
                instructions.push(Instruction::Cmp(operand1, Operand::Reg(Reg::R11)));
            }
            _ => instructions.push(instruction),
        }
    }

    instructions
}

pub fn assemble(program: tacker::Program) -> Program {
    let mut functions: Vec<FunctionDefinition> = Vec::new();
    for function in program.functions {
        functions.push(FunctionDefinition {
            name: function.identifier.clone(),
            instructions: tacky_function_to_assembly(function),
        });
    }
    let mut fixed_up_functions: Vec<FunctionDefinition> = Vec::new();
    for mut function in functions {
        let stack_size = replace_pseudo_operands(&mut function.instructions);
        fixed_up_functions.push(FunctionDefinition {
            name: function.name,
            instructions: fix_up(function.instructions, stack_size),
        });
    }

    Program {
        functions: fixed_up_functions,
    }
}

fn val_to_operand(val: tacker::Val) -> Operand {
    match val {
        tacker::Val::Constant(value) => Operand::Imm(value),
        tacker::Val::Var(name) => Operand::Pseudo(name),
    }
}
