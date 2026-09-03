use std::collections::HashMap;
use std::io;
use std::io::Write;

pub enum Record {
    Put { key: String, value: String },
    Delete { key: String },
}

impl Record {
    fn encode(&self) -> Vec<u8> {
        let (op, key, value) = match self {
            Record::Put {key, value} => (1u8, key.as_str(), value.as_str()),
            Record::Delete { key } => (2u8, key.as_str(), "")
        };
        let key_len = key.len() as u32;
        let value_len = value.len() as u32;
        let key_len_len = 4 as u32;
        let value_len_len = 4 as u32;
        let op_len = 1 as u32;
        let tot_len = key_len + value_len + key_len_len + value_len_len + op_len;

        let mut out = Vec::with_capacity(4 + tot_len as usize);
        out.extend_from_slice(&tot_len.to_le_bytes());
        out.extend_from_slice(&op.to_le_bytes());
        out.extend_from_slice(&key_len.to_le_bytes());
        out.extend_from_slice(&value_len.to_le_bytes());
        out.extend_from_slice(&key.as_bytes());
        out.extend_from_slice(&value.as_bytes());
        out
    }
}

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
        let r = Record::Put {
            key: key.to_string(),
            value: value.to_string(),
        };
        let bytes: Vec<u8> = r.encode();
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
