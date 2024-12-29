use compiler;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: program <output_executable> <source_files...> [--lex|--parse|--validate|--tacky|--codegen] [-c]");
        std::process::exit(1);
    }

    let executable_file = &args[1];
    let source_files: Vec<String> = args[2..args.len()]
        .iter()
        .filter(|&arg| !arg.starts_with("-"))
        .cloned()
        .collect();

    let stop_at = args.iter().find(|&arg| {
        matches!(
            arg.as_str(),
            "--lex" | "--parse" | "--validate" | "--tacky" | "--codegen"
        )
    });
    let no_main = args.contains(&"-c".to_string());

    compiler::run(executable_file, source_files, stop_at, no_main)
}
