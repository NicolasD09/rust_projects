use std::{collections::HashMap, error::Error};

fn main() {
    let mut args = std::env::args().skip(1); // gets the arguments from the command line, skips the first element because it is the executable path
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents);

    println!("Key: {:?}  | Value: {:?}", key, value);

    // Run with arguments  : cargo r -- arg arg

}

struct Database {
    inner: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(e) => return Err(e)
        };

        Ok(Database{
            inner: HashMap::new()
        })
    }
}

