use clap::Parser;
use rand::prelude::*;
use std::collections::HashSet;

fn str_to_vec(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn is_left_hand(c: char) -> bool {
    "qwertasdfgzxcvb".contains(c)
}

fn is_right_hand(c: char) -> bool {
    "yuiophjklnm".contains(c)
}

struct English {
    vowels: Vec<char>,
    consonants: Vec<char>,
}

impl Default for English {
    fn default() -> Self {
        Self {
            vowels: str_to_vec("aeiou"),
            consonants: str_to_vec("qwrtypsdfghjklzxcvbnm"),
        }
    }
}

impl English {
    fn gen_word(&self, min: usize, max: usize) -> String {
        let mut rng = rand::rng();
        let total_syl = rng.random_range(min..=max);

        let mut word = String::new();
        for _ in 0..total_syl {
            word.push_str(&self.gen_syl(&mut rng));
        }
        word
    }
    fn gen_syl(&self, rng: &mut ThreadRng) -> String {
        match rng.random_range(0..3) {
            0 => format!("{}{}", self.gen_cons(rng), self.gen_vow(rng)),
            1 => format!("{}{}", self.gen_vow(rng), self.gen_cons(rng)),
            2 => format!(
                "{}{}{}",
                self.gen_cons(rng),
                self.gen_vow(rng),
                self.gen_cons(rng)
            ),
            _ => unreachable!(),
        }
    }
    fn gen_vow(&self, rng: &mut ThreadRng) -> char {
        *self.vowels.choose(rng).unwrap()
    }
    fn gen_cons(&self, rng: &mut ThreadRng) -> char {
        *self.consonants.choose(rng).unwrap()
    }
}

struct Japanese {
    syllables: Vec<&'static str>,
    not_for_start: Vec<&'static str>,
}

impl Default for Japanese {
    fn default() -> Self {
        let all: Vec<&'static str> = HashSet::from([
            "a", "i", "u", "e", "o", "ka", "ki", "ku", "ke", "ko", "sa", "shi", "su", "se", "so",
            "ta", "chi", "tsu", "te", "to", "na", "ni", "nu", "ne", "no", "ha", "hi", "fu", "he",
            "ho", "ma", "mi", "mu", "me", "mo", "ya", "yu", "yo", "ra", "ri", "ru", "re", "ro",
            "wa", "wo", "n",
            "ga", "gi", "gu", "ge", "go", "za", "ji", "zu", "ze", "zo", "da", "ji", "zu", "de",
            "do",
            "ba", "bi", "bu", "be", "bo",
            "pa", "pi", "pu", "pe", "po",
            "kya", "kyu", "kyo", "sha", "shu", "sho", "cha", "chu", "cho", "nya", "nyu", "nyo",
            "hya", "hyu", "hyo", "mya", "myu", "myo", "rya", "ryu", "ryo", "gya", "gyu", "gyo",
            "ja", "ju", "jo", "bya", "byu", "byo", "pya", "pyu", "pyo",

            "kka", "kki", "kku", "kke", "kko", "ssa", "sshi", "ssu", "sse", "sso", "tta", "tti",
            "ttu", "tte", "tto", "ppa", "ppi", "ppu", "ppe", "ppo",
        ]).into_iter().collect();
        let not_for_start = vec![
            "kka", "kki", "kku", "kke", "kko", "ssa", "sshi", "ssu", "sse", "sso", "tta", "tti",
            "ttu", "tte", "tto", "ppa", "ppi", "ppu", "ppe", "ppo",
        ];
        Self {
            syllables: all,
            not_for_start
        }
    }
}

impl Japanese {
    fn gen_word(&self, min: usize, max: usize) -> String {
        let mut rng = rand::rng();
        let total_syl = rng.random_range(min..=max);

        let mut word = String::from(self.gen_start_syl(&mut rng));
        for _ in 1..total_syl {
            word.push_str(self.gen_syl(&mut rng));
        }

        word
    }
    fn gen_syl(&self, rng: &mut ThreadRng) -> &str {
        self.syllables.choose(rng).unwrap()
    }
    fn gen_start_syl(&self, rng: &mut ThreadRng) -> &str {
        loop {
            let res = self.syllables.choose(rng).unwrap();
            if !self.not_for_start.contains(res) {
                return res
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    English,
    Japanese,
}

impl<S: AsRef<str>> From<S> for Language {
    fn from(s: S) -> Self {
        let s = s.as_ref().to_ascii_lowercase();
        match s.as_str() {
            "english" => Language::English,
            "japanese" => Language::Japanese,
            _ => panic!("Provided language not supported!"),
        }
    }
}

#[derive(Debug, Parser)]
struct OnamerConfig {
    #[clap(long = "min", default_value = "2")]
    min_syl: usize,
    #[clap(long = "max", default_value = "3")]
    max_syl: usize,

    #[clap(short = 'c', long = "count", default_value = "10")]
    word_count: usize,

    #[clap(short = 'L', long, default_value = "english")]
    language: Language,

    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long)]
    quiet: bool,
}

impl OnamerConfig {
    fn print_info(&self) {
        println!("INFO:");
        println!("  language: {:?}", self.language);
        println!("  word_count: {}w", self.word_count);
        println!(
            "  syllable range: from {} to {}",
            self.min_syl, self.max_syl
        );
        println!();
    }
}

fn main() {
    let config = OnamerConfig::parse();
    if config.verbose {
        config.print_info()
    }

    match config.language {
        Language::English => {
            let eng = English::default();
            println!("GENERAGE:");
            for _ in 0..config.word_count {
                let word = eng.gen_word(config.min_syl, config.max_syl);
                println!(" * {word}");
            }
        }
        Language::Japanese => {
            let jp = Japanese::default();
            println!("GENERAGE:");
            for _ in 0..config.word_count {
                let word = jp.gen_word(config.min_syl, config.max_syl);
                println!(" * {word}");
            }

        },
    }
}
