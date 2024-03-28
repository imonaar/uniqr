use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(about = "report or filter out repeated lines in a file", long_about=None)]
#[command(author = "Kevin Monari")]
#[command(version = "0.1.0")]
pub struct Config {
    #[arg(default_value = "-", value_name = "IN_FILE", help = "Input file")]
    in_file: String,
    #[arg(value_name = "OUT_FILE", help = "Output file")]
    out_file: Option<String>,
    #[arg(
        short,
        long,
        help = "count of the number of times the line occurred in the input"
    )]
    #[arg(value_name = "COUNT", help = "show count")]
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match config.out_file {
        Some(file) => Box::new(File::create(file)?),
        _ => Box::new(stdout()),
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if previous.trim_end() != line.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &previous)?;
    Ok(())
}
