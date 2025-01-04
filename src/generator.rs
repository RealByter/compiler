use crate::assembler;
use std::fs::File;
use std::io::{self, Write};
use crate::type_checker::SymbolTable;

fn write_reg(file: &mut File, reg: assembler::Reg) -> io::Result<()> {
    match reg {
        assembler::Reg::AX => write!(file, "%rax")?,
        assembler::Reg::DX => write!(file, "%rdx")?,
        assembler::Reg::CX => write!(file, "%rcx")?,
        assembler::Reg::DI => write!(file, "%rdi")?,
        assembler::Reg::SI => write!(file, "%rsi")?,
        assembler::Reg::R8 => write!(file, "%r8")?,
        assembler::Reg::R9 => write!(file, "%r9")?,
        assembler::Reg::R10 => write!(file, "%r10")?,
        assembler::Reg::R11 => write!(file, "%r11")?,
    }
    Ok(())
}

fn write_reg_double(file: &mut File, reg: assembler::Reg) -> io::Result<()> {
    match reg {
        assembler::Reg::AX => write!(file, "%eax")?,
        assembler::Reg::DX => write!(file, "%edx")?,
        assembler::Reg::CX => write!(file, "%ecx")?,
        assembler::Reg::DI => write!(file, "%edi")?,
        assembler::Reg::SI => write!(file, "%esi")?,
        assembler::Reg::R8 => write!(file, "%r8d")?,
        assembler::Reg::R9 => write!(file, "%r9d")?,
        assembler::Reg::R10 => write!(file, "%r10d")?,
        assembler::Reg::R11 => write!(file, "%r11d")?,
    }
    Ok(())
}

fn write_reg_byte(file: &mut File, reg: assembler::Reg) -> io::Result<()> {
    match reg {
        assembler::Reg::AX => write!(file, "%al")?,
        assembler::Reg::DX => write!(file, "%dl")?,
        assembler::Reg::CX => write!(file, "%cl")?,
        assembler::Reg::DI => write!(file, "%dil")?,
        assembler::Reg::SI => write!(file, "%sil")?,
        assembler::Reg::R8 => write!(file, "%r8b")?,
        assembler::Reg::R9 => write!(file, "%r9b")?,
        assembler::Reg::R10 => write!(file, "%r10b")?,
        assembler::Reg::R11 => write!(file, "%r11b")?,
    }
    Ok(())
}

fn write_stack(file: &mut File, offset: i64) -> io::Result<()> {
    write!(file, "{}(%rbp)", offset)
}

fn write_imm(file: &mut File, value: i64) -> io::Result<()> {
    write!(file, "${}", value)
}

fn write_operand_double(file: &mut File, operand: assembler::Operand) -> io::Result<()> {
    match operand {
        assembler::Operand::Imm(value) => write_imm(file, value)?,
        assembler::Operand::Reg(reg) => write_reg_double(file, reg)?,
        assembler::Operand::Stack(offset) => write_stack(file, offset)?,
        assembler::Operand::Pseudo(_) => panic!("Shouldn't have a pseudo register at this stage."),
    }
    Ok(())
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

fn write_operand_byte(file: &mut File, operand: assembler::Operand) -> io::Result<()> {
    match operand {
        assembler::Operand::Imm(value) => write_imm(file, value)?,
        assembler::Operand::Reg(reg) => write_reg_byte(file, reg)?,
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
        assembler::Operand::Reg(reg) => write_reg_double(file, reg)?,
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

// FIRST LIFETIME USAGE
fn cond_code_to_letters<'a>(cc: assembler::CondCode) -> &'a str {
    match cc {
        assembler::CondCode::E => "e",
        assembler::CondCode::NE => "ne",
        assembler::CondCode::L => "l",
        assembler::CondCode::LE => "le",
        assembler::CondCode::G => "g",
        assembler::CondCode::GE => "ge",
    }
}

pub fn generate(file_path: &str, symbol_table: SymbolTable, program: assembler::Program) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for function in program.functions {
        generate_function(&mut file, &symbol_table, function)?;
    }

    writeln!(file, ".section .note.GNU-stack,\"\",@progbits")?;

    Ok(())
}

fn generate_function(file: &mut File, symbol_table: &SymbolTable, function: assembler::FunctionDefinition) -> io::Result<()> {
    writeln!(file, "\t.globl {}", function.name)?;
    writeln!(file, "{}:", function.name)?;
    writeln!(file, "\tpushq %rbp")?;
    writeln!(file, "\tmovq %rsp, %rbp")?;
    for instruction in function.instructions {
        match instruction {
            assembler::Instruction::AllocateStack(size) => {
                writeln!(file, "\tsubq ${}, %rsp", size)?;
            }
            assembler::Instruction::Mov(src, dst) => {
                write!(file, "\tmovl ")?;
                write_operand_double(file, src)?;
                seperate(file)?;
                write_operand_not_imm(file, dst)?;
                newline(file)?;
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

                write_operand_not_imm(file, operand)?;
                newline(file)?;
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

                write_operand_double(file, operand1)?;
                seperate(file)?;
                write_operand_not_imm(file, operand2)?;
                newline(file)?;
            }
            assembler::Instruction::Idiv(operand) => {
                write!(file, "\tidivl ")?;
                write_operand_not_imm(file, operand)?;
                newline(file)?;
            }
            assembler::Instruction::Cdq => {
                writeln!(file, "\tcdq")?;
            }
            assembler::Instruction::Cmp(operand1, operand2) => {
                write!(file, "\tcmpl ")?;
                write_operand_double(file, operand1)?;
                seperate(file)?;
                write_operand_not_imm(file, operand2)?;
                newline(file)?;
            }
            assembler::Instruction::Jmp(target) => {
                writeln!(file, "\tjmp .L{}", target)?;
            }
            assembler::Instruction::JmpCC(cc, target) => {
                writeln!(file, "\tj{} .L{}", cond_code_to_letters(cc), target)?;
            }
            assembler::Instruction::SetCC(cc, dst) => {
                write!(file, "\tset{} ", cond_code_to_letters(cc))?;
                write_operand_byte(file, dst)?;
                newline(file)?;
            }
            assembler::Instruction::Label(identifier) => {
                writeln!(file, ".L{}:", identifier)?;
            }
            assembler::Instruction::Ret => {
                writeln!(file, "\tmovq %rbp, %rsp")?;
                writeln!(file, "\tpopq %rbp")?;
                writeln!(file, "\tret")?;
            }
            assembler::Instruction::DeallocateStack(size) => {
                writeln!(file, "\taddq ${}, %rsp", size)?;
            },
            assembler::Instruction::Push(operand) => {
                write!(file, "\tpushq ")?;
                write_operand(file, operand)?;
                newline(file)?;
            },
            assembler::Instruction::Call(label) => {
                if symbol_table.contains_key(&label) {
                    writeln!(file, "\tcall {}", label)?;
                } else {
                    writeln!(file, "\tcall {}@PLT", label)?;
                }
            },
        }
    }

    Ok(())
}