#![feature(format_args_nl)]

use clap::{Parser, Subcommand, ArgAction};
use std::path::PathBuf;
use std::io::{stdout, Write};
#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "A echo implemented by rust-lang",  long_about = None)]
#[command(author = "aydenegg")]
struct Echor {
    #[arg(action = ArgAction::Append)]
    src: Option<Vec<String>>,
    #[arg(short, long, value_name = "FILE")]    
    from: Option<PathBuf>,
    #[arg(short, long, action = ArgAction::Count)]
    cpus: u8,
}

fn main() {

    let cli = Echor::parse();
    println!("{:#?}", cli);
    let stdin_vec = cli.src.unwrap_or(Vec::from(["".to_owned()]));
    let stdin_str = stdin_vec.join(" ");
    let stdout = stdout();
    let mut handle = stdout.lock();
    handle.write_fmt(format_args_nl!("{}", stdin_str));
    /*if let Some(file) = cli.from.as_deref() {
        println!("From file {}", file.display());
    } 
    */
}