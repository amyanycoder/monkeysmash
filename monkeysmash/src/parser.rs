use csv::Reader;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

//Word(word, part of speech)
pub struct Word(&'static str, &'static str);


pub fn parse_file() -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path("src/words.csv")?;

    for line in reader.records(){
        let mut value = line?;

        let mut v: Vec<&str> = value.split(',').collect();
        println!("{:?}", value);

    }


    Ok(())

}