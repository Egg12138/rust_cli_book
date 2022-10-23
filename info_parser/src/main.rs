/// 我们在这里学习如何解析一些简单的`.toml`配置文件
/// 参考rustlings源码 && crate toml源码
/// ```rust
/// use serde_derive::Deserialize; 
/// #[derive(Deserialize)]
/// struct Config {
///     ip: String,
///     port: Option<u16>,
///     keys: Keys,     
/// }
/// #[derive(Deserialize)]
/// struct Keys {
///     github: String,
///     travis: Option<String>
/// }
/// 
/// fn main() {
///     let config_raw_str = r#"
///     ip = '127.0.0.1'
/// 
/// 
///     [keys] 
///     github = 'git@github.com:Egg12138
///     travis = '...'
/// "#;
///     let config: Config = toml::from_str(config_raw_str).unwrap();
///     assert_eq!(config.ip, "127.0.0.1");
///     assert_eq!(config.port, None);
///     assert_eq!(config.keys.github, "git@github.com:Egg12138");
///     assert_eq!(config.keys.travis.as_ref().unwrap(), "...");
/// }
use argh::FromArgs;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
use std::process;
const  VERSION: &str = "0.0.1";
#[derive(Deserialize)]
struct Info {
    name: String,
    age:  Option<u8>,
}


#[derive(Deserialize)]
struct CmplxInfo {
    name: String,
    age: Option<u8>,
    books: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Partner {
    name: String,
    uid: u16,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}
#[derive(Deserialize, FromArgs, PartialEq, Debug)]
/// 由于 argh::description的要求，我们必须要提供注释……
struct Args {
    /// capture the output in terminal(false => no output)
    #[argh(switch)]
    nocapture: bool,
    #[argh(switch, short='v')]
    /// version, short 'v', 
    version: bool, 
}

fn main() {
    // 先看 rustlings 的逻辑
    let args: Args = argh::from_env();
    if args.nocapture && args.version {
        println!("-v{}", VERSION);
    }

    // 检查 info.toml 存在性
    if !Path::new("info.toml").exists() {
        println!("Error: Cannot find the info file`info.toml` in the current directory. {} must be with  it.", 
            std::env::current_exe().unwrap().to_str().unwrap()         
    );
        process::exit(1);
    }

    //  检查某个prog的存在性
    if !prog_exists("rustc") {

    }

    let toml_raw = &fs::read_to_string("info.toml").unwrap();
    match toml::from_str::<Partner>(toml_raw) {
        Ok(parsed_info) => {
            //TODO: parse the commands inside `struct Partner`
            println!("{:#?}", parsed_info);
        },
        Err(e) => {
            println!("{:?}: Failed to parse to toml raw content!, ", e);
            process::exit(1);
        }
    } 
}


pub fn prog_exists(prog: &str) -> bool {

    process::Command::new(prog)
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)

}


#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};
    #[test]
    fn try_command() {
        let mut_dir = Command::new("exa")
            .stdout(Stdio::null())
            .spawn()
            .expect("Error");
        println!("{:?}", mut_dir);
    }

}