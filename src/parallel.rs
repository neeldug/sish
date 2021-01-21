use std::process::Command;
use std::sync::{Arc, Barrier};
use threadpool::ThreadPool;

pub(crate) fn parallel_dispatch(commands: Vec<Vec<String>>) {
    let n_workers = commands.len();
    let n_jobs = commands.len();
    let pool = ThreadPool::new(n_workers);

    let barrier = Arc::new(Barrier::new(n_jobs + 1));
    for command in commands {
        let barrier = barrier.clone();
        pool.execute(move || {
            let args: Vec<&str> = command[1..].iter().map(AsRef::as_ref).collect();
            let _exit_code = Command::new(command[0].as_str())
                .args(args)
                .status()
                .expect("process failed to execute");
            barrier.wait();
        });
    }
    barrier.wait();
}
