use std::error::Error;
use std::fmt;
use std::string::String;
use crate::dictionary::*;


//Word(word, part of speech)
pub struct Word(pub String, pub String);

//describes how the word tuple should be printed.
impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0}, {1})", self.0, self.1)

    }

}

//runs through csv file, passing along each parsed word to handle_filing() to determine insertion in database.
pub fn parse_file() -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path("src/words.csv")?;

    //iterates through file, sending word to handle_filing()
    for line in reader.records(){
        let value = line?;

        let current_word = Word(String::from(value.get(0).unwrap()), String::from(value.get(1).unwrap()));

        handle_filing(current_word);
    }

    Ok(())

}

//inserts word into database if it meets specific criteria.
fn handle_filing(current_word: Word){
    //logic tree determining word table based on part of speech
    //eliminates words with more than 10 characters
    //forbids words that do not contain these specific part of speech indicators.
    if current_word.0.chars().count() < 10
    && (current_word.1.contains("pl.")
        || current_word.1.contains("v. i.") 
        || current_word.1.contains("n.")
        || current_word.1.contains("a.")
        || current_word.1.contains("adv.")
        || current_word.1.contains("v. t.")
        || current_word.1.contains("p. p.")
        || current_word.1.contains("pr. p.")
        || current_word.1.contains("prep.")
        || current_word.1.contains("interj.")
        || current_word.1.contains("conj."))    {

        insert_word(current_word);

    }

}
