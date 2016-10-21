extern crate clap;
use clap::{Arg, App};
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fs::File;

const BUFFER_SIZE: u32 = 4096;

struct Params {
    input_file:     String,
    output_file:    String,
    buffer_size:    usize,
}

fn main() {
    let p: Params = get_args();
    let fr = File::open(&p.input_file).unwrap();
    let fw = File::create(&p.output_file).unwrap();

    let mut buffer_r = BufReader::with_capacity(p.buffer_size, fr);
    let mut buffer_w = BufWriter::with_capacity(p.buffer_size, fw);
    let mut amount: usize = 0;
    loop {
        let length = {
            let content = buffer_r.fill_buf().unwrap();
            buffer_w.write(content).unwrap();
            buffer_w.flush().unwrap();

            content.len()
        };
        amount += length;
        if length == 0 { break }
        buffer_r.consume(length);
    }
    println!("Size: {}", amount);
}

fn get_args() -> Params {
    let matches = App::new("rcp")
                    .version("0.2.1")
                    .about("Copy Files - Rustlang based files copy.")
                    .arg(Arg::with_name("buffer")
                        .short("b")
                        .long("buffer")
                        .value_name("NUMBER")
                        .help("Sets a buffer size")
                        .takes_value(true))
                    .arg(Arg::with_name("INPUT")
                        .help("Set the input file to use")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("OUTPUT")
                        .help("Set the output file to use")
                        .required(true)
                        .index(2))
                    .get_matches();
    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();
    let buffer_size: usize = matches
        .value_of("buffer")
        .unwrap_or(&BUFFER_SIZE.to_string())
        .parse()
        .unwrap();
    Params {
        input_file: input_file.to_owned(),
        output_file: output_file.to_owned(),
        buffer_size: buffer_size,
    }
}
