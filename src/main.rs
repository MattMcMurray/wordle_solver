use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    process,
    path::Path
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let lines = read_lines_from_file(Path::new(&config.wordfile));

    println!("Read {} words from {}", lines.len(), config.wordfile)
}

struct Config {
    wordfile: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let wordfile = args[1].clone();

        Ok(Config { wordfile })
    }
}

fn read_lines_from_file(filename: &Path) -> Vec<String> {
    let file = File::open(&filename)
        .unwrap_or_else(|_| panic!("No such file"));

    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
