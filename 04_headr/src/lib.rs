use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(version)]
#[command(about = "A clone of head by Rust", long_about = "No more description...")]
#[command(author="Aydenegg")]
pub struct Headr {
    /// files passed to headr
    files: Vec<String>,
    #[arg(short = 'n', long, default_value = "10",  value_name = "LENS_NUM")]
    lines: isize,
    /// How many first bytes to be read into the school
    #[arg(short = 'c', long, value_name = "FIRST_BYTES")]
    bytes: Option<usize>,
}

impl Default for Headr {
    fn default() -> Self{
        #[cfg(target_os = "linux")]
        let fs = "/usr/share/man/man1/head.1.gz".to_owned();
        #[cfg(target_os = "linux")]
        let fs = "C:\\User\\headr_empty.txt".to_owned();
        Headr {
            files: [fs].to_vec(),
            lines: -1,
            bytes: Some(1),
        }
    }
}
// --------------------------------------------------
pub fn get_args() -> MyResult<Headr> {
    let headrconfig = Headr::parse(); 
    Ok(headrconfig)
}

pub fn run(headr: Headr) -> MyResult<()> {

    let num_files = headr.files.len();

    for (fileindex, fname) in headr.files.iter().enumerate() {
        match open(fname) {
            Err(err) => {
                eprintln!("open{}: {}", fname, err);
            }, 
            Ok(mut f) => {
                println!("{}{}", if headr.files.len() > 0 { "\n"} else { "" } , &fname);
                if let Some(num_bytes) = headr.bytes {
                    let mut handle = f.take(num_bytes as u64); 
                    let mut buffer = [0; 10];
                    let read_bytes_num = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..read_bytes_num]));
                } else {
                    let mut line_string = String::new();
                    for _ in 0..headr.lines {
                        let read_bytes_num = f.read_line(&mut line_string)?;
                        print!("{}", line_string);
                        line_string.clear();
                    }
                }
            }
        }
    }

    Ok(())

}



fn open(fname: &str) -> MyResult<Box<dyn BufRead>> {
   match fname {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(fname)?))),
   } 
}


// --------------------------------------------------
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

// --------------------------------------------------
#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
