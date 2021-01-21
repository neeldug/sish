use crate::parallel::parallel_dispatch;
use std::env;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::{exit, Command};

// TODO: implement parallelism in shell

struct Program<'a> {
    command: &'a str,
    args: Vec<String>,
}

impl Program<'_> {
    fn new(command: &str, args: Vec<String>) -> Program {
        Program { command, args }
    }
}

fn dispatch_command<'a>(command: Program, path: &mut Vec<PathBuf>) -> Result<(), &'a str> {
    let instruction = command.command;
    let args = command.args;
    let mut ret = Ok(());

    match instruction {
        "exit" => {
            exit(0);
        }
        "cd" => match args.len() {
            1 => {
                if env::set_current_dir(PathBuf::from(args[0].clone())).is_err() {
                    ret = Err("invalid directory for cd")
                }
            }
            _ => {
                ret = Err("invalid number of args for cd");
            }
        },
        "path" => match args.len() {
            0 => eprintln!("{:?}", path),
            _ => {
                path.clear();
                for arg in args {
                    path.push(PathBuf::from(arg));
                }
            }
        },
        &_ => match check_path(instruction, path) {
            true => {
                let _exit_code = Command::new(instruction)
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

pub(crate) fn interactive_loop(mut path: Vec<PathBuf>) {
    loop {
        let stdin = io::stdin();
        eprint!("sish> ");
        let line = stdin.lock().lines().next().unwrap().unwrap();
        if line.contains('&') {
            let split = line.split('&').map(|e| e.to_string());
            let mut parallel_cmds = Vec::new();
            for commands in split {
                let cmd = commands
                    .split_whitespace()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>();
                parallel_cmds.push(cmd);
            }
            parallel_dispatch(parallel_cmds);
        } else {
            let split = line.split_whitespace().skip(1).map(|e| e.to_string());
            let args = split.collect::<Vec<String>>();
            let command = line.split_whitespace().collect::<Vec<&str>>()[0];
            // Build my struct:
            let exec = Program::new(command, args);
            match dispatch_command(exec, &mut path) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e)
                }
            }
        }
    }
}

fn check_path(command: &str, path: &[PathBuf]) -> bool {
    for p in path {
        if p.join(command).exists() {
            return true;
        }
    }
    false
}
