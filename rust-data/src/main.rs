use log::{ info };
use std::fs;
use std::path::Path;
use serde::Deserialize;
use serde_json;
use redis::{ Client, Commands };


#[derive(Deserialize)]
struct Animation {
    // blendMode: i32,
    // duration: f64,
    name: String,
    // tracks: Vec<TrackItem>, // Array of TrackItem structs
    // uuid: String,
}

// #[derive(Deserialize)]
// struct TrackItem {
//     name: String,
//     times: Vec<f64>,
//     values: Vec<f64>,
//     r#type: String, // Use r# to avoid "type" keyword clash
// }

/**
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/

fn main() {
   
    // read json data from file, and save it to Redis
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    info!("Logger is ready!");

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

        let data: Animation = serde_json
            ::from_str(&contents)
            .expect("Should have been able to parse the json");

        // println!("Name: {}, Duration: {}, uuid: {}", data.name, data.duration, data.uuid);

        let serialized_data = serde_json::to_string(&contents).expect("Failed to serialize data");

        let client = Client::open("redis://127.0.0.1:6379").expect("Redis connection failed");
        let mut con = client.get_connection().expect("Failed to get Redis connection");

        con.set::<std::string::String, std::string::String, ()>(data.name, serialized_data).expect(
            "Failed to save data to Redis"
        );

        println!("Animation data saved to Redis successfully!");
    }
}
