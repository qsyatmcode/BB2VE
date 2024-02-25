use serde_json;
use serde::{Deserialize, Serialize};
use std::{fs, io, process, time::Instant};
use std::iter::Inspect;

use bb2ve::types::{self, bb, ve};

fn main() {
    let args_vec: Vec<String> = std::env::args().collect();
    if args_vec.len() < 2 {
        println!("You need to specify the path to the file in the second argument");
        process::exit(1);
    }
    let json_data = match fs::read_to_string(&args_vec[1]) {
        Ok(r) => match serde_json::from_str::<bb::Obj>(&r) {
            Ok(j) => j,
            Err(e) => {
                println!("{}", e.to_string());
                process::exit(1);
            }
        },
        Err(e) => {
            println!("{}", e.to_string());
            process::exit(1);
        }
    };
    let mut privitive_count = 0;
    let now = Instant::now();

    if let Ok(out) = bb2ve::create_outobj(&mut privitive_count, String::from("TEXTURE_NAME"), json_data) {
        let elapsed_time = now.elapsed();
        println!("{} primitives converted in {:.2?}", privitive_count, elapsed_time);
        fs::write("output.json", serde_json::to_string_pretty(&out).unwrap()).expect("Error of writing");
    } else {
        process::exit(69);
    }
}

