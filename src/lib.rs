use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type RcatResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_line: bool,
}

pub fn run(config: Config) -> RcatResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprint!("Failed to open {}:{}", filename, e),
            Ok(file) => {
                let mut number_non_blank = 0;
                for (line_num, lines) in file.lines().enumerate() {
                    let line = lines?;
                    if config.number_lines {
                        println!("{:>6} {}", line_num + 1, line);
                    } else if config.number_nonblank_line {
                        if line.is_empty() {
                            println!();
                        } else {
                            number_non_blank += 1;
                            println!("{:>6} {}", number_non_blank, line);
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

pub fn get_args() -> RcatResult<Config> {
    let matches = App::new("rcat")
        .version("0.1.0")
        .author("Zafar Hussain Luni <zafar.luni@gmail.com>")
        .about("A simple implementation of cat command in Rust")
        .arg(
            Arg::with_name("files")
                .short("f")
                .long("file")
                .value_name("Files")
                .multiple(true)
                .default_value("-")
                .index(1)
                .help("Input file(s)"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(false)
                .help("Number lines")
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number_nonblank")
                .takes_value(false)
                .help("Number non-blank lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_line: matches.is_present("number_nonblank"),
    })
}

fn open(filename: &str) -> RcatResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
