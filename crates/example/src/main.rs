use std::{
    env::args,
    io::{stdin, BufRead, BufReader, Read},
};

// use rw::generic::Reader;
use rw::{error_line, LineError};

fn process_lines<F, R>(f: F) -> impl Fn(R) -> Result<(), LineError>
where
    F: Fn(String) + Copy,
    R: Read,
{
    move |r| {
        BufReader::new(r)
            .lines()
            .enumerate()
            .map(|(no, rs)| rs.map(f).map_err(error_line(Some(no + 1))))
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }
}

fn args_argument(prefixes: &[&str]) -> Option<String> {
    args()
        .skip(1)
        .position(|s| prefixes.contains(&(s.as_str())))
        .and_then(|i| args().skip(1).nth(i + 1))
}

fn print(s: String) {
    println!("{}", s);
}

fn main() -> Result<(), LineError> {
    match args_argument(&["-f", "--file"]).map(std::fs::File::open) {
        Some(file_result) => file_result
            .map_err(error_line(None))
            .and_then(process_lines(print)),
        None => Ok(stdin()).and_then(process_lines(print)),
    }
}
