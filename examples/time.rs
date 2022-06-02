use chrono::Utc;

fn main() {
    let b = Utc::now();
    thesaurus::init();
    println!("Init took {} ms", Utc::now().signed_duration_since(b).num_milliseconds());
}
