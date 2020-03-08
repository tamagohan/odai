use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result;

fn usage() {
    println!("anagram WORD DIC_FILE_PATH")
}

struct Anagram {
    map: HashMap<String, Vec<String>>,
}

impl Anagram {
    pub fn new(word_file_path: &str) -> Result<Self, &str> {
        let mut anagram = Self {
            map: HashMap::new(),
        };
        let f = match File::open(&word_file_path) {
            Ok(file) => file,
            Err(_) => return Err("An error occured while opening file."),
        };
        let bf = BufReader::new(f);
        for line in bf.lines() {
            match line {
                Ok(line) => anagram.add(&line),
                Err(_) => return Err("An error occured while reading a line."),
            };
        }
        return Ok(anagram);
    }

    pub fn add(&mut self, word: &str) {
        let sorted_word = Anagram::sort(word);
        match self.map.get_mut(&sorted_word) {
            Some(values) => {
                values.push(word.to_string());
            }
            None => {
                self.map.insert(sorted_word, vec![word.to_string()]);
            }
        };
        return;
    }

    pub fn search(&self, word: &str) -> Option<&Vec<String>> {
        let sorted_word = Anagram::sort(word);
        return self.map.get(&sorted_word);
    }

    fn sort(word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        return chars.into_iter().collect::<String>();
    }
}

fn main() {
    let word = match env::args().nth(1) {
        Some(word) => word,
        None => {
            usage();
            return;
        }
    };

    let dic_file_path = match env::args().nth(2) {
        Some(dic_file_path) => dic_file_path,
        None => "/usr/share/dict/words".to_string(),
    };
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
    assert_eq!(anagram.search("a"), Some(&vec!("a".to_string())));
    assert_eq!(
        anagram.search("aab"),
        Some(&vec!["aba".to_string(), "baa".to_string()])
    );
    assert_eq!(anagram.search("nonexists"), None);
}
