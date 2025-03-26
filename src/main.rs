use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct Requests {
    maps: Vec<Map>
}
#[derive(Serialize, Deserialize, PartialEq)]
struct Map {
    bsr: String
}

fn main() {
    // Set up path
    let path = env::var("LOCALAPPDATA")
        .expect("No APP_DATA directory");
    let directory_binding = Path::new("..")
        .join(&path)
        .join(".BungRequest");
    let directory_path = directory_binding
        .as_path();

    // If the directory doesn't exist, create it
    if !Path::exists(directory_path) {
        fs::create_dir(directory_path)
            .expect("Error creating directory .BungRequest");
    }

    // Add file to path
    let requests_binding = Path::new("..")
        .join(&directory_path)
        .join("requests.json");
    let requests_path = requests_binding
        .as_path();

    // If the file doesn't exist, create it
    if !Path::exists(requests_path) {
        let mut file = File::create(requests_path)
            .expect("Error creating file requests.json");
        file.write_all(b"{\r\n    \"maps\": [\r\n    ]\r\n}")
            .expect("Error writing data to file requests.json");
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // Read the requests from the file
        let mut requests: Requests = serde_json::from_str(&*fs::read_to_string(requests_path)
            .expect("Error reading file requests.json"))
            .expect("Error parsing JSON");
        // Create a regex to check if the argument is a valid BSR code (hexadecimal)
        let reg = Regex::new(r"^[0-9a-fA-F]+$")
            .expect("Error creating regex");

        for bsr in args.iter() {
            if reg.is_match(bsr) {
                let request = Map { bsr: String::from(bsr) };
                if requests.maps.contains(&request) { // Prevent adding the same code twice
                    println!("Map is already requested!");
                } else {
                    requests.maps.push(request);
                    println!("Requested map: {} :3c", bsr);
                }
            }
        }

        // Write the updated requests to the file
        let json = serde_json::to_string(&requests)
            .expect("Error serializing JSON");
        fs::write(requests_path, json)
            .expect("Error writing to file requests.json");
    }
}