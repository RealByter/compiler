use crate::assembler;
use std::fs::File;
use std::io::{self, Write};

pub fn generate(file_path: &str, program: assembler::Program) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    writeln!(file, "\t.globl {}", program.function.name)?;
    writeln!(file, "{}:", program.function.name)?;
    for instruction in program.function.instructions {
        match instruction {
            assembler::Instruction::Mov(src, dst) => {
                write!(file, "\tmovl ")?;
                match src {
                    assembler::Operand::Imm(value) => {
                        write!(file, "${}, ", value)?;
                    }
                    assembler::Operand::Register => {
                        write!(file, "%eax, ")?;
                    }
                }
                match dst {
                    assembler::Operand::Imm(_) => {
                        panic!("Invalid destination operand.");
                    }
                    assembler::Operand::Register => {
                        writeln!(file, "%eax")?;
                    }
                }
            }
            assembler::Instruction::Ret => {
                writeln!(file, "\tret")?;
            }
        }
    }

    writeln!(file, ".section .note.GNU-stack,\"\",@progbits")?;

    Ok(())
}
