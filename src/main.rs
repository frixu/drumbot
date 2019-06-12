mod structs;

use std::{collections::HashMap, clone::Clone, io::BufRead};
use reqwest::{Client, StatusCode};

const ENDPOINT: &str = "https://api.noopschallenge.com/drumbot/patterns/";

fn main() {
    let mut patterns = HashMap::new();
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
            patterns.insert(buffer, pattern);
        }
        println!("Here we go!");
    }
    else {
        println!("No such pattern exists!");
    }

}
