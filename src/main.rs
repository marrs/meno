use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Show each annotation on a single line
    #[clap(short, long, action)]
    line: bool,

    file: String,
}

fn for_file_lines<F: FnMut(String)>(filename: String, mut func: F) {
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(x) = line {
                func(x)
            }
        }
        println!("");
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let filename = args.file;
    let mut is_annotation = false;

    let re_mark = Regex::new(r"^(\d{0,4}:)?([0-5]?[0-9]:)?([0-5]?[0-9])(\.\d{3})?").unwrap();
    let re_body = Regex::new(r"^\s").unwrap();

    if args.line {
        for_file_lines(filename, |x| {
            if is_annotation && re_body.is_match(&x) {
                print!(" {}", x.trim());
            } else if re_mark.is_match(&x) {
                is_annotation = true;
                print!("\n{}", x);
            } else {
                is_annotation = false;
            }
        });
    } else {
        for_file_lines(filename, |x| {
            if is_annotation && re_body.is_match(&x) {
                println!("{}", x);
            } else if re_mark.is_match(&x) {
                is_annotation = true;
                println!("{}", x);
            } else {
                is_annotation = false;
            }
        });
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
