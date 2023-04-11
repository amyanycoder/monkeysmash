use std::error::Error;
use std::fmt;
use std::string::String;
use crate::dictionary::*;


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

        handle_filing(current_word);
    }

    Ok(())

}

fn handle_filing(current_word: Word){
    println!("{:}", current_word);

    //

    //logic tree determining word table based on part of speech
    //eliminates words with more than 10 characters
    if current_word.1.chars().count() > 10{

    }
    else if current_word.1.contains("pl."){
        fill_plural(current_word);
    }
    else if current_word.1.contains("v. i."){
        fill_indicative(current_word);
    }
    else if current_word.1.contains("n."){
        fill_nouns(current_word);
    }
    else if current_word.1.contains("a."){
        fill_adjectives(current_word);
    }
    else if current_word.1.contains("adv."){
        fill_adverbs(current_word);
    }
    else if current_word.1.contains("v. t."){
        fill_trans(current_word);
    }
    else if current_word.1.contains("p. p."){
        fill_past_participle(current_word);
    }
    else if current_word.1.contains("pr. p."){
        fill_present_participle(current_word);
    }
    else if current_word.1.contains("prep."){
        fill_prepositions(current_word);
    }
    else if current_word.1.contains("interj."){
        fill_interjections(current_word);
    }
    else if current_word.1.contains("conj."){
        fill_conjunctions(current_word);
    }

    
}
