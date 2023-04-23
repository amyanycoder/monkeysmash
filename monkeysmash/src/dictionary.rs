use sqlite::{State};

use crate::parser::Word;

//builds the database
pub fn build_database() {
    //clears all tables
    clear_dictionary();
    create_dictionary();

    //fills database with words
    if let Err(e) = crate::parser::parse_file(){
        eprintln!("{}", e);
    }
    //adds words not easily extracted from the csv
    insert_pronouns();
    insert_articles();
}

//returns a single random word
pub fn pick_word(table: &str) -> String{
    let mut random_word = String::from("");

    let connection = sqlite::open("dictionary.db").unwrap();
    
    //pulls word from database and assigns it to rust variable
    let query = format!("SELECT word FROM {} ORDER BY random() LIMIT 1;", table);
    let mut statement = connection.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next(){
        random_word = statement.read::<String, _>("word").unwrap();

    }

    String::from(random_word + " ")

}

//creates database and tables
fn create_dictionary() {
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = "
        CREATE TABLE nouns (id INTEGER PRIMARY KEY, word TEXT);
        
        CREATE TABLE adjectives (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE adverbs (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE prepositions (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE verbs (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE exclamations (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE conjunctions (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE pronouns (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE modal_verbs (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE articles (id INTEGER PRIMARY KEY, word TEXT);

    ";

    connection.execute(query).unwrap();

}

//inserts word from csv based on the part of speech.
pub fn insert_word(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let table: String;

    //determines table to insert word to based on part of speech
    if word.1.contains("Modal verb"){
        table = "modal_verbs".to_string();
    }
    else if word.1.contains("Noun") || word.1.contains("Number"){
        table = "nouns".to_string();
    }
    else if word.1.contains("Adjective"){
        table = "adjectives".to_string();
    }
    else if word.1.contains("Adverb"){
        table = "adverbs".to_string();
    }
    else if word.1.contains("Verb"){
        table = "verbs".to_string();
    }
    else if word.1.contains("Preposition"){
        table = "prepositions".to_string();
    }
    else if word.1.contains("Exclamation"){
        table = "exclamations".to_string();
    }
    else if word.1.contains("Conjunction"){
        table = "conjunctions".to_string();
    }
    //should never be reached
    else{
        table = "".to_string();
    }

    //SQL query to insert word to database
    let query = format!("
        INSERT INTO {} (word) VALUES ('{}');
    ", table.as_str(), word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

//inserts pronouns, as the pronouns in the csv do not fit the program's needs.
fn insert_pronouns(){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO pronouns (word) VALUES ('I');
        INSERT INTO pronouns (word) VALUES ('you');
        INSERT INTO pronouns (word) VALUES ('we');
        INSERT INTO pronouns (word) VALUES ('they');
    ");

    println!("{}", query);
    connection.execute(query).unwrap();
}


//inserts articles, as the csv does not distinguish them.
fn insert_articles(){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO articles (word) VALUES ('a');
        INSERT INTO articles (word) VALUES ('the');
    ");

    println!("{}", query);
    connection.execute(query).unwrap();
}

//drops all database tables
fn clear_dictionary() {
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = "
        DROP TABLE nouns;
        DROP TABLE adjectives;
        DROP TABLE adverbs;
        DROP TABLE conjunctions;
        DROP TABLE verbs;
        DROP TABLE exclamations;
        DROP TABLE prepositions;
        DROP TABLE pronouns;
        DROP TABLE modal_verbs;
        DROP TABLE articles;
    ";

    connection.execute(query).unwrap(); 

}
