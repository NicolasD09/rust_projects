use std::{collections::HashMap};

fn main() {
    let mut args = std::env::args().skip(1); // gets the arguments from the command line, skips the first element because it is the executable path
    
    /*
      unwrap() throws an error if the method next() fails and the value is null
    */
    let key = args.next().unwrap(); // gets the next argument
    let value = args.next().unwrap(); // gets the next argument

    let contents = format!("{}\t{}\n", key, value);
    //std::fs::write("kv.db", contents);
    println!("Key: {:?}  | Value: {:?}", key, value);

    // Run with arguments  : cargo r -- arg arg

    let database = Database::new();

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
        /* Same thing as above */ let contents = std::fs::read_to_string("kv.db")?;

        Ok(Database{
            map: HashMap::new()
        })
    }
}

