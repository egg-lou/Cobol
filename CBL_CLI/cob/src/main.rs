use std::process::{Command, exit};
use std::path::Path;
use clap::{App, Arg};

fn main() {
    let matches = App::new("COBOL CLI")
        .version("1.0")
        .author("EggLou")
        .about("Compile and run COBOL programs")
        .subcommand(
            App::new("run")
                .about("Compile and run a COBOL program")
                .arg(Arg::with_name("filename").index(1).required(true))
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(run_matches)) => {
            let filename = run_matches.value_of("filename").unwrap();
            let name = Path::new(filename)
                .file_stem()
                .expect("Invalid file name")
                .to_str()
                .expect("Invalid file name");

            let compilation_result = compile_cobol(filename);

            if compilation_result.success() {
                let executable_name = format!("{}.exe", name);
                run_cobol_program(&executable_name);
                print_red("\nProgram Terminated.");
            } else {
                println!("COBOL compilation failed.");
                exit(1);
            }
        }
        _ => {
            println!("Usage: cob run <filename.cbl>");
            exit(1);
        }
    }
}

fn compile_cobol(filename: &str) -> std::process::ExitStatus {
    let path = Path::new(filename);
    let stem = path.file_stem().expect("Invalid file name").to_str().expect("Invalid file name");
    let executable_name = format!("{}.exe", stem);

    let compilation_result = Command::new("cobc")
        .args(&["-x", filename, "-o", &executable_name])
        .status()
        .expect("Failed to execute COBOL compilation command.");

    compilation_result
}

fn run_cobol_program(filename: &str) {
    clear_terminal();
    print_green("Program is running....\n");
    let run_command = format!("./{}", filename);

    let execution_result = Command::new(run_command)
        .status()
        .expect("Failed to execute COBOL program.");

    if !execution_result.success() {
        print_red("Failed to run the compiled COBOL program.");
        exit(1);
    }
}

fn clear_terminal() {
    Command::new("clear")
        .status()
        .expect("Failed to clear the terminal");
}

fn print_green(text: &str) {
    println!("\x1B[32m{}\x1B[0m", text);
}

fn print_red(text: &str) {
    println!("\x1B[31m{}\x1B[0m", text);
}