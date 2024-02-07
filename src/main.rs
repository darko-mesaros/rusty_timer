// TODO:
// - Load HTML content from a file -OK
// - Generate the current date and time and add
//   X minutes to it. -OK
// - Get user input for the how many minutes the  
//   timer should run. -OK
// - figure out how to return errors
use std::env;
use std::process;

use rusty_clock::Config;
use rusty_clock::run;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("ERROR - Problem parsing arugments: {}", err);
        process::exit(1);
    });

    run(config)
}
