use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::str::FromStr;

// TODO:
// - [x] Save a key value pair to memory
// - [x] Get all key value pairs from memory
// - [x] Get a key value pair with a key from memory
// - [x] Update a key value pair in memory
// - [x] Delete a key value pair from memory
// - [x] CLI to send commands to interact with the db
// - [ ] Multiple data types as value like HashMap::<String, i32> or HashMap::<String, HashMap::<String, i32>>
// - [ ] Atomicity
// - [ ] B-tree datastrucutre instead of just regular hashmap

pub enum Value {
    ValueInt(i32),
    ValueString(String),
}

struct Pair {
    key: String,
    value: Value,
}

fn main() {
    let mut storage: HashMap::<String, Value> = Default::default();

    // cli to use interact with
    loop {
        print!("storage > "); 
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        println!("Command: {}", command);

        parse_command(&command.to_string(), &mut storage);
    }

    // EXAMPLES
    // SET hello, world
    // save(String::from("hello"), String::from("world"), &mut storage);
    //
    // GET *
    // get_all(&mut storage);
    //
    // save(String::from("hello_two"), String::from("world two"), &mut storage);
    //
    // get_all(&mut storage);
    //
    // // GET hello
    // let value = get_by_key(&mut storage, String::from("hello"));
    // println!("{:?}", value.unwrap());
    // let value_two = get_by_key(&mut storage, String::from("hello_two"));
    // println!("{:?}", value_two.unwrap());
    //
    // // SET also updates existing value
    // // SET hello_two, world updated
    // save(String::from("hello_two"), String::from("world updated"), &mut storage);
    // let value_two_upd = get_by_key(&mut storage, String::from("hello_two"));
    // println!("{:?}", value_two_upd.unwrap());
    //
    // let removed_value = delete_by_key(&mut storage, String::from("hello_two"));
    // println!("{:?}", removed_value.as_ref().unwrap());
    //
    // get_all(&mut storage);
}


// Saves new key value pair
fn save(key: String, value: Value, storage: &mut HashMap<String, Value>) -> &HashMap<String, Value> {
    let _ = &storage.insert(key, value);

    storage
}

// Returns every key val pair in storage
fn get_all(storage: &mut HashMap<String, Value>) {
    println!("{:?}", storage);
}

// Returns value by specified key
fn get_by_key(storage: &mut HashMap<String, Value>, key: String) -> std::option::Option<&Value> {
    let value = &storage.get(&key.to_string());

    *value
}

// Deletes value by specified key
fn delete_by_key(storage: &mut HashMap<String, Value>, key: String) -> std::option::Option<Value> {
    let removed_value = &storage.remove(&key.to_string());

    removed_value.clone()
}

// Parses command and calls function to use
fn parse_command(command: &str, storage: &mut HashMap<String, Value>) {
    let split_command: Vec<&str> = command.split(" ").collect();
    println!("{:?}", split_command);

    let params = split_command[1].trim();

    if split_command[0] == "GET" {
        if params == "*" {
            get_all(storage);
        }
        else {
            println!("{:?}", get_by_key(storage, params.to_string()).unwrap());
        }
    }
    else if split_command[0] == "SET" {
        let key_value: Vec<&str> = params.split(",").collect();
        let key = key_value[0];
        let value = key_value[1];

        if value.is_digit(10) {
            save(key.to_string(), Value::ValueInt(FromStr::from_str(value).unwrap()), storage);
        }
        else {
            save(key.to_string(), Value::ValueString(value.to_string()), storage);
        }
    }
    else if split_command[0] == "DELETE" {
        println!("{:}", delete_by_key(storage, params.to_string()).as_ref().unwrap());
    }
    else {
        println!("Command not recoginzed");
    }
    // Every function should propably return some custome type. After that we could
    // implement the stuff above with this simple match statement
    // match split_command[0] {
    //     // "GET" if params == "*" => get_all(storage),
    //     // "GET" => get_by_key(storage, params.to_string()),
    //     // "SET" => save(key, value, storage),
    //     // "DELETE" => delete_by_key(storage, params),
    //     _ => println!("Command not recoginzed"),
    // }
}
