use std::env;

use ema::{repl::repl, run_code};

fn main() {
    let cwd = env::current_dir().unwrap();
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        repl();
    } else {
        let file_name = &args[1];
        let file_path = format!("{}/{}", cwd.display(), file_name);
        println!("{}", file_path);

        run_code(file_path);
    }
}
