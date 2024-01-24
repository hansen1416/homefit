use log::{ info };
use std::fs;
use std::path::Path;
use serde::Serialize;
use serde_json;
use redis::{ Client, Commands, Connection };

// #[derive(Deserialize)]
// struct Animation {
//     blendMode: i32,
//     duration: f64,
//     name: String,
//     tracks: Vec<TrackItem>, // Array of TrackItem structs
//     uuid: String,
// }

// #[derive(Deserialize)]
// struct TrackItem {
//     name: String,
//     times: Vec<f64>,
//     values: Vec<f64>,
//     r#type: String, // Use r# to avoid "type" keyword clash
// }

#[derive(Serialize)]
struct AnimationMetadata {
    name: String,
    repeat: i32,
    text: Option<String>,
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

fn main() {
    // read json data from file, and save it to Redis
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    info!("Logger is ready!");

    let client = Client::open("redis://127.0.0.1:6379").expect("Redis connection failed");

    let mut con = client.get_connection().expect("Failed to get Redis connection");

    // save list data to Redis
    let list = vec![
        AnimationMetadata {
            name: "greeting".to_string(),
            repeat: 0,
            text: Some("Hi there, how may I assist you today".to_string()),
        },
        AnimationMetadata {
            name: "pointing-forward".to_string(),
            repeat: 0,
            text: None,
        }
    ];

    let serialized_list = serialize_json_list(&list);

    println!("serialized_list: {:?}", serialized_list);

    let list_key = "amq:greeting";

    save_to_redis(&mut con, &serialized_list, list_key);

    // read json data from file, and save it to Redis
    let data_dir = Path::new("./data");

    let paths = fs::read_dir(data_dir).expect("data_dir should be a directory");

    for path in paths {
        let path_buf = path.unwrap().path(); // Handle potential errors

        // Method 1: Using file_name()
        let filename = path_buf
            .file_name()
            .unwrap()
            .to_str()
            .expect("Failed to convert filename to string");

        // if not a .json file, continue
        if !filename.ends_with(".json") {
            info!("Not a json file: {}", filename);
            continue;
        }

        info!("Found json file: {}", filename);

        let contents = fs
            ::read_to_string(path_buf)
            .expect("Should have been able to read the content from json file");

        info!(
            "read content bytes: {}mb",
            format!("{:.2}", (contents.as_bytes().len() as f64) / 1024.0 / 1024.0)
        );

        let data: serde_json::Value = serde_json
            ::from_str(&contents)
            .expect("Should have been able to parse the json");

        let animation_name = match data["name"].as_str() {
            // prefix with "am::" to indicate animation metadata
            Some(name) => format!("am:{}", name),
            None => panic!("Invalid data type for 'name' field"),
        };

        con.set::<std::string::String, std::string::String, ()>(
            animation_name.clone(),
            serde_json::to_string(&data).unwrap()
        ).expect("Failed to save data to Redis");

        info!("Animation data {} saved to Redis successfully!", &animation_name);

        // println!("Pretty-printed JSON: {}", serde_json::to_string(&data).unwrap());

        // let value: String = con.get(&animation_name).unwrap();

        // println!("value: {}", value);
    }
}
