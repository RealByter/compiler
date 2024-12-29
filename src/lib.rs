use std::fs;
use std::io;

mod assembler;
mod gcc;
mod generator;
mod lexer;
mod loop_labeler;
mod parser;
mod tacker;
mod variable_resolver;

pub fn run(
    executable_file: &str,
    source_files: Vec<String>,
    stop_at: Option<&String>,
    no_main: bool,
) -> io::Result<()> {
    let mut assembly_files: Vec<String> = Vec::new();
    for source_file in &source_files {
        let base_name = match source_file.rfind('.') {
            Some(pos) => &source_file[..pos],
            None => source_file,
        };

        println!("{}:", base_name);

        let input = fs::read_to_string(source_file)?;

        // let preprocessed_file = format!("{}.i", base_name);
        let assembly_file = format!("{}.s", base_name);

        let tokens = lexer::tokenize(&input);
        // gcc::preprocess(input_file, &preprocessed_file)?;
        for token in &tokens {
            println!("{:?}", token);
        }

        if stop_at == Some(&"--lex".to_string()) {
            continue;
        }

        // gcc::generate_assembly(&preprocessed_file, &assembly_file)?;
        let program = parser::parse_program(&mut tokens.into_iter().peekable()).unwrap();
        println!("{:#?}", program);
        if stop_at == Some(&"--parse".to_string()) {
            continue;
        }

        let program = variable_resolver::resolve_variables(program).unwrap();
        println!("{:#?}", program);
        let program = loop_labeler::label_loops(program).unwrap();
        if stop_at == Some(&"--validate".to_string()) {
            continue;
        }

        let tacky = tacker::generate_tacky(program);
        println!("{:#?}", tacky);
        if stop_at == Some(&"--tacky".to_string()) {
            continue;
        }

        let assembly = assembler::assemble(tacky);
        println!("{:#?}", assembly);

        if stop_at == Some(&"--codegen".to_string()) {
            continue;
        }

        generator::generate(&assembly_file, assembly)?;
        assembly_files.push(assembly_file);
    }

    gcc::compile_executable(&assembly_files, &executable_file, no_main)?;

    Ok(())
}
