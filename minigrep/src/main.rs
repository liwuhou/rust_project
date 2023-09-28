use std::env;
use std::fs;
use std::process;

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl<'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            Err("not enough parameters")
        } else {
            Ok(Config {
                query: &args[1],
                file_path: &args[2],
            })
        }
    }
}

fn run(config: &Config) {
    let context =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file.");

    println!("With text:\n{context}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(&config)
}
