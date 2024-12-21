use std::env;
use std::io;
use compiler;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 && args.len() != 3 {
        eprintln!("Invalid args. Should be: \"program <input_file> [--lex|--parse|--tacky|--codegen]\"");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let stop_at = if args.len() == 3 {
        Some(args[2].as_str())
    } else {
        None
    };

    compiler::run(input_file, stop_at)
}
