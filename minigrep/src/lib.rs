use std::{fs,error::Error};
mod tests;
// input config
pub struct Config {
    pub query:String,
    pub file_name:String,
}

impl Config {
    pub fn build(args:&[String])->Result<Config,&'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        Ok(Config { query, file_name})
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;
    for line in search(&config.query, &contents){
        println!("{line}");
    }
    Ok(())

}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            res.push(line)
        }
    }
    res
}