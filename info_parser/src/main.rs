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
/// 

use argh::FromArgs;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process;

const VERSION: &str = "0.0.1";
#[cfg(target_os = "linux")]
const CONFIG_PATH: &str = "~/.local/.Accountparser"; 
#[cfg(target_os = "windows")]
const CONFIG_PATH: &str = "C:\\bin\\.Accountparser"; 


#[derive(Deserialize)]
struct Account {
    name: String,
    age:  Option<u8>,
}
           

#[derive(Deserialize, Debug)]
#[serde(rename(serialize = "let"))]
pub struct CmplxAccount {
    #[serde(rename = "nm")]
    name: String,
    age: Option<u8>,
    books: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename = "member")]
pub struct Partner {
    #[serde(rename = "let")]
    name: String,
    uid: u32,
}
#[derive(Deserialize, Serialize)]
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

    // 检查 Account.toml 存在性
    if !Path::new("Account.toml").exists() {
        println!("Error: Cannot find the Account file`Account.toml` in the current directory. {} must be with  it.", 
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
    let toml_raw = &fs::read_to_string("Account.toml").unwrap();
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
                    .unwrap_or_else(|err| {
                        match err.kind() {
                            std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                            _ => std::process::exit(1),
                        }
                    });
           }); 
           let toml_content = toml::to_string_pretty(&partners.partners).unwrap();
           println!("{}", toml_content);
           
        
        },
        Err(err) => {
            let err = err.to_string();
            println!("Failed to parse the toml raw content!, {}", err);
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
    use serde::Deserialize;
    use super::*;
    use std::io::ErrorKind;
    enum Server {
        Github,
        Gitee,
        Gitlab,
        Svn,
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct Repo {
        ip: String,
        port: Option<u16>,
        config: Keys,
    }
    impl std::default::Default for  Repo {
        fn default() -> Self {
            Repo {
                ip: "192.168.2.1".to_owned(),
                port: Some(8080),
                config: Keys { server: "git@github.com".to_owned(), token: Some("no-ssh-key".to_owned()) }
            }
        }
    }
    #[derive(Deserialize, Serialize, Debug)]
    struct Keys {
       // server: Server,
        server: String,
        token: Option<String>,
    }

    fn try_toml_export() {
        #[warn(clippy::useless_format)] 
        let raw_toml = &fs::read_to_string("testing.toml").unwrap();
        let repo_config = toml::from_str::<Repo>(raw_toml).unwrap_or_default();
        assert_eq!(repo_config.ip, "127.0.0.1".to_owned());
        assert_eq!(repo_config.port, Some(65535));
        assert_eq!(repo_config.config.server, "git@github.com".to_owned());
        let export_toml = toml::to_string_pretty(&repo_config).unwrap();
        // don't need to use try_exists instead of exists !
        let configpath = Path::new(CONFIG_PATH);
        if let Err(err) = fs::write(configpath, export_toml)  {
            if err.kind() == ErrorKind::PermissionDenied {
                println!("PermissionDenied: Fialed to write configuration into {}", CONFIG_PATH);
            }
        } 
    }

    fn try_command() {
        let mut_dir = Command::new("exa")
            .stdout(Stdio::null())
            .spawn()
            .expect("Error");
        println!("{:?}", mut_dir);
    }

    fn try_parse_tomlvalues() {

        #[derive(Deserialize, Debug)]
        struct CmplxAccounts {
            cmplxaccount: Vec<CmplxAccount>
        }
        let default = "info.toml".to_owned();
        println!("--------{:<10}-------------", "TRY TOML PRIMARY TYPES");
        let default = "Account.toml".to_owned();
        let toml_raw = &fs::read_to_string("testing.toml").unwrap_or(default);
        match toml::from_str::<toml::Value>(toml_raw) {
         Ok(toml_str) => {
            println!("* === {} ===", toml_str);
        },
        Err(err) => {
            let err = err.to_string();
            println!("Failed to parse the raw string in the toml file, {}", err);
            process::exit(1);
        },
    }

        match toml::from_str::<CmplxInfos>(toml_raw) {
        println!("-------TRY COMPLEX Account------------");
        match toml::from_str::<CmplxAccounts>(toml_raw) {
            Ok(cmplx_str) => {
                println!("Works");
            },

            Err(err) => {
                let err = err.to_string();
                println!("Failed to parse the toml raw content!, {}", err);
            },
        }
    }

    
    fn try_parse_many() {

        println!("--------{:<10}-------------", "TRY parse complicated toml");
        let toml_raw = &fs::read_to_string("Account.toml").unwrap();
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
                    .unwrap_or_else(|err| {
                        match err.kind() {
                            std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                            _ => process::exit(1),
                        }
                    });
           }) 
        },
         Err(err) => {
            let err = err.to_string();
            println!("Failed to parse the toml raw content!, {}", err);
            process::exit(1);
            
        }
    } 

    }

    #[test]
    fn open_stdin() {
        


    }
    fn serde_enum_representations() {
        use serde_json;
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        //#[serde(tag = "MessageType", content = "item")]
        enum Message {
            #[serde(rename = "RequestInfo")]
            Request { id: String, method: String},
            #[serde(rename = "ResponseInfo")]
            Response { id: String, result: serde_json::Value },
        }
       let msg = Message::Request { id: "Egg12138".to_string(), method: "CNN".to_string()};
       let serialized_msg = serde_json::to_string_pretty(&msg).unwrap(); 
       println!("serialized => {}", serialized_msg);
       let de_msg = serde_json::from_str::<Message>(&serialized_msg).unwrap();
       println!("deserialized => {:#?}", de_msg);
       assert_eq!(de_msg, msg);
       if let Err(err) = fs::write(Path::new("./testjson.json"), &serialized_msg) {
        	let err = err.to_string();
            println!("{}", err);
       }
       
    }

}
