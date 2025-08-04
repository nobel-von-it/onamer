use clap::Parser;
use rand::{rngs::ThreadRng, seq::IndexedRandom};

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

const CONSONANTS: &[char] = &[
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
    'z',
];

const CONSONANT_FREQ: &[char] = &[
    's', 'c', 'p', 'd', 't', 'b', 'f', 'g', 'h', 'l', 'm', 'n', 'r', 'v', 'w',
];
const CLUSTERS_START: &[&str] = &[
    "pl", "pr", "bl", "br", "tr", "st", "sh", "ch", "th", "sk", "fl",
];

fn rng_char(source: &[char]) -> char {
    *source.get(rand::random_range(0..source.len())).unwrap()
}

fn rng_str(source: &[&str]) -> String {
    source
        .get(rand::random_range(0..source.len()))
        .unwrap()
        .to_string()
}

fn gen_syl(is_start: bool, needed_chars: Option<&str>) -> String {
    match rand::random_range(0..3) {
        // CV
        0 => {
            if is_start {
                format!("{}{}", rng_str(CLUSTERS_START), rng_char(VOWELS))
            } else {
                format!("{}{}", rng_char(CONSONANTS), rng_char(VOWELS))
            }
        }
        // VC
        1 => {
            format!("{}{}", rng_char(VOWELS), rng_char(CONSONANTS))
        }
        // CVC
        _ => {
            if is_start {
                format!(
                    "{}{}{}",
                    rng_str(CLUSTERS_START),
                    rng_char(VOWELS),
                    rng_char(CONSONANTS)
                )
            } else {
                format!(
                    "{}{}{}",
                    rng_char(CONSONANTS),
                    rng_char(VOWELS),
                    rng_char(CONSONANTS)
                )
            }
        }
    }
}

fn gen_word(syl_size: usize, needed_chars: Option<&str>) -> String {
    let mut is_start = matches!(rand::random_range(0..2), 0);
    (0..syl_size)
        .map(|_| {
            let syl = gen_syl(is_start, needed_chars);
            is_start = false;
            syl
        })
        .collect()
}

#[derive(Debug, Parser)]
struct Config {
    #[clap(short, long, default_value = "2")]
    syl_size: usize,
    #[clap(short, long, default_value = "10")]
    word_count: usize,
    #[clap(short = 'N', long, default_value = "false")]
    handful: bool,

    #[clap(short, long)]
    needed_chars: Option<String>,
}

fn main() {
    let config = Config::parse();

    for _ in 0..config.word_count {
        let word = gen_word(config.syl_size, config.needed_chars.as_deref());
        println!("{word}");
    }
}
