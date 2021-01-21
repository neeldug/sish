mod batch;
mod interactive;

use crate::batch::batch_execute;
use crate::interactive::interactive_loop;
use std::env;
use std::path::PathBuf;

fn main() {
    // Interactive mode for wish CLI
    let init = PathBuf::from("/bin/");
    let path = vec![init];
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        1 => interactive_loop(path),
        2 => batch_execute(path, PathBuf::from(args[1].clone())),
        _ => eprintln!("too many args"),
    }
}
