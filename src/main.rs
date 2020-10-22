extern crate exec;

use std::{
    env::args,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process::exit
};

fn args_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let mut argv: Vec<String> = args().skip(1).collect();
    if argv.len() < 1 {
        println!("Must specify command to execute");
        exit(1);
    }
    if argv.len() < 2 {
        println!("No arguments file found");
        exit(1);
    }

    // Args are:
    println!("Args: {:?}", argv);
                                                  // [exec, args, file]
    let arg_file = argv.pop().expect("No arg file specified");
    println!("Args: {:?}", argv);
    let mut file_argv = args_from_file(arg_file); // [exec, args]
    println!("Args: {:?}", argv);
    argv.append(&mut file_argv);                  // [exec, args, file_args...]


    println!("Args: {:?}", argv);

    // Exec the specified program.  If all goes well, this will never
    // return.  If it does return, it will always retun an error.
    let err = exec::execvp(&argv[0],&argv);
    println!("Error: {}", err);
    exit(1);
}
