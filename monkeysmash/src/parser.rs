use std::error::Error;
use std::fmt;

use crate::dictionary::fill_dictionary;


//Word(word, part of speech)
pub struct Word(pub String, pub String);

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0}, {1})", self.0, self.1)

    }

}


pub fn parse_file() -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path("src/words.csv")?;

    for line in reader.records(){
        let value = line?;

        let current_word = Word(String::from(value.get(0).unwrap()), String::from(value.get(1).unwrap()));

        //println!("{:}", current_word);

        fill_dictionary(current_word);
        

    }

    Ok(())

}