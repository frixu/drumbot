mod structs;

use std::{collections::HashMap, clone::Clone, io::BufRead, thread, time::Duration};
use reqwest::{Client, StatusCode};
use ears::{Sound, AudioController};

const ENDPOINT: &str = "https://api.noopschallenge.com/drumbot/patterns/";

fn main() {
    let mut patterns = HashMap::new();
    let mut instruments = HashMap::new();
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
        let file_name = format!("samples/{}.wav", name);
        let sound = Sound::new(&file_name[..]).unwrap();
        instruments.insert(name, sound);
    }

    println!("Here you go! Stop playback with ^C.");

    // Play the pattern.
    let mut tick_number : u16 = 0;
    loop {
        for track in &pattern.tracks {
            // If the step is marked as 1, play the sound.
            if track.steps[tick_number as usize] == 1 {
                let name = track.instrument.clone();
                let sound = &mut instruments.get_mut(&name).unwrap();
                sound.play();
            }
        }
        // 60000 milliseconds in a minute. Assuming quadruple drum pattern.
        thread::sleep(Duration::from_millis((15000 / pattern.beats_per_minute) as u64));
        // Move to the next tick. If the pattern is over, restart.
        tick_number += 1;
        if tick_number == pattern.step_count {
            tick_number = 0;
        }
    }
}