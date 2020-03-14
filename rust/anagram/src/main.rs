use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::result::Result;

struct Anagram {
    map: HashMap<String, Vec<String>>,
}

impl Anagram {
    pub fn new<P: AsRef<Path>>(word_file_path: P) -> Result<Self, io::Error> {
        let mut anagram = Self {
            map: HashMap::new(),
        };
        let f = File::open(&word_file_path)?;
        let bf = BufReader::new(f);
        for line in bf.lines() {
            let word = line?;
            anagram.add(word);
        }
        return Ok(anagram);
    }

    pub fn search(&self, word: &str) -> Option<&Vec<String>> {
        let sorted_word = Anagram::sort(word);
        return self.map.get(&sorted_word);
    }

    fn add(&mut self, word: String) {
        let sorted_word = Anagram::sort(&word);
        self.map.entry(sorted_word).or_insert(Vec::new()).push(word);
    }

    fn sort(word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        return chars.into_iter().collect::<String>();
    }
}

fn main() {
    let word = env::args().nth(1).expect("USAGE: word");
    let dic_file_path = env::args()
        .nth(2)
        .unwrap_or("/usr/share/dict/words".to_string());
    let anagram = match Anagram::new(&dic_file_path) {
        Ok(anagram) => anagram,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };
    match anagram.search(&word) {
        Some(words) => println!("{} are found.", words.join(",")),
        None => println!("No words found."),
    }
}

#[test]
fn test_gen_anagram() {
    let anagram = Anagram::new("./data/word.txt").unwrap();
    assert_eq!(Some(&vec!("a".to_string())), anagram.search("a"));
    assert_eq!(
        Some(&vec!["aba".to_string(), "baa".to_string()]),
        anagram.search("aab"),
    );
    assert_eq!(anagram.search("nonexists"), None);
}

#[test]
fn test_invalid_file() {
    assert!(Anagram::new("./nonexist/path.txt").is_err());
}
