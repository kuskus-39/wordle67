use std::println;

use itertools::Itertools;

use crate::LetterReq::{Black, Green};

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

fn main() {
    #[allow(non_snake_case)]
    let WORDLIST = include_str!("valid-wordle-words.txt")
        .lines()
        .map(|s| s.to_string())
        .collect_vec();

    let pattern_strings: Vec<Vec<LetterReq>> = include_str!("pattern.in")
        .lines()
        .map(|s| {
            s.chars()
                .map(LetterReq::try_from)
                .map(|res| res.expect("incorrect character"))
                .collect()
        })
        .collect();

    assert!(pattern_strings.iter().all(|v| v.len() == 5));

    let drawing: Drawing = pattern_strings
        .iter()
        .map(|v| [v[0], v[1], v[2], v[3], v[4]])
        .collect();

    // let word: String = get_var("word: ").expect("cannot parse");

    let word = std::env::args()
        .nth(1)
        .expect("provide 1 argument")
        .trim()
        .to_owned()
        .to_ascii_lowercase();

    assert!(word.len() == 5);

    let lists_of_words = drawing
        .iter()
        .map(|pat| {
            WORDLIST
                .iter()
                .filter(|w| match_pattern(pat, w.as_str(), word.as_str()))
                .collect_vec()
        })
        .collect_vec();

    println!("Found matches:");
    for v in lists_of_words {
        println!("{v:?}");
    }
}

// fn get_var<T: FromStr>(prompt: &str) -> Option<T> {
//     println!("{prompt}");
//     let mut buf = String::new();
//     io::stdin().read_line(&mut buf).ok()?;
//     buf.parse().ok()
// }

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
