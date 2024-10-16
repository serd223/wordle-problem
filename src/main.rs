//! Use release mode
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod letters;
mod word;
use word::*;

const WORD_FILE: &str = "res/wordle-nyt-allowed-guesses.txt";
const WORD_2_FILE: &str = "res/wordle-nyt-answers-alphabetical.txt";

const JQ: u32 = 1 << 27 | 1 << 26;
/*
Is the whole JQ thing cheating?
Well, if we wanted to do this same search on words from other languages, it would not work.
Is that a bad thing or is it okay? I am not sure.
*/

pub fn read_words(path: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    let p = Path::new(path);
    let d = p.display();
    let mut buf = String::new();

    {
        let mut f = match File::open(p) {
            Ok(f) => f,
            Err(_) => panic!("Couldn't open {}", d),
        };
        match f.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(_) => panic!("Couldn't read from {}", d),
        }
    }

    if let Some('\n') = buf.chars().next_back() {
        buf.pop();
    }
    if let Some('\r') = buf.chars().next_back() {
        buf.pop();
    }

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
    let mut words: Vec<Word> = word_string_buf
        .par_lines()
        .filter(|s| {
            s.len() == 5 // This check is redundant unless we use the words_alpha file as a source
        })
        .map(Word::new)
        .filter(|s| s.int_repr.count_ones() == 5)
        .collect();

    words.sort_unstable_by(|a, b| (a.int_repr ^ JQ).cmp(&(b.int_repr ^ JQ)));
    words.dedup_by(|a, b| a.int_repr == b.int_repr);

    let mut jq_split = 0;
    while (words[jq_split].int_repr & JQ) != 0 {
        jq_split += 1; // 284
    }

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
    println!("Created next_word vector in {:?}.\n", t.elapsed());

    println!("Starting search...");
    let results: Vec<[&str; 5]> = vec![];
    let res_mut = Arc::new(Mutex::new(results));

    let t = Instant::now();
    words[..jq_split].par_iter().enumerate().for_each(|(i, w)| {
        let mut res_buf = vec![];
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

                        res_buf.push([
                            w.str_repr,
                            wj.str_repr,
                            wk.str_repr,
                            wl.str_repr,
                            wm.str_repr,
                        ]);
                    }
                }
            }
        }

        let mut res_guard = res_mut.lock().unwrap();
        res_guard.append(&mut res_buf);
    });
    println!("Searched through all words in {:?}.\n", t.elapsed());

    let res_guard = res_mut.lock().unwrap();
    for r in res_guard.iter() {
        println!("{:?}", r);
    }
    // drop(res_guard);

    println!("\nTotal: {:?}", t_total.elapsed());
}
