use std::io;
use std::process::Command;

// pub fn preprocess(input_file: &str, preprocessed_file: &str) -> io::Result<()> {
//     let status = Command::new("gcc")
//         .args(["-E", "-P", input_file, "-o", preprocessed_file])
//         .status()?;
//     if !status.success() {
//         return Err(io::Error::new(io::ErrorKind::Other, "Preprocessing failed"));
//     }
//     Ok(())
// }

// pub fn generate_assembly(preprocessed_file: &str, assembly_file: &str) -> io::Result<()> {
//     let status = Command::new("gcc")
//         .args([
//             "-S",
//             "-O",
//             "-fno-asynchronous-unwind-tables",
//             "-fcf-protection=none",
//             preprocessed_file,
//             "-o",
//             assembly_file,
//         ])
//         .status()?;
//     fs::remove_file(preprocessed_file)?;
//     if !status.success() {
//         return Err(io::Error::new(
//             io::ErrorKind::Other,
//             "Assembly generation failed",
//         ));
//     }
//     Ok(())
// }

pub fn compile_executable(
    assembly_files: &[String],
    executable_file: &str,
    no_main: bool,
) -> io::Result<()> {
    let mut args = assembly_files.to_vec();
    args.push("-o".into());
    args.push(executable_file.into());

    if no_main {
        args.push("-c".into());
    }

    let status = Command::new("gcc").args(&args).status()?;

    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Compilation failed"));
    }
    Ok(())
}
