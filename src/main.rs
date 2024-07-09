extern crate clap;
extern crate log;

use clap::Parser;
use log::{debug, info};
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    env_logger::init();
    let mut line_number: u64 = 1;
    let args = Cli::parse();
    let file_not_found_msg = format!("File {:?} not found!", &args.path);
    let fd = std::fs::File::open(&args.path).expect(&file_not_found_msg);
    let mut reader = BufReader::new(fd);
    let mut line: String = String::new();

    debug!(
        "Looking for match of {:?} in the file {:?}",
        &args.pattern, &args.path
    );
    loop {
        let bytes: usize = reader.read_line(&mut line).unwrap();

        if bytes > 0 {
            // if om_grrs::find_matches(&line, &args.pattern) {
            //     info!(
            //         "I found the pattern {:?} on line {:?}",
            //         &args.pattern, line_number
            //     );
            // }
            let found = line.contains(&args.pattern);
            // println!("{:?}", found);
            if found {
                info!(
                    "I found the pattern {:?} on line {:?}",
                    &args.pattern, line_number
                );
            }

            line_number += 1;

            // I don't want to append to the previous line so I clear the
            // contents of the line buffer.
            line.clear();
        } else {
            debug!("End of file reached!");
            break;
        }
    }
}
