/*
If you want to see an actually good implementation of this, check out:
https://www.youtube.com/channel/UC9m7D4XKPJqTPCLSBym3BCg/ (Fred Overflow)
https://github.com/fredoverflow/wordle 

This was just a fun little project that I spent like an hour on.
I would like to thank Fred Overflow again for the idea to represent words as integers.

*/


//! Use release mode
use rayon::prelude::*;
use std::time::Instant;

mod word;
use word::*;

const WORD_FILE: &str = "res/wordle-nyt-allowed-guesses.txt";
const WORD_2_FILE: &str = "res/wordle-nyt-answers-alphabetical.txt";

pub fn read_words(path: &str) -> String {
    use std::path::Path;
    use std::io::Read;
    use std::fs::File;
    let p = Path::new(path);
    let d = p.display();
    let mut buf = String::new();

    {
        let mut f = match File::open(p) {
            Ok(f) => f,
            Err(_) => panic!("Couldn't open {}", d)
        };
        match f.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(_) => panic!("Couldn't read from {}", d)
        }
    }

    if let Some('\n') = buf.chars().next_back() { buf.pop(); }
    if let Some('\r') = buf.chars().next_back() { buf.pop(); }

    buf
}


pub fn main() {
    let t_total = Instant::now();

    let mut word_string_buf = String::new();
    println!("Reading words from disk...");
    let t = Instant::now();
    word_string_buf += &read_words(WORD_FILE);
    word_string_buf.push('\n');
    word_string_buf += &read_words(WORD_2_FILE);
    println!("Done: {:?}\n", t.elapsed());

    println!("Loading words...");
    let t = Instant::now();
    let mut words: Vec<Word> = word_string_buf.par_lines()
        .map(|s| {
            Word::new(s)
        })
        .filter(|s| {
            s.int_repr.count_ones() == 5
        })
        .collect();

    words.sort_unstable_by(|a, b| a.int_repr.cmp(&b.int_repr));
    words.dedup_by(|a, b| a.int_repr == b.int_repr);

    let words_len = words.len();
    println!("Loaded {} words in {:?}.\n", words_len, t.elapsed());
    
    println!("Creating next_word vector...");
    let t = Instant::now();
    let next_word: Vec<Vec<usize>> = words
        .par_iter()
        .enumerate()
        .map(|(i, w)| {
            let mut res = vec![];
            for j in (i + 1)..words_len {
                if w & &words[j] == 0 {
                    res.push(j);
                }
            }

            res
        })
        .collect();
    println!("Done: {:?}\n", t.elapsed());

    println!("Starting search...");
    let t = Instant::now();    
    words.par_iter().enumerate().for_each(|(i, w)| {
        let js = &next_word[i];
        for &j in js {
            let wj = &words[j];
            let ks = &next_word[j];
            for &k in ks {
                let wk = &words[k];
                if wk & w != 0 {
                    continue;
                }
                let ls = &next_word[k];
                for &l in ls {
                    let wl = &words[l];
                    if wl & w != 0 || wl & wj != 0 {
                        continue;
                    }
                    let ms = &next_word[l];
                    for &m in ms {
                        let wm = &words[m];
                        if wm & w != 0 || wm & wj != 0 || wm & wk != 0 {
                            continue;
                        }
                        println!("{}, {}, {}, {}, {}", w, wj, wk, wl, wm);
                    }
                }
            }
        }
    });
    println!("Done: {:?}\n", t.elapsed());

    println!("Total: {:?}", t_total.elapsed());
}
