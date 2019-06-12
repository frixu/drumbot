mod structs;

use std::{collections::HashMap, clone::Clone, io::{BufRead, BufReader}, fs::File};
use std::{time::Duration, thread};
use reqwest::{Client, StatusCode};
use rodio::{Source, Sink};

const ENDPOINT: &str = "https://api.noopschallenge.com/drumbot/patterns/";

fn main() {
    let mut patterns = HashMap::new();
    let device = rodio::default_output_device().unwrap();
    let client = Client::new();
    
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

    println!("Which pattern do you want to play? Available options:");
    for key in patterns.keys() { println!("{}", key); }
    let buffer = std::io::stdin().lock().lines().next().unwrap().unwrap();
    if patterns.contains_key(&buffer) {
        if patterns.get(&buffer).unwrap().beats_per_minute == 0 {
            let url = format!("{}{}", ENDPOINT, buffer);
            let new_pattern = client.get(&url[..]).send().unwrap().text().unwrap();
            let pattern : structs::Pattern = serde_json::from_str(&new_pattern).unwrap();
            patterns.insert(buffer.clone(), pattern);
        }
        println!("Here we go!");
        let pattern = patterns.get(&buffer).unwrap();
        let first_track = &pattern.tracks[0];
        let file = File::open(format!("samples/{}.wav", first_track.instrument.clone())).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
        thread::sleep(Duration::from_secs(86400));
    }
    else {
        println!("No such pattern exists!");
    }

}
