pub fn create_dictionary() {
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = "
        CREATE TABLE words (id INTEGER PRIMARY KEY, word TEXT, pos TEXT);
    
        INSERT INTO words (word, pos) VALUES ('test', 'noun');
    ";

    connection.execute(query).unwrap();

}

pub fn fill_dictionary() {
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = "
    
    
    ";

    connection.execute(query).unwrap();
}

pub fn clear_dictionary() {
    let connection = sqlite::open("dictionary.db").unwrap();

    let query = "
        DROP TABLE words;
    ";

    connection.execute(query).unwrap(); 

}