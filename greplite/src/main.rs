use clap::{App, Arg};
use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn fetch_args() -> (String, String, String) {
    let args = App::new("grep_lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("ctx_lines")
                .help("The number of lines to return for context")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("The file to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let term = args.value_of("pattern").unwrap().to_string();
    let file_path = args.value_of("input").unwrap_or("-").to_string();
    let ctx_lines_input = args.value_of("ctx_lines").unwrap().to_string();

    (term, file_path, ctx_lines_input)
}

fn process_lines(buffer: String, re: Regex, ctx_lines: usize) {
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, &str)>> = vec![];
    for (i, line) in buffer.lines().enumerate() {
        if let Some(_) = re.find(line) {
            tags.push(i);
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }
    if !tags.is_empty() {
        for (i, line) in buffer.lines().enumerate() {
            for (j, tag) in tags.iter().enumerate() {
                let lb = tag.saturating_sub(ctx_lines);
                let ub = tag + ctx_lines;

                if (i >= lb) && (i <= ub) {
                    let local_ctx = (i, line);
                    ctx[j].push(local_ctx);
                }
            }
        }
    }
    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_number = i + 1;
            println!("{}: {}", line_number.to_string().yellow(), line);
        }
    }
}

fn main() -> Result<(), String> {
    let (term, file_path, ctx_lines_input) = fetch_args();
    let ctx_lines: usize = ctx_lines_input.parse().unwrap();
    let re = Regex::new(term.as_str()).unwrap();

    if file_path == "-" {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut buffer = String::new();
        match reader.read_to_string(&mut buffer) {
            Ok(_) => {
                process_lines(buffer, re, ctx_lines);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    } else {
        let mut f = File::open(file_path).map_err(|e| format!("Failed to open file: {:?}", e))?;
        let mut buffer = String::new();
        match f.read_to_string(&mut buffer) {
            Ok(_) => {
                process_lines(buffer, re, ctx_lines);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}
