use serde_json;
use std::fs;

fn main() {
    let mut wordle = fs::read_to_string("input.txt").unwrap();
    let options: serde_json::Value = serde_json::from_str(&fs::read_to_string("replace.json")
    .unwrap())
    .expect("JSON was not properly formatted.");
    wordle = wordle.replace("🟩", &options["correct"]
    .to_string()).replace("🟨", &options["in_word"]
    .to_string()).replace("⬛", &options["wrong"]
    .to_string()).replace('"', "");

    println!("{}", wordle);
}