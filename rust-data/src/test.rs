// use log::{ info };
// use std::fs;
// use std::path::Path;
use serde::Serialize;
use serde_json;
use redis::{ Client, Commands, Connection };

#[derive(Serialize)]
struct AnimationMetadata {
    name: String,
    repeat: i32,
    text: Option<String>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn serialize_json_list(list: &[AnimationMetadata]) -> Vec<String> {
    list.iter()
        .map(|obj| serde_json::to_string(obj).unwrap())
        .collect()
}

fn save_to_redis(client: &mut Connection, list: &[String], list_key: &str) {
    for json_string in list {
        client
            .rpush::<&&str, &std::string::String, ()>(&list_key, json_string)
            .expect("RPUSH failed");
    }
}

fn save_list_to_redis() {
    // try serialize_json_list
    let list = vec![
        AnimationMetadata {
            name: "yoga-starting".to_string(),
            repeat: 0,
            text: Some("Hi there, how may I assist you today".to_string()),
        },
        AnimationMetadata {
            name: "yoga-starting".to_string(),
            repeat: 0,
            text: None,
        }
    ];

    let serialized_list = serialize_json_list(&list);

    println!("serialized_list: {:?}", serialized_list);

    // Establish Redis connection here
    let client = redis::Client::open("redis://127.0.0.1:6379").expect("Failed to connect to Redis");
    let mut con = client.get_connection().expect("Failed to get Redis connection");

    let list_key = "amq:greeting";

    save_to_redis(&mut con, &serialized_list, list_key);
}

fn main() {
    save_list_to_redis();

    // let file_path = Path::new("./data/simple.json");

    // let contents = fs
    //     ::read_to_string(file_path)
    //     .expect("Should have been able to read the content from json file");

    // println!("read content {}", contents);

    // let data: serde_json::Value = serde_json
    //     ::from_str(&contents)
    //     .expect("Should have been able to parse the json");

    // println!("Pretty-printed JSON: {}", serde_json::to_string(&data).unwrap());

    // let animation_name = data["name"].to_string();

    // println!("Animation name: {}", &animation_name);

    // // Establish Redis connection here
    // let client = redis::Client::open("redis://127.0.0.1:6379").expect("Failed to connect to Redis");
    // let mut con = client.get_connection().expect("Failed to get Redis connection");

    // // con.set::<std::string::String, std::string::String, ()>(
    // //     animation_name.clone(),
    // //     serde_json::to_string(&data).unwrap()
    // // ).expect("Failed to save data to Redis");

    // // let value: String = con.get(animation_name).unwrap();

    // // println!("fetched data from redis, size {}", value.as_bytes().len());

    // // print_type_of(&value);

    // // println!("value: {}", value);

    // let value1: String = con.get("yoga-starting").unwrap();

    // println!("value: {}", value1);
}
