use std::{env,process};
use minigrep::{Config,run};
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parseing arguments: {}",err);
        // exit process
        process::exit(1);
    });
    eprintln!("Searching for {}",config.query);
    eprintln!("In file {}",config.file_name);
    if let Err(e) = run(config){
        eprintln!("Application error:{}",e);
        process::exit(1)
    }
}
