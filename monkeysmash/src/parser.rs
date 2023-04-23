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

        //current_word is defined by the word, followed by the part of speech.
        let current_word = Word(String::from(value.get(1).unwrap()), String::from(value.get(2).unwrap()));

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
    && (current_word.1.contains("Modal verb")
        || current_word.1.contains("Noun") 
        || current_word.1.contains("Adjective")
        || current_word.1.contains("Adverb")
        || current_word.1.contains("Verb")
        || current_word.1.contains("Preposition")
        || current_word.1.contains("Exclamation")
        || current_word.1.contains("Conjunction")
        || current_word.1.contains("Number"))    {

        insert_word(current_word);

    }

}
