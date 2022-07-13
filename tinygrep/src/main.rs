use std::{env, fs, process};
use tinygrep::Config;

fn main() {
    let args:Vec<String>=env::args().collect();
    let config=Config::new(&args).unwrap_or_else(|err|{
        eprintln!("problem parsing args:{}",err);
        process::exit(1);
    });
    if let Err(e)=tinygrep::run(config) {
        eprintln!("application run error:{}",e);
        process::exit(1);
    }

    // println!("Read contents:\n{}",contents)

}
