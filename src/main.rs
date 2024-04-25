use std::ops::Rem;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Content of the ciphertext
    #[arg(short, long)]
    text: String,

    /// Key to use for the permutation. Must be a string of numbers
    #[arg(short, long)]
    key: String,

    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// Decrypt the message
    #[arg(short, long)]
    decrypt: bool,
}

// Do a permutation cypher
fn main() {
    let args = Args::parse();

    let encryption_key: Vec<usize> = args
        .key
        .as_bytes()
        .iter()
        .map(|letter| (letter - 48) as usize)
        .collect::<Vec<usize>>();

    let decryption_key = encryption_key.iter().enumerate().fold(
        vec![0; encryption_key.len()],
        |mut acc, (index, value)| {
            acc[*value as usize] = index.into();
            acc
        },
    );

    let key = if args.encrypt {
        encryption_key
    } else {
        decryption_key
    };

    let mut input = args.text.chars().collect::<Vec<_>>();
    let padding_length = (key.len() - input.len().rem(key.len())).rem(key.len());
    input.extend(std::iter::repeat('_').take(padding_length));

    let output: String = input
        .chunks_exact(key.len())
        .into_iter()
        .flat_map(|unit| (0..key.len()).map(|index| unit[key[index]]))
        .collect::<String>();

    println!("{:?}", output);
}
