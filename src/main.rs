use std::collections::HashMap;
use std::io;
use std::io::Write;

fn get(db: &HashMap<String, String>, command: &Vec<&str>) {
    if command.len() != 2 {
        println!("get command needs only single argument");
    } else {
        println!(
            "{}",
            db.get(command[1]).map(String::as_str).unwrap_or_default()
        );
    }
}

fn put(db: &mut HashMap<String, String>, command: &Vec<&str>) {
    if command.len() != 3 {
        println!("put command needs 2 arguments");
    } else {
        // let key: str = command[1].map(String::as_str).unwrap_or_default();
        let key: &str = command[1];
        // let value: str = command[2].map(String::as_str).unwrap_or_default();
        let value: &str = command[2];
        db.insert(key.to_string(), value.to_string());
        println!("ok");
    }
}

fn main() {
    let mut db = HashMap::<String, String>::new();
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let words: Vec<&str> = input.split_whitespace().collect();
        if words[0] != "get" && words[0] != "put" {
            println!("Commands Supported :- get, put");
        } else {
            if words[0] == "get" {
                get(&db, &words);
            } else {
                put(&mut db, &words);
            }
        }
        input.clear();
    }
}
