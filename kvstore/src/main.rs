use std::{collections::HashMap};

const DB_FILE_PATH: &'static str = "kv.db";
fn main() {
    let mut args = std::env::args().skip(1); // gets the arguments from the command line, skips the first element because it is the executable path
    
    /*
      unwrap() throws an error if the method next() fails and the value is null
    */
    let key = args.next().unwrap(); // gets the next argument
    let value = args.next().unwrap(); // gets the next argument

    // Run with arguments  : cargo r -- arg arg
    // the "--" separates the run command from the arguments

    let mut database = Database::new().expect("Creating DB failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();

}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(e) => return Err(e)
        // };
        /* Same thing as above
           read_to_string returns a Result that we loop on after
           the ? at the end manages the errors as the code above does

           a String is an owned string, which owns the memory associated with the value
           a &str is a string slice, which is a pointer that lets us view parts of a String, it is borrowing the memory, not allocating it
           a &string is also a pointer, but has all the data of the String, and not just a part of it
        */
        let contents = std::fs::read_to_string(DB_FILE_PATH)?;

        let mut tmp_map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            // (key, value) is a tuple, which is like an array but allows different types of values
            let (key, value) = line.split_once('\t').expect("Corrupt database");

            // Inserts the key and value into the map
            // .to_owned() allocates new memory 
            tmp_map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database{
            map: tmp_map
        })
    }

    // insert is a method because we have the argument "self"
    // if we didn't have self, we would need to pass the database as an argument
    // e.g : Database::insert(database, key, value), instead of database.insert(key, value)
    fn insert(&mut self, key: String, value: String){
        // self is the database object instantiated
        // the insert method comes with Hashmaps : std::collections::hash::map:HashMap
        self.map.insert(key,value);
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write(DB_FILE_PATH, contents)
    }
}

