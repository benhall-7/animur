mod args;

use animur::murmur32_2;
use args::{Args, Command};
use structopt::StructOpt;

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    match Args::from_args().cmd {
        Command::Calc(calc) => {
            println!("0x{:08x}", murmur32_2(calc.word.as_bytes(), 0));
        }
        Command::Reverse(rev) => {
            let mut mutator = Vec::<u8>::with_capacity(rev.max_length as usize);
            let mut length = 1;
            let mut results = Vec::<String>::new();
            let targets = HashSet::from_iter(rev.hex_values);

            while length <= rev.max_length {
                if rev.capitalize {
                    iter_hash_capitalized(
                        &mut mutator,
                        &targets,
                        &|data: &[u8]| murmur32_2(data, 0),
                        length,
                        &rev.alphabet,
                        &mut results,
                    );
                } else {
                    iter_hash(
                        &mut mutator,
                        &targets,
                        &|data: &[u8]| murmur32_2(data, 0),
                        length,
                        &rev.alphabet,
                        &mut results,
                    );
                }

                length += 1;
            }

            if results.is_empty() {
                println!("No results");
            }
        }
    }
}

fn iter_hash<Hasher>(
    current: &mut Vec<u8>,
    targets: &HashSet<u32>,
    hasher: &Hasher,
    max_len: usize,
    alphabet: &str,
    results: &mut Vec<String>,
) where
    Hasher: Fn(&[u8]) -> u32,
{
    let current_hash = hasher(current);
    if current.len() < max_len {
        for &c in alphabet.as_bytes().iter() {
            current.push(c);
            iter_hash(current, targets, hasher, max_len, alphabet, results);
            current.pop();
        }
    } else if targets.contains(&current_hash) {
        println!("0x{:08x}: {}", current_hash, String::from_utf8_lossy(current));
        results.push(String::from_utf8(current.clone()).unwrap());
    }
}

fn iter_hash_capitalized<Hasher>(
    current: &mut Vec<u8>,
    targets: &HashSet<u32>,
    hasher: &Hasher,
    max_len: usize,
    alphabet: &str,
    results: &mut Vec<String>,
) where
    Hasher: Fn(&[u8]) -> u32,
{
    let current_hash = hasher(current);
    if current.len() < max_len {
        for &c in alphabet.to_uppercase().as_bytes().iter() {
            current.push(c);
            iter_hash(current, targets, hasher, max_len, alphabet, results);
            current.pop();
        }
    } else if targets.contains(&current_hash) {
        println!("0x{:08x}: {}", current_hash, String::from_utf8_lossy(current));
        results.push(String::from_utf8(current.clone()).unwrap());
    }
}
