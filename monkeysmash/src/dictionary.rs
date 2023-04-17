use sqlite::{State};

use crate::parser::Word;

//builds the database
pub fn build_database() {
    clear_dictionary();

    create_dictionary();

    //fills database with words
    if let Err(e) = crate::parser::parse_file(){
        eprintln!("{}", e);
    }
    //adds words not easily extracted from the csv
    insert_pronouns();
    insert_future();
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

    String::from(random_word.to_lowercase())

}

//creates database and tables
fn create_dictionary() {
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

        CREATE TABLE future (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE articles (id INTEGER PRIMARY KEY, word TEXT);

    ";

    connection.execute(query).unwrap();

}

//inserts word from csv based on the part of speech.
pub fn insert_word(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let table: String;

    if word.1.contains("pl."){
        table = "plural_nouns".to_string();
    }
    else if word.1.contains("v. i."){
        table = "indicative_verbs".to_string();
    }
    else if word.1.contains("n."){
        table = "nouns".to_string();
    }
    else if word.1.contains("a."){
        table = "adjectives".to_string();
    }
    else if word.1.contains("adv."){
        table = "adverbs".to_string();
    }
    else if word.1.contains("v. t."){
        table = "transitive_verbs".to_string();
    }
    else if word.1.contains("p. p."){
        table = "past_participle".to_string();
    }
    else if word.1.contains("pr. p."){
        table = "present_participle".to_string();
    }
    else if word.1.contains("prep."){
        table = "prepositions".to_string();
    }
    else if word.1.contains("interj."){
        table = "interjections".to_string();
    }
    else if word.1.contains("conj."){
        table = "conjunctions".to_string();
    }
    //should never be reached
    else{
        table = "".to_string();
    }

    //SQL query to insert word to database based on part of speech
    let query = format!("
        INSERT INTO {} (word) VALUES ('{}');
    ", table.as_str(), word.0);

    println!("{}", query);
    connection.execute(query).unwrap();
}

//inserts pronouns, as the dictionary csv does not distinguish pronouns.
fn insert_pronouns(){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO pronouns (word) VALUES ('I');
        INSERT INTO pronouns (word) VALUES ('You');
        INSERT INTO pronouns (word) VALUES ('We');
        INSERT INTO pronouns (word) VALUES ('They');
    ");

    println!("{}", query);
    connection.execute(query).unwrap();
}

//inserts words and phrases that indicate a future tense, as the dictionary csv does not distinguish them.
fn insert_future(){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO future (word) VALUES ('Will');
        INSERT INTO future (word) VALUES ('Shall');
        INSERT INTO future (word) VALUES ('Will not');
        INSERT INTO future (word) VALUES ('Shall not');
    ");

    println!("{}", query);
    connection.execute(query).unwrap();
}

//inserts articles, as the dictionary does not distinguish them.
fn insert_articles(){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO articles (word) VALUES ('A');
        INSERT INTO articles (word) VALUES ('The');

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
        DROP TABLE indicative_verbs;
        DROP TABLE interjections;
        DROP TABLE past_participle;
        DROP TABLE plural_nouns;
        DROP TABLE prepositions;
        DROP TABLE present_participle;
        DROP TABLE pronouns;
        DROP TABLE transitive_verbs;
        DROP TABLE future;
        DROP TABLE articles;
    ";

    connection.execute(query).unwrap(); 

}
