use std::{fs,error::Error,env};
mod tests;
// input config
pub struct Config {
    pub query:String,
    pub file_name:String,
    pub ignore_case:bool,
}

impl Config {
    pub fn build(mut args:impl Iterator<Item=String>)->Result<Config,&'static str>{
        args.next();
        let query = match args.next(){
            Some(q)=>q,
            None=>return Err("Didn't get a query string")
        };
        let file_name = match args.next(){
            Some(f)=>f,
            None=>return Err("Didn't get a file_name string")
        };
        let ignore_case = match env::var("IGNORE_CASE") {
            Err(_) => match args.next() {
                Some(flag) => flag != "0",
                None => false,
            },
            Ok(_) => true,
        };
        Ok(Config { query, file_name,ignore_case})
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;
    let lines = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };
    for line in lines{
        println!("{line}");
    }
    Ok(())

}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

// ignore case
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
    
}