use crate::assembler;
use std::fs::File;
use std::io::{self, Write};

fn write_reg(file: &mut File, reg: assembler::Reg) -> io::Result<()> {
    match reg {
        assembler::Reg::AX => write!(file, "%eax")?,
        assembler::Reg::DX => write!(file, "%edx")?,
        assembler::Reg::R10 => write!(file, "%r10d")?,
        assembler::Reg::R11 => write!(file, "%r11d")?,
    }
    Ok(())
}

fn write_stack(file: &mut File, offset: i64) -> io::Result<()> {
    write!(file, "{}(%rbp)", offset)
}

fn write_imm(file: &mut File, value: i64) -> io::Result<()> {
    write!(file, "${}", value)
}

fn write_operand(file: &mut File, operand: assembler::Operand) -> io::Result<()> {
    match operand {
        assembler::Operand::Imm(value) => write_imm(file, value)?,
        assembler::Operand::Reg(reg) => write_reg(file, reg)?,
        assembler::Operand::Stack(offset) => write_stack(file, offset)?,
        assembler::Operand::Pseudo(_) => panic!("Shouldn't have a pseudo register at this stage."),
    }
    Ok(())
}

fn write_operand_not_imm(file: &mut File, operand: assembler::Operand) -> io::Result<()> {
    match operand {
        assembler::Operand::Imm(_) => {
            panic!("Invalid operand imm.");
        }
        assembler::Operand::Reg(reg) => write_reg(file, reg)?,
        assembler::Operand::Stack(offset) => write_stack(file, offset)?,
        assembler::Operand::Pseudo(_) => {
            panic!("Shouldn't have a pseudo register at this stage.");
        }
    }
    Ok(())
}

fn seperate(file: &mut File) -> io::Result<()> {
    write!(file, ", ")
}

fn newline(file: &mut File) -> io::Result<()> {
    writeln!(file)
}

pub fn generate(file_path: &str, program: assembler::Program) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    writeln!(file, "\t.globl {}", program.function.name)?;
    writeln!(file, "{}:", program.function.name)?;
    writeln!(file, "\tpushq %rbp")?;
    writeln!(file, "\tmovq %rsp, %rbp")?;
    for instruction in program.function.instructions {
        match instruction {
            assembler::Instruction::AllocateStack(size) => {
                writeln!(file, "\tsubq ${}, %rsp", size)?;
            }
            assembler::Instruction::Mov(src, dst) => {
                write!(file, "\tmovl ")?;
                write_operand(&mut file, src)?;
                seperate(&mut file)?;
                write_operand_not_imm(&mut file, dst)?;
                newline(&mut file)?;
            }
            assembler::Instruction::Unary(operator, operand) => {
                match operator {
                    assembler::UnaryOperator::Neg => {
                        write!(file, "\tnegl ")?;
                    }
                    assembler::UnaryOperator::Not => {
                        write!(file, "\tnotl ")?;
                    }
                }

                write_operand_not_imm(&mut file, operand)?;
                newline(&mut file)?;
            }
            assembler::Instruction::Binary(operator, operand1, operand2) => {
                match operator {
                    assembler::BinaryOperator::Add => {
                        write!(file, "\taddl ")?;
                    }
                    assembler::BinaryOperator::Sub => {
                        write!(file, "\tsubl ")?;
                    }
                    assembler::BinaryOperator::Mult => {
                        write!(file, "\timull ")?;
                    }
                    assembler::BinaryOperator::And => {
                        write!(file, "\tandl ")?;
                    }
                    assembler::BinaryOperator::Or => {
                        write!(file, "\torl ")?;
                    }
                    assembler::BinaryOperator::Xor => {
                        write!(file, "\txorl ")?;
                    }
                    assembler::BinaryOperator::LeftShift => {
                        write!(file, "\tshl ")?;
                    }
                    assembler::BinaryOperator::RightShift => {
                        write!(file, "\tshr ")?;
                    }
                }

                write_operand(&mut file, operand1)?;
                seperate(&mut file)?;
                write_operand_not_imm(&mut file, operand2)?;
                newline(&mut file)?;
            }
            assembler::Instruction::Idiv(operand) => {
                write!(file, "\tidivl ")?;
                write_operand_not_imm(&mut file, operand)?;
                newline(&mut file)?;
            }
            assembler::Instruction::Cdq => {
                writeln!(file, "\tcdq")?;
            }
            assembler::Instruction::Ret => {
                writeln!(file, "\tmovq %rbp, %rsp")?;
                writeln!(file, "\tpopq %rbp")?;
                writeln!(file, "\tret")?;
            }
        }
    }

    writeln!(file, ".section .note.GNU-stack,\"\",@progbits")?;

    Ok(())
}
