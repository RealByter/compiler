use std::fs;
use std::io;

mod gcc;
mod lexer;
mod parser;

pub fn run(input_file: &str, stop_at: Option<&str>) -> io::Result<()> {
    let base_name = match input_file.rfind('.') {
        Some(pos) => &input_file[..pos],
        None => input_file,
    };

    let input = fs::read_to_string(input_file)?;
    let mut tokens = lexer::tokenize(&input);
    for token in &tokens {
        println!("{:?}", token);
    }
    let program = parser::parse_program(&mut tokens);
    println!("{:?}", program);

    let preprocessed_file = format!("{}.i", base_name);
    let assembly_file = format!("{}.s", base_name);
    let executable_file = format!("{}.out", base_name);

    gcc::preprocess(input_file, &preprocessed_file)?;

    if stop_at == Some("--parse") {
        return Ok(());
    }

    gcc::generate_assembly(&preprocessed_file, &assembly_file)?;

    if stop_at == Some("--codegen") {
        return Ok(());
    }

    gcc::compile_executable(&assembly_file, &executable_file)?;

    Ok(())
}
