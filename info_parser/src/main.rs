#![allow(unused_variables, dead_code)]
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
use std::fmt::format;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process;
const  VERSION: &str = "0.0.1";



#[derive(Deserialize)]
struct Info {
    name: String,
    age:  Option<u8>,
}


#[derive(Deserialize, Debug)]
pub struct CmplxInfo {
    name: String,
    age: Option<u8>,
    books: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Partner {
    name: String,
    uid: u32,
}
#[derive(Deserialize)]
pub struct PartnerList {
    pub partners: Vec<Partner>
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

#[warn(clippy::needless_doctest_main)]
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
        println!("Failed to find `rustc`");        
        std::process::exit(0x01);
    }

    if !prog_exists("exa") {
        println!("Failed to find `exa`");        
        std::process::exit(0x01);
    }
    // fetch the raw content of the toml file
    let toml_raw = &fs::read_to_string("info.toml").unwrap();
    match toml::from_str::<PartnerList>(toml_raw) {
        Ok(partners) => {
            //TODO: parse the commands inside `struct Partner`
           partners.partners.iter().for_each(|p| {
                let uid = format!("[{}]", p.uid);
                let name = p.name.trim();
                let namestr = format!("'{}'", name);
                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                let outputs = format!("{namestr}:{uid}\n");
                handle.write_all(outputs.as_bytes())
                    .unwrap_or_else(|e| {
                        match e.kind() {
                            std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                            _ => std::process::exit(1),
                        }
                    });
           }) 
        },
        Err(e) => {
            let e = e.to_string();
            println!("Failed to parse the toml raw content!, {}", e);
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
    use super::*;

    fn try_command() {
        let mut_dir = Command::new("exa")
            .stdout(Stdio::null())
            .spawn()
            .expect("Error");
        println!("{:?}", mut_dir);
    }

    #[test]
    fn try_parse_tomlvalues() {

        #[derive(Deserialize, Debug)]
        struct CmplxInfos {
            cmplxinfos: Vec<CmplxInfo>
        }
        println!("--------{:<10}-------------", "TRY TOML PRIMARY TYPES");
        let default = "info.toml".to_owned();
        let toml_raw = &fs::read_to_string("testing.toml").unwrap_or(default);
        match toml::from_str::<toml::Value>(toml_raw) {
         Ok(toml_str) => {
            println!("* === {} ===", toml_str);
        },
        Err(e) => {
            let e = e.to_string();
            println!("Failed to parse the raw string in the toml file, {}", e);
            process::exit(1);
        },
    }

        println!("-------TRY COMPLEX INFO------------");
        match toml::from_str::<CmplxInfos>(toml_raw) {
            Ok(cmplx_str) => {
                println!("Works");
            },
            Err(e) => {
                let e = e.to_string();
                println!("Failed to parse the toml raw content!, {}", e);
            },
        }
    }

    fn try_parse_many() {

        println!("--------{:<10}-------------", "TRY parse complicated toml");
        let toml_raw = &fs::read_to_string("info.toml").unwrap();
        match toml::from_str::<PartnerList>(toml_raw) {
         Ok(partners) => {
            //TODO: parse the commands inside `struct Partner`
           partners.partners.iter().for_each(|p| {
                let uid = format!("[{}]", p.uid);
                let name = p.name.trim();
                let namestr = format!("'{}'", name);
                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                let outputs = format!("{namestr}:{uid}\n");
                handle.write_all(outputs.as_bytes())
                    .unwrap_or_else(|e| {
                        match e.kind() {
                            std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                            _ => process::exit(1),
                        }
                    });
           }) 
        },
         Err(e) => {
            let e = e.to_string();
            println!("Failed to parse the toml raw content!, {}", e);
            process::exit(1);
            
        }
    } 

    }


}