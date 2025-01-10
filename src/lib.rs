use std::fs;
use std::io;

mod assembler;
mod gcc;
mod generator;
mod identifier_resolver;
mod lexer;
mod parser;
mod semantic_analyzer;
mod tacker;
mod type_checker;

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

        let program = identifier_resolver::resolve_identifiers(program).unwrap();
        println!("{:#?}", program);
        let program = semantic_analyzer::analyze_semantics(program).unwrap();
        let symbol_table = type_checker::check_types(&program).unwrap();
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

        generator::generate(&assembly_file, symbol_table, assembly)?;
        assembly_files.push(assembly_file);
    }

    if stop_at.is_some() {
        Ok(())
    } else {
        gcc::compile_executable(&assembly_files, &executable_file, no_main)
    }
}
