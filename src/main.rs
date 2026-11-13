use std::{fs, println};

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::{
    Commands::{Solve, Test},
    LetterReq::{Black, Green},
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // /// Selects the dictionary to use (default: <inner>)
    // #[arg(short, long, value_name = "FILE")]
    // dict: Option<String>,

    // Selects the drawing to validate against
    #[arg(short, long, value_name = "FILE", default_value_t = "examples/67.in".to_owned())]
    drawing: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solves the drawing for a specific Wordle answer
    Solve {
        #[arg(required = true)]
        wordle_answer: String,
    },
    /// Checks all words for compatibility with the drawing and returns a count
    Test,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LetterReq {
    Black,
    Green,
}

impl TryFrom<char> for LetterReq {
    type Error = ();

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '-' => Ok(Black),
            'G' => Ok(Green),
            _ => Err(()),
        }
    }
}

type WordPattern = [LetterReq; 5];
type Drawing = Vec<WordPattern>;

lazy_static! {
    static ref WORDLIST: Vec<String> = include_str!("valid-wordle-words.txt")
        .lines()
        .map(|s| s.to_string())
        .collect_vec();
}
fn main() {
    let cli = Cli::parse();

    let drawing_strings: Vec<Vec<LetterReq>> = fs::read_to_string(cli.drawing)
        .expect("invalid drawing path!")
        .lines()
        .map(|s| {
            s.chars()
                .map(LetterReq::try_from)
                .map(|res| res.expect("incorrect character"))
                .collect()
        })
        .collect();

    assert!(drawing_strings.iter().all(|v| v.len() == 5));

    let drawing: Drawing = drawing_strings
        .iter()
        .map(|v| [v[0], v[1], v[2], v[3], v[4]])
        .collect();

    match &cli.command {
        Solve { wordle_answer } => {
            assert!(wordle_answer.len() == 5);

            let valid_words = match_drawing(&drawing, wordle_answer.as_str());

            println!("Found matches:");
            for v in valid_words {
                println!("{v:?}");
            }
        }
        Test => {
            let valid_count = WORDLIST
                .iter()
                .filter(|w| match_drawing(&drawing, &w).iter().all(|v| v.len() >= 1))
                .count();
            println!(
                "possible words: {}/{} ({}%)",
                valid_count,
                WORDLIST.len(),
                f64::trunc((valid_count as f64 / WORDLIST.len() as f64) * 100.0)
            );
        }
    }
}

fn match_pattern(reqs: &WordPattern, needle: &str, wordle_answer: &str) -> bool {
    let all_match =
        itertools::izip!(reqs, needle.chars(), wordle_answer.chars()).all(|t| match t {
            (Green, c1, c2) => c1 == c2,
            (Black, c1, c2) => c1 != c2,
        });

    let no_yellow =
        itertools::izip!(reqs, needle.chars(), wordle_answer.chars()).all(|t| match t {
            (Black, c1, _) => reqs
                .iter()
                .zip(wordle_answer.chars())
                .all(|(&req2, c2)| (req2 == Black && c1 != c2) || (req2 == Green)),
            _ => true,
        });

    all_match && no_yellow
}

fn match_drawing<'a>(drawing: &'a Drawing, wordle_answer: &'a str) -> Vec<Vec<&'a String>> {
    drawing
        .iter()
        .map(|pat| {
            WORDLIST
                .iter()
                .filter(|w| match_pattern(pat, w.as_str(), wordle_answer))
                .collect_vec()
        })
        .collect_vec()
}
