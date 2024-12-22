use std::fs;
use std::io;

mod assembler;
mod gcc;
mod generator;
mod lexer;
mod parser;
mod tacker;

pub fn run(input_file: &str, stop_at: Option<&str>) -> io::Result<()> {
    let base_name = match input_file.rfind('.') {
        Some(pos) => &input_file[..pos],
        None => input_file,
    };

    let input = fs::read_to_string(input_file)?;

    // let preprocessed_file = format!("{}.i", base_name);
    let assembly_file = format!("{}.s", base_name);
    let executable_file = format!("{}.out", base_name);

    let tokens = lexer::tokenize(&input);
    // gcc::preprocess(input_file, &preprocessed_file)?;
    for token in &tokens {
        println!("{:?}", token);
    }

    if stop_at == Some("--lex") {
        return Ok(());
    }

    // gcc::generate_assembly(&preprocessed_file, &assembly_file)?;
    let program = parser::parse_program(&mut tokens.into_iter().peekable()).unwrap();
    println!("{:#?}", program);

    if stop_at == Some("--parse") {
        return Ok(());
    }

    let tacky = tacker::generate_tacky(program);
    println!("{:#?}", tacky);
    if stop_at == Some("--tacky") {
        return Ok(());
    }

    let assembly = assembler::assemble(tacky);
    println!("{:?}", assembly);
    
    if stop_at == Some("--codegen") {
        return Ok(());
    }
    
    generator::generate(&assembly_file, assembly)?;
    gcc::compile_executable(&assembly_file, &executable_file)?;

    Ok(())
}
