use brainfuck2c;
use std::fs;
mod codegen;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Brainfuck2C")
        .arg(
            Arg::with_name("input")
                .short("i")
                .takes_value(true)
                .help("Sets the input BF file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .required(true)
                .help("Sets the output C file"),
        )
        .get_matches();

    let rom = fs::read_to_string(matches.value_of("input").unwrap()).unwrap();
    let parsed = brainfuck2c::brainfuck_parser(rom);
    println!("Parsed!");
    println!("Optimizing...");
    use std::time::Instant;
    let now = Instant::now();
    let optimized = brainfuck2c::brainfuck_optimizer(parsed);
    println!("Optimized!");
    let cg = codegen::brainfuck_codegen(&optimized);
    //println!("{}", cg);
    let elapsed = now.elapsed();
    println!(
        "Elapsed: {:.3} seconds",
        elapsed.as_millis() as f64 / 1000.0
    );
    fs::write(matches.value_of("output").unwrap(), cg).expect("Unable to write file");
}
