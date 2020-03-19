mod args;

use animur::murmur32;
use args::{Args, Command};
use structopt::StructOpt;

use std::fmt::Display;

struct LogVec<A: Display>(Vec<A>);

impl<A: Display> LogVec<A> {
    fn new() -> Self {
        LogVec(Vec::<A>::new())
    }

    fn push(&mut self, result: A) {
        println!("{}", result);
        self.0.push(result);
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn main() {
    match Args::from_args().cmd {
        Command::Calc(calc) => {
            println!("0x{:08x}", murmur32(calc.word.as_bytes()));
        }
        Command::Reverse(rev) => {
            let mut mutator = Vec::<u8>::with_capacity(rev.max_length as usize);
            let mut length = 1;
            let mut results = LogVec::<String>::new();

            while length <= rev.max_length {
                if rev.capitalize {
                    iter_hash_capitalized(
                        &mut mutator,
                        rev.hex_value,
                        &murmur32,
                        length,
                        &rev.alphabet,
                        &mut results,
                    );
                } else {
                    iter_hash(
                        &mut mutator,
                        rev.hex_value,
                        &murmur32,
                        length,
                        &rev.alphabet,
                        &mut results,
                    );
                }

                length += 1;
            }

            if results.len() == 0 {
                println!("No results");
            }
        }
    }
}

fn iter_hash<Hasher>(
    current: &mut Vec<u8>,
    target: u32,
    hasher: &Hasher,
    max_len: usize,
    alphabet: &str,
    results: &mut LogVec<String>,
) where
    Hasher: Fn(&[u8]) -> u32,
{
    let current_hash = hasher(current);
    if current.len() < max_len {
        for &c in alphabet.as_bytes().iter() {
            current.push(c);
            iter_hash(current, target, hasher, max_len, alphabet, results);
            current.pop();
        }
    } else if current_hash == target {
        results.push(String::from_utf8(current.clone()).unwrap());
    }
}

fn iter_hash_capitalized<Hasher>(
    current: &mut Vec<u8>,
    target: u32,
    hasher: &Hasher,
    max_len: usize,
    alphabet: &str,
    results: &mut LogVec<String>,
) where
    Hasher: Fn(&[u8]) -> u32,
{
    let current_hash = hasher(current);
    if current.len() < max_len {
        for &c in alphabet.to_uppercase().as_bytes().iter() {
            current.push(c);
            iter_hash(current, target, hasher, max_len, alphabet, results);
            current.pop();
        }
    } else if current_hash == target {
        results.push(String::from_utf8(current.clone()).unwrap());
    }
}
