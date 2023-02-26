use std::path::Path;
use std::process::Command;

fn main() {
    let program = "/usr/bin/curl";
    let path = Path::new(program);
    if !path.is_file() {
        std::process::exit(1);
    }
    let args = vec!["--help"];
    Command::new(program)
        .args(args)
        .spawn()
        .expect(&format!("Failed to start external executable! ({program})"))
        .wait()
        .expect(&format!("Failed to wait external executable! ({program})"));
}
