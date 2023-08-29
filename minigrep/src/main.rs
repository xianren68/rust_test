use std::{env,process};
use minigrep::{Config,run};
fn main() {
    let args:Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parseing arguments: {}",err);
        // exit process
        process::exit(1);
    });
    println!("Searching for {}",config.query);
    println!("In file {}",config.file_name);
    if let Err(e) = run(config){
        println!("Application error:{}",e);
        process::exit(1)
    }
}
