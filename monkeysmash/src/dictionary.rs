use crate::parser::Word;

pub fn create_dictionary() {
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = "
        CREATE TABLE nouns (id INTEGER PRIMARY KEY, word TEXT);
        
        CREATE TABLE adjectives (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE adverbs (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE prepositions (id INTEGER PRIMARY KEY, word TEXT;

        CREATE TABLE transitive verbs (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE plural nouns (id INTEGER PRIMARY KEY, word TEXT);

        CREATE TABLE past participle (id INTEGER PRIMARY KEY, word TEXT);
        

    ";

    connection.execute(query).unwrap();

}

pub fn fill_dictionary(word: Word){
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = format!("
        INSERT INTO words (word, pos) VALUES ('{}', '{}');
    ", word.0, word.1);

    println!("{}", query);
    connection.execute(query).unwrap();
}

pub fn clear_dictionary() {
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = "
        DROP TABLE words;
    ";

    connection.execute(query).unwrap(); 

}