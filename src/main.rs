//! Use release mode
use rayon::prelude::*;
use std::time::Instant;

pub fn read_words_to(path: &str, buf: &mut String) -> Result<(), String>{
    use std::path::Path;
    use std::io::Read;
    use std::fs::File;
    let p = Path::new(path);
    let d = p.display().to_string();

    {
        let mut f = match File::open(p) {
            Ok(f) => f,
            Err(why) => return Err(d + ": " + &why.to_string())
        };
        match f.read_to_string(buf) {
            Ok(_) => (),
            Err(why) => return Err(d + ": " + &why.to_string())
        }
    }

    Ok(())
}

const WORD_FILE: &str = "res/wordle-nyt-allowed-guesses.txt";
const WORD_2_FILE: &str = "res/wordle-nyt-answers-alphabetical.txt";

pub fn main() {
    let t = Instant::now();

    let mut word_string_buf = String::new();
    read_words_to(WORD_FILE, &mut word_string_buf).expect("Couldn't read {WORD_FILE}");
    read_words_to(WORD_2_FILE, &mut word_string_buf).expect("Couldn't read {WORD_2_FILE}");

    
    println!("Total: {:?}", t.elapsed());
}
