use std::{collections::HashMap};

const DB_FILE_PATH: &'static str = "kv.db";
fn main() {
    let mut args = std::env::args().skip(1); // gets the arguments from the command line, skips the first element because it is the executable path
    
    /*
      unwrap() throws an error if the method next() fails and the value is null
    */
    let key = args.next().unwrap(); // gets the next argument
    let value = args.next().unwrap(); // gets the next argument

    let contents = format!("{}\t{}\n", key, value);
    
    std::fs::write(DB_FILE_PATH, contents);

    println!("Key: {:?}  | Value: {:?}", key, value);

    // Run with arguments  : cargo r -- arg arg

    let database = Database::new().expect("Creating DB failed");

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
        /* Same thing as above */ let contents = std::fs::read_to_string(DB_FILE_PATH)?;

        let mut tmp_map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            // (key, value) is a tuple, which is like an array but allows different types of values
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            tmp_map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database{
            map: tmp_map
        })
    }
}

