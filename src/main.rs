use std::{
    io::{Error, ErrorKind},
    result,
};

use getopts::{Matches, Options};

fn main() {
    match do_run() {
        Ok(_) => {
            println!("ok!!")
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(99);
        }
    };
}

fn do_run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    //build option flag command
    let mut opts = Options::new();

    opts.optflag("h", "help", "Help you how to use this tool");

    //parse user input on type
    let flagArgs: Matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    if flagArgs.opt_present("h"){
        let program = args[0].clone();
        do_print_help(&program, &opts);
        return Ok(());
    }

    let result: Result<(), std::io::Error> = Err(std::io::Error::new(
        ErrorKind::BrokenPipe,
        "Dummy error broken pipe",
    ));

        // let result:Result<(), std::io::Error> = 

    match result {
        Err(e) if e.kind() == ErrorKind::BrokenPipe => Err(e.to_string()),
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(()),
    }
}

fn do_print_help(program_name: &str, opts: &Options){
    let desc = "description";

    println!("this should be help document for usage {}",program_name);

    let brief = format!("Usage: {} [options]\n\n{}", program_name, desc);
    print!("{}", opts.usage(&brief));
}