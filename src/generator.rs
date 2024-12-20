use crate::assembler;
use std::fs::File;
use std::io::{self, Write};

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
                match src {
                    assembler::Operand::Imm(value) => {
                        write!(file, "${}, ", value)?;
                    }
                    assembler::Operand::Reg(reg) => match reg {
                        assembler::Reg::AX => write!(file, "%eax, ")?,
                        assembler::Reg::R10D => write!(file, "%r10d, ")?,
                    },
                    assembler::Operand::Stack(offset) => {
                        write!(file, "{}(%rbp), ", offset)?;
                    }
                    assembler::Operand::Pseudo(_) => {
                        panic!("Shouldn't have a pseudo register at this stage.");
                    }
                }
                match dst {
                    assembler::Operand::Imm(_) => {
                        panic!("Invalid destination operand.");
                    }
                    assembler::Operand::Reg(reg) => match reg {
                        assembler::Reg::AX => writeln!(file, "%eax")?,
                        assembler::Reg::R10D => writeln!(file, "%r10d")?,
                    },
                    assembler::Operand::Stack(offset) => {
                        writeln!(file, "{}(%rbp)", offset)?;
                    }
                    assembler::Operand::Pseudo(_) => {
                        panic!("Shouldn't have a pseudo register at this stage.");
                    }
                }
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

                match operand {
                    assembler::Operand::Imm(_) => {
                        panic!("Invalid operand.");
                    }
                    assembler::Operand::Reg(reg) => match reg {
                        assembler::Reg::AX => writeln!(file, "%eax")?,
                        assembler::Reg::R10D => writeln!(file, "%r10d")?,
                    },
                    assembler::Operand::Stack(offset) => {
                        writeln!(file, "{}(%rbp)", offset)?;
                    }
                    assembler::Operand::Pseudo(_) => {
                        panic!("Shouldn't have a pseudo register at this stage.");
                    }
                }
            }
            assembler::Instruction::Ret => {
                writeln!(file, "movq %rbp, %rsp")?;
                writeln!(file, "popq %rbp")?;
                writeln!(file, "\tret")?;
            }
        }
    }

    writeln!(file, ".section .note.GNU-stack,\"\",@progbits")?;

    Ok(())
}
