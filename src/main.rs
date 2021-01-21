use std::env;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{exit, Command};

struct Program<'a> {
    command: &'a str,
    args: Vec<&'a str>,
    path: &'a Path,
}

fn main() {
    // Interactive mode for wish CLI
    let path = Path::new("/bin/");
    loop {
        let stdin = io::stdin();
        eprint!("wish> ");
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let split = line
            .split_whitespace()
            .enumerate()
            .filter(|&(i, _)| i > 0)
            .map(|(_, e)| e);
        let args = split.collect::<Vec<&str>>();
        let command = line.split_whitespace().collect::<Vec<&str>>()[0];
        // Build my struct:
        let exec = Program {
            command,
            args,
            path,
        };
        dispatch_command(exec);
    }
}

fn dispatch_command(command: Program) -> Result<(), &str> {
    let instruction = command.command;
    let path = command.path;
    let args = command.args;
    let mut ret = Ok(());
    match instruction {
        "exit" => {
            exit(0);
        }
        "cd" => match args.len() {
            1 => {
                env::set_current_dir(Path::new(args[0])).expect("invalid directory");
            }
            _ => {
                ret = Err("invalid number of args for cd");
            }
        },
        "path" => {}
        &_ => match check_path(instruction, path) {
            true => {
                let cli_command = Command::new(instruction)
                    .args(args)
                    .status()
                    .expect("process failed to execute");
            }
            false => {
                ret = Err("Not Found in Path");
            }
        },
    }
    ret
}

fn check_path(command: &str, path: &Path) -> bool {
    path.join(command).exists()
}
