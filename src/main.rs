use std::{
    env::args,
    io::{BufRead, BufReader},
};

use crate::reader::Reader;

mod reader;

fn main() -> Result<(), std::io::Error> {
    let reader = args()
        .skip(1)
        .next()
        .map(Reader::from_file)
        .unwrap_or(Ok(Reader::default()))?;
    let lines = BufReader::new(reader).lines();
    lines.map_while(Result::ok).for_each(|result| {
        println!("{}", result);
    });
    println!("Hello, world!");
    Ok(())
}
