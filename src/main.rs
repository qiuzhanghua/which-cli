use std::env;
use which::which;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: which <command>...");
        std::process::exit(1);
    }

    let mut exit_code = 0;

    for arg in args.iter().skip(1) {
        match which(arg) {
            Ok(path) => {
                println!("{}", path.display());
            }
            Err(_e) => {
                exit_code = -1;
            }
        }
    }

    std::process::exit(exit_code);
}
