use std::io::{Error, ErrorKind};

mod jsonx;
use getopts::{Matches, Options};

fn main() {
    match do_run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(99);
        }
    };
}

fn do_run() -> Result<(), String> {
    //if execute "jsonx -h"
    //output: ["jsonx", "-h"]
    let args: Vec<String> = std::env::args().collect();

    //build option flag command
    let mut opts = Options::new();

    opts.optflag("h", "help", "Help you how to use this tool");
    opts.optflag("m", "minimize", "Minimize file type json into one liner");

    //parse user input on type
    //args[1..] => ["-h"]
    let flagArgs: Matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    //when has flag "-h"
    //then print help and exit
    if flagArgs.opt_present("h") {
        let program = args[0].clone();
        do_print_help(&program, &opts);
        return Ok(());
    }

    //read file i/o
    let mut input: Box<dyn std::io::Read> = Box::new(std::io::stdin());
    let mut output: Box<dyn std::io::Write> = Box::new(std::io::stdout());

    // let result: Result<(), std::io::Error> = Err(std::io::Error::new(
    //     ErrorKind::BrokenPipe,
    //     "Dummy error broken pipe",
    // ));

    let result = if flagArgs.opt_present("m") {
        let mut fm = jsonx::Formatter::minimizer();
        fm.do_format(&mut input, &mut output);
        Ok(())
    } else {
        Err(std::io::Error::new(
            ErrorKind::BrokenPipe,
            "Dummy rror broken pipe",
        ))
    };

    match result {
        Err(e) if e.kind() == ErrorKind::BrokenPipe => Err(e.to_string()),
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(()),
    }
}

fn do_print_help(program_name: &str, opts: &Options) {
    let desc = "Jsonx is a JSON transformer. Provide pretty-printing and minimizing of JSON-encode UTF-8 data.";

    let example_usage = "
    Minimize file into one liner:
        jsonx -m <file_name_input.json> file_name_out.json
    ";

    let brief = format!("Usage: {} [options]\n\n{}", program_name, desc);
    print!("{}", opts.usage(&brief));
    println!("{}", example_usage);
}
