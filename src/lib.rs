use clap::{App, Arg};
use std::error::Error;

type RcatResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_line: bool,
}

pub fn run() -> RcatResult<()> {
    let config = get_args()?;
    for filename in config.files {
        println!("{}", filename);
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
