const WORDNET: &'static str = "./src/wordnet.jsonl";
use std::{collections::HashMap, env, fs, io::Write};

use libflate::gzip::Encoder;
use serde_json::Value;

fn main() {
    // Tell Cargo that if the dictionary files change, to rerun this build script.
    println!("cargo:rerun-if-changed={WORDNET}");

    let thes = fs::read_to_string(WORDNET).expect(&format!("Failed to read \"{WORDNET}\"")).to_lowercase();

    let mut thes_structured: HashMap<String, Vec<String>> = HashMap::new();

    for line in thes.lines() {
        let json: Value = serde_json::from_str(&line).unwrap();

        let name = json["word"].to_string().replace('"', "");
        let synonyms = json["synonyms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.to_string().replace('"', ""))
            .collect::<Vec<String>>();

        if let Some(x) = thes_structured.get_mut(&name) {
            x.extend_from_slice(&synonyms);
            x.dedup();
        } else {
            thes_structured.insert(name, synonyms);
        }
    }

    let mut thes_final = String::new();

    for (name, synonyms) in thes_structured {
        thes_final.push_str(&format!("{}|{}\n", name, synonyms.join("|")))
    }

    thes_final.pop();

    let compressed = {
        let mut encoder = Encoder::new(Vec::new()).expect("Failed to create compressor");
        encoder
            .write_all(thes_final.as_bytes())
            .expect("Failed to compress dictionary");
        encoder
            .finish()
            .into_result()
            .expect("Failed to finish compressing dictionary")
    };

    let target_dir = env::var("OUT_DIR").unwrap();

    fs::write(format!("{}/{}", target_dir, "wordnet.gz"), compressed).expect(&format!(
        "Failed to write compressed dictionary to `{}/wordnet.gz`",
        target_dir
    ));
}