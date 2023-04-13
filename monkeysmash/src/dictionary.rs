use sqlite::{State, Row};

use crate::parser::Word;

pub fn create_dictionary() {
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = "
        CREATE TABLE nouns (id INTEGER PRIMARY KEY, word TEXT);
        
        CREATE TABLE adjectives (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE adverbs (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE prepositions (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE transitive_verbs (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE plural_nouns (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE past_participle (id INTEGER PRIMARY KEY, word TEXT);
        
        CREATE TABLE present_participle (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE indicative_verbs (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE interjections (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE conjunctions (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE pronouns (id INTEGER PRIMARY KEY, word TEXT);

    ";

    connection.execute(query).unwrap();

}

pub fn fill_nouns(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO nouns (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_adjectives(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO adjectives (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_adverbs(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO adverbs (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_prepositions(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO prepositions (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_trans(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO transitive_verbs (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_plural(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO plural_nouns (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_past_participle(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO past_participle (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_present_participle(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO present_participle (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_indicative(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO indicative_verbs (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_interjections(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO interjections (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_conjunctions(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO conjunctions (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn fill_pronouns(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO pronouns (word) VALUES ('{}');
    ", word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn clear_dictionary() {
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = "
        DROP TABLE nouns;
        DROP TABLE adjectives;
        DROP TABLE adverbs;
        DROP TABLE conjunctions;
        DROP TABLE indicative_verbs;
        DROP TABLE interjections;
        DROP TABLE past_participle;
        DROP TABLE plural_nouns;
        DROP TABLE prepositions;
        DROP TABLE present_participle;
        DROP TABLE pronouns;
        DROP TABLE transitive_verbs;
    ";

    connection.execute(query).unwrap(); 

}

pub fn pick_word(table: &str) -> String{
    let mut random_word = String::from("");


    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("SELECT word FROM {} ORDER BY random() LIMIT 1;", table);
    let mut statement = connection.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next(){
        random_word = statement.read::<String, _>("word").unwrap();

 
    }

    String::from(random_word.to_lowercase() + " ")

}