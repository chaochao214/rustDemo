//https://doc.rust-lang.org/std/process/struct.Command.html

use std::process::Command;

fn main() {
    printCmd()

}

fn printCmd() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hellotwc"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let hello = output;
    println!("{:?}", hello);
}
