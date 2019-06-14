mod structs;

use std::{collections::HashMap, clone::Clone, io::{BufRead, BufReader}, fs::File};
use reqwest::{Client, StatusCode};

const ENDPOINT: &str = "https://api.noopschallenge.com/drumbot/patterns/";

fn main() {
    let mut patterns = HashMap::new();
    let mut instruments = HashMap::new();
    // Create an audio listening device.
    let device = rodio::default_output_device().unwrap();
    // Create a HTTP client.
    let client = Client::new();
    
    // Fetch a list of available patterns.
    let mut list = client.get(ENDPOINT).send().unwrap();
    match list.status() {
        StatusCode::OK => {
            let body = list.text().unwrap();
            let names : Vec<structs::Pattern> = serde_json::from_str(&body).unwrap();
            for pattern in names {
                patterns.insert(pattern.name.clone(), pattern);
            }
        },
        _ => panic!("The request did not return OK."),
    }

    // Ask the user which pattern to play.
    println!("Which pattern do you want to play? Available options:");
    for key in patterns.keys() { println!("{}", key); }
    let buffer = std::io::stdin().lock().lines().next().unwrap().unwrap();

    // Did the user choose an incorrect pattern?
    if !patterns.contains_key(&buffer) {
        println!("No such pattern exists!");
        std::process::exit(0);
    }

    // Check if the pattern is cached in memory.
    if patterns.get(&buffer).unwrap().beats_per_minute == 0 {
        // If not, download it.
        let url = format!("{}{}", ENDPOINT, buffer);
        let body = client.get(&url[..]).send().unwrap().text().unwrap();
        let pattern : structs::Pattern = serde_json::from_str(&body).unwrap();
        patterns.insert(buffer.clone(), pattern);
    }

    let pattern = patterns.get(&buffer).unwrap();

    // Load instrument samples.
    for track in &pattern.tracks {
        let name = track.instrument.clone();
        let file = File::open(format!("samples/{}.wav", name)).unwrap();
        instruments.insert(name, file);
    }

    // TODO: Create a loop that plays the pattern.

    //If Enter/Return is pressed, stop the application.
    std::io::stdin().lock().lines().next().unwrap().unwrap();
}