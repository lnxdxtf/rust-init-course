#![allow(dead_code)]
#![allow(non_snake_case)]

use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    pattern: String,
    replacement: String,
    inputFile: String,
    outputFile: String,
}

fn print_debug<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}
fn print_colorized<T: std::fmt::Display>(value: T, ln: bool) {
    if ln {
        eprintln!("{}", value);
    } else {
        eprint!("{}", value);
    }
}

fn printHelp() {
    print_colorized("Find and replace - ".green(), false);
    print_colorized("replace a string with a new string", true);
}

fn getArguments() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        printHelp();
        eprintln!(
            "{} wrong number of arguments. Expected 4, got {}",
            "Error".red().bold(),
            args.len().to_string().blue()
        );
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replacement: args[1].clone(),
        inputFile: args[2].clone(),
        outputFile: args[3].clone(),
    }
}

fn readAndWrite(arg: &Arguments) {
    let data = match fs::read_to_string(&arg.inputFile) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} failed to read from file {} \n {:?}",
                "Error".red().bold(),
                &arg.inputFile,
                e
            );
            std::process::exit(1);
        }
    };

    let replaceData = match replace(&arg.pattern, &arg.replacement, &data) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{} failed to replace text \n {:?}", "Error".red().bold(), e);
            std::process::exit(1);  
        }
    };

    match fs::write(&arg.outputFile, &replaceData) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write from file {} \n {:?}",
                "Error".red().bold(),
                &arg.inputFile,
                e
            );
            std::process::exit(1);
        }
    };
}

fn replace(target: &str, replacement: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, replacement).to_string())
}

pub fn run() {
    let args = getArguments();
    readAndWrite(&args);
}
