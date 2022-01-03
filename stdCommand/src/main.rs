//https://doc.rust-lang.org/std/process/struct.Command.html

use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

fn main() {


    // start_service();

}



/**
 *espanso 启动服务
**/
#[test]
fn test_start_espanso() {
    // "D:\ideawork\espanso\target\debug\espanso.exe"
    // https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html

    // start_service文件路径：      espanso/src/cli/service/win.rs:95

    let path = PathBuf::from(r"D:/ideawork/rustDemo/target/debug/_02_gussing_game.exe");
    // let path = PathBuf::from(r"D:/ideawork/rustDemo/target/debug/_03_variables.exe");
    Command::new(path)
        .args(&["launcher"])
        .creation_flags(0x08000008) // CREATE_NO_WINDOW + DETACHED_PROCESS
        // .spawn()?;
        .spawn();
}


/**
 *@Description
 *@Date 2022/1/3 22:07
**/
#[test]
fn test_start_variable_count() {

    let path = PathBuf::from(r"D:\ideawork\espanso\target\debug\espanso.exe");
    Command::new(path)
        .args(&["launcher"])
        .creation_flags(0x08000008) // CREATE_NO_WINDOW + DETACHED_PROCESS
        // .spawn()?;
        .spawn();


}




 /**
  *@Description
  *@Date 2022/1/3 20:55
 **/
 #[test]
 fn test_cmd() {
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



/**
 *@Description  cmd的复用    output() 可以实现
 *@Date 2022/1/3 14:20
**/
#[test]
fn test_echo_hello() {
    let mut echo_hello = Command::new("cmd");

    echo_hello.args(["/C", "echo hellotwc"]);
    // echo_hello.arg("-c").arg("echo hello");
    let hello_1 = echo_hello.output().expect("failed to execute process");
    println!("{:?}", hello_1);
    let hello_2 = echo_hello.output().expect("failed to execute process");
    println!("{:?}", hello_2);
}


/**
 *@Description
 *@Date 2022/1/3 21:04
**/
#[test]
fn test_spawn() {
    let mut echo_hello = Command::new("cmd");

    echo_hello.args(["/C", "echo hellotwc"]);

    echo_hello.spawn().expect("spawn  失败");



}







/**
 *@Description  windows上不可用
 *@Date 2022/1/3 20:35
**/
#[test]
fn test_ls() {
    let mut list_dir = Command::new("ls");

// Execute `ls` in the current directory of the program.
//     list_dir.status().expect("process failed to execute");

    // println!();

// Change `ls` to execute in the root directory.
    list_dir.current_dir(r"D:\data");

// And then execute `ls` again but in the root directory.
    list_dir.status().expect("process failed to execute");

}





/**
 *@Description   windows 上不可用
 *@Date 2022/1/3 14:13
**/ 
#[test]
fn test_current_dir() {
    Command::new("ls")
        .current_dir("/bin")
        .spawn()
        .expect("ls command failed to start");
}