use clap::{Arg, Parser, ArgAction};
use std::error::Error;
use std::fs::{File, read};
use std::io::{self, BufRead, BufReader, Read, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

impl Default for  Config {
    fn default() -> Self {
        
        #[cfg(target_os = "linux")]
        let empty_file = "/usr/share/man/man1/cat.1.gz"; // cat  man 
        #[cfg(target_os = "windows")]
        let empty_file = "C:\\User\\cat_empty.txt";
        Config {
            files: [empty_file.to_owned()].to_vec(),
            number_lines: false,
            number_nonblank_lines: false,
        }
    }

}


#[derive(Debug,Parser)]
#[command(version)]
#[command(about = "A clone of cat by Rust", long_about = "This catr contains arguments only -b and -n.")]
#[command(author = "Aydenegg")]
pub struct Catr {
    /// files to be catr
    files: Vec<String>, // required
    /// show the line number
    #[arg(short, long, value_name = "LINENUM")]
    nonempty_linenums: bool,
    /// show only nonempty line
    #[arg(short, long, value_name = "LINENUM_NONEMPTY")]
    empty_linenums: bool,
}

pub fn get_args() -> MyResult<Catr> {

    let config = Catr::parse();
    Ok(config)

}

/* --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

*/
#[inline]
pub fn run(config: Catr) -> MyResult<()> {

    let files_vec = config.files;
    let mut files_iter = files_vec.iter().filter(|s| !s.is_empty()); 
    //while let Some(&fpath) =  files_iter.next() {
    for fpath in files_iter {
        match open(fpath) {
            Ok(f) => {
                let mut nonempty_line_num = 0u32;
                for (file_line_num, line_result) in f.lines().enumerate() {
                    let line = line_result?;
                    // nonempty_linenums holds the highest priority.
                    if config.nonempty_linenums {
                        println!("{:6}\t{}", file_line_num + 1, line);
                    } else if config.empty_linenums {
                        if !line.is_empty() {
                            nonempty_line_num += 1;
                            println!("{:6}\t{}", nonempty_line_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{line}");
                    } 
                }
            },
            Err(err) => {
                let es = format!("{} >> error.", fpath);
                eprintln!("{}",es);
            } 
        }
    }


    Ok(())
}
/*  --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
*/


// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
