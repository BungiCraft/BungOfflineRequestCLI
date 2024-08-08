use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Requests {
    maps: Vec<Map>
}
#[derive(Serialize, Deserialize, PartialEq)]
struct Map {
    bsr: String
}

fn main() {
    let path = env::var("LOCALAPPDATA").expect("No APP_DATA directory");
    let directory_binding = Path::new("..").join(&path).join(".BungRequest");
    let directory_path = directory_binding.as_path();
    if !Path::exists(directory_path) {
        fs::create_dir(directory_path).expect("Error creating directory .BungRequest");
    }
    let requests_binding = Path::new("..").join(&directory_path).join("requests.json");
    let requests_path = requests_binding.as_path();
    if !Path::exists(requests_path) {
        let mut file = File::create(requests_path).expect("Error creating file requests.json");
        file.write_all(b"{\r\n    \"maps\": [\r\n    ]\r\n}").expect("Error writing data to file requests.json");
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let bsr = &args[1];
        let mut requests: Requests = serde_json::from_str(&*fs::read_to_string(requests_path).expect("Error reading file requests.json")).unwrap();
        let request = Map { bsr: String::from(bsr) };
        if requests.maps.contains(&request) { // Prevent adding the same code twice
            return println!("Map is already requested!");
        }
        requests.maps.push(request);
        let json = serde_json::to_string(&requests).unwrap();
        fs::write(requests_path, json).expect("Error writing to file requests.json");
        println!("Requested map: {} :3c", bsr)
    }
}
