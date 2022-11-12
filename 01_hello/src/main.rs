use std::process::Command;
fn main() {
    #[cfg(target_os = "linux")]
    static CMD: &str = "exa";
    #[cfg(target_os = "windows")]
    static CMD: &str = "ls";

    let mut command = Command::new(CMD);
    let outputs = command.output();  
    println!("{:?}", outputs.unwrap());
}
