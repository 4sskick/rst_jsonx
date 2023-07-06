use std::{
    io::{Error, ErrorKind},
    result,
};

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

    let result: Result<(), std::io::Error> = Err(std::io::Error::new(
        ErrorKind::BrokenPipe,
        "Dummy error broken pipe",
    ));
    match result {
        Err(e) if e.kind() == ErrorKind::BrokenPipe => Err(e.to_string()),
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(()),
    }
}
