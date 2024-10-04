use  kv_store::KVStore;
use  std::io;
fn main() {
    let mut  kv_store = KVStore::new();

    loop {
        println!("Enter a command (SET, GET, DELETE o EXIT):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_parts: Vec<&str> = input.trim().split_whitespace().collect();

        if input_parts.is_empty() {
            continue;
        }

        match input_parts[0].to_uppercase().as_str() {
            "SET" => {
                if input_parts.len() == 3 {
                    kv_store.set(input_parts[1].to_string(), input_parts[2].to_string());
                    println!("Key-value pair set.");
                } else {
                    println!("Usage: SET key value");
                }
            },
            "GET" => { 
                if input_parts.len() == 2 {
                    match kv_store.get(input_parts[1]) {
                        Some(value) => println!("Value: {}", value),
                        None => println!("Key not found"),
                    }
                } else {
                    println!("Usage: SET key value")
                }

            },
            "DELETE" => {
                if input_parts.len() == 2 {
                    match kv_store.delete(input_parts[1]) {
                        Some(_) => println!("Key deleted"),
                        None => println!("Key not found"),
                    }
                } else {
                    println!("Usage: DELETE key")
                }
            },
            "EXIT" => {
                println!("Exiting...");
                break;
            },
            _ => {  
                println!("Unknown command. Please use SET, GET, DELETE, OR EXIT.");
            }
        }
    }
}
