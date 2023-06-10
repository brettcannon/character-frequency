extern crate unicode_normalization;

use std::{collections::HashMap, fs, thread};

use unicode_normalization::UnicodeNormalization;

fn main() {
    println!(
        "Loading in the corpus of {} files ...",
        std::env::args().len() - 1
    );
    let mut counters = Vec::new();
    for file in std::env::args().skip(1) {
        counters.push(thread::spawn(move || {
            let raw_contents = fs::read_to_string(file).expect("Failed to read file");
            let contents: Vec<char> = raw_contents.nfc().collect();

            let mut unigrams: HashMap<char, u32> = HashMap::new();
            let mut bigrams: HashMap<(char, char), u32> = HashMap::new();
            for ch in contents.iter() {
                unigrams
                    .entry(*ch)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }

            for pairs in contents.windows(2) {
                if pairs[0] == pairs[1] || pairs[0] == ' ' || pairs[1] == ' ' {
                    continue;
                }

                bigrams
                    .entry((pairs[0], pairs[1]))
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }

            (unigrams, bigrams)
        }));
    }

    let mut unigrams: HashMap<char, u32> = HashMap::new();
    let mut bigrams: HashMap<(char, char), u32> = HashMap::new();

    for thread in counters {
        let (file_unigrams, file_bigrams) = thread.join().unwrap();

        for (ch, count) in file_unigrams {
            unigrams
                .entry(ch)
                .and_modify(|counter| *counter += count)
                .or_insert(count);
        }

        for (pair, count) in file_bigrams {
            bigrams
                .entry(pair)
                .and_modify(|counter| *counter += count)
                .or_insert(count);
        }
    }

    println!("Unigrams: {}", unigrams.len());
    println!("Bigrams: {}", bigrams.len());
}
