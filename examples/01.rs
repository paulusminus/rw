use std::{
    env::args,
    io::{BufRead, BufReader, Read},
};

use rw::{error_line, LineError, Reader};

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

fn parse_args() -> Result<Reader<'static>, LineError> {
    args_argument(&["-f", "--file"])
        .map(Reader::from_file)
        .unwrap_or(Ok(Reader::default()))
        .map_err(error_line(None))
}

fn main() -> Result<(), LineError> {
    parse_args().and_then(process_lines(|s| println!("{}", s)))
}
