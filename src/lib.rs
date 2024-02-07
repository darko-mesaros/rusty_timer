use std::fs;
use std::io::prelude::*;

use chrono::{Local, Duration};

#[derive(Debug)]
pub struct Config {
    // amount of minutes
    pub interval: i64,
    pub template: String,
    pub filename: String,
    pub font: String,
    // theme
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // error handling
        if args.len() < 2 {
            return Err("not enough arguments")
        }
        let interval = match args[1].parse() {
            Ok(i) =>  {
                if i > 200 {
                    eprintln!("Sorry, the interval requested is too big, try a number smaller than 200");
                    std::process::exit(1);
                } else {
                    i
                }
            },
            Err(e) => {
                panic!("Could not parse a number from the argument: {}",e);
            }
        };
        let config = Config {
            interval,
            template: String::from("template.html"),
            filename: String::from("timer.html"),
            font: String::from("Anonymous Pro")
        };

        // return the values
        Ok(config)
    }
}

pub fn load_template(filename: &String) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}

pub fn generate_html(filename: &String, interval: i64, font: String) -> Result<String, Box<dyn std::error::Error>> {
    // TODO HANDLE ERRORS
    
    let template = match load_template(filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR - Sorry, there was an error reading the file: {}", e);
            std::process::exit(1);
        }
    };

    // generate the time stamp
    let current_time = Local::now();
    let modified_time = current_time + Duration::minutes(interval);
    //let formatted_time = current_time.format("%b %-e, %Y %H:%M:%S").to_string();

    let generate = template.replace("{{interval}}", modified_time.to_string().as_str())
                            .replace("{{fontfamily}}", font.to_string().as_str());

    Ok(generate)
}

pub fn write_timer(genf: String, filename: &String) -> Result<(), Box<dyn std::error::Error>> {
    // TODO HANDLE ERRORS

    let mut new_file = fs::File::create(filename)?;
    new_file.write_all(genf.as_bytes())?;
    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // TODO HANDLE ERRORS - OR NOT
    // CLEAN THIS UP
    
    //let genf = match generate_html(&config.template, config.interval, config.font) {
    //    Ok(f) => f,
    //    Err(e) => {
    //        eprintln!("There was an issue generating the HTML file: {}",e);
    //        std::process::exit(1);
    //    }
    //};

    //match write_timer(genf, &config.filename) {
    //    Ok(w) => w,
    //    Err(e) => {
    //        eprintln!("There was an issue writing the file: {}",e );
    //        std::process::exit(1);
    //    }
    //};

    generate_html(&config.template, config.interval, config.font)
        .and_then(|genf|write_timer(genf, &config.filename))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config_valid_args() {
        let args = vec![
            String::from("program"),
            String::from("10"),
            String::from("template.html"),
            String::from("timer.html"),
            String::from("Anonymous Pro"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.interval, 10);
        assert_eq!(config.template, "template.html");
        assert_eq!(config.filename, "timer.html");
        assert_eq!(config.font, "Anonymous Pro");
    }

    #[test]
    fn test_new_config_invalid_args() {
        let args = vec![String::from("program")];
        let result = Config::new(&args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "not enough arguments");
    }

    // TODO - Figure out how to test a match expression
    // so it does not exit the process
    //#[test]
    //fn test_new_config_invalid_interval() {
    //    let args = vec![String::from("program"), String::from("201")];
    //    let result = Config::new(&args);
    //    assert!(result.is_err());
    //    assert_eq!(
    //        result.unwrap_err(),
    //        "Sorry, the interval requested is too big, try a number smaller than 200"
    //    );
    //}
}
