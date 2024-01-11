use log::{ info };
use std::fs;
use std::path::Path;
// use serde::Deserialize;
use serde_json;
use redis::{ Client, Commands };

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let file_path = Path::new("./data/simple.json");

    let contents = fs
        ::read_to_string(file_path)
        .expect("Should have been able to read the content from json file");

    println!("read content {}", contents);

    let data: serde_json::Value = serde_json
        ::from_str(&contents)
        .expect("Should have been able to parse the json");

    println!("Pretty-printed JSON: {}", serde_json::to_string(&data).unwrap());

    let animation_name = data["name"].to_string();

    println!("Animation name: {}", &animation_name);

    // Establish Redis connection here
    let client = redis::Client::open("redis://127.0.0.1:6379").expect("Failed to connect to Redis");
    let mut con = client.get_connection().expect("Failed to get Redis connection");

    // con.set::<std::string::String, std::string::String, ()>(
    //     animation_name.clone(),
    //     serde_json::to_string(&data).unwrap()
    // ).expect("Failed to save data to Redis");

    // let value: String = con.get(animation_name).unwrap();

    // println!("fetched data from redis, size {}", value.as_bytes().len());

    // print_type_of(&value);

    // println!("value: {}", value);


    let value1: String = con.get("yoga-starting").unwrap();

    println!("value: {}", value1);
}
