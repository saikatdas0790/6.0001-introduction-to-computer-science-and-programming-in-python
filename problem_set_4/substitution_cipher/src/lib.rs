#![allow(dead_code)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use permutation::get_permutations;

fn load_words(file_name: String) -> Vec<String> {
    // file_name (string): the name of the file containing
    // the list of words to load
    // Returns: a list of valid words. Words are strings of lowercase letters.

    // Depending on the size of the word list, this function may
    // take a while to finish.
    let mut words: Vec<String> = Vec::new();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line_words: Vec<String> = line
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        words.extend(line_words);
    }
    words
}

#[cfg(test)]
#[test]
fn test_load_words() {
    let words = load_words("../words.txt".to_string());
    assert_eq!(words.len(), 55901);
}

fn is_word(word_list: &[String], word: String) -> bool {
    // Determine if word is a valid word, ignoring
    // capitalization and punctuation
    // word_list (list): list of words in the dictionary.
    // word (string): a possible word.
    // Returns: True if word is in word_list, False otherwise
    let word = word.to_lowercase();
    word_list.contains(&word)
}

#[cfg(test)]
#[test]
fn test_is_word() {
    let words = load_words("../words.txt".to_string());
    assert!(is_word(&words, "bat".to_string()));
    assert!(!is_word(&words, "asdf".to_string()));
}

const VOWELS_LOWER: &str = "aeiou";
const VOWELS_UPPER: &str = "AEIOU";
const CONSONANTS_LOWER: &str = "bcdfghjklmnpqrstvwxyz";
const CONSONANTS_UPPER: &str = "BCDFGHJKLMNPQRSTVWXYZ";

struct SubMessage {
    message_text: String,
    valid_words: Vec<String>,
}

impl SubMessage {
    pub fn new(text: String) -> Self {
        let valid_words = load_words("../words.txt".to_string());
        Self {
            message_text: text,
            valid_words,
        }
    }

    pub fn get_message_text(&self) -> String {
        self.message_text.clone()
    }

    pub fn get_valid_words(&self) -> Vec<String> {
        self.valid_words.clone()
    }

    pub fn build_transpose_dict(&self, vowels_permutation: &str) -> HashMap<char, char> {
        // vowels_permutation (string): a string containing a permutation of vowels (a, e, i, o, u)

        // Creates a dictionary that can be used to apply a cipher to a letter.
        // The dictionary maps every uppercase and lowercase letter to an
        // uppercase and lowercase letter, respectively. Vowels are shuffled
        // according to vowels_permutation. The first letter in vowels_permutation
        // corresponds to a, the second to e, and so on in the order a, e, i, o, u.
        // The consonants remain the same. The dictionary should have 52
        // keys of all the uppercase letters and all the lowercase letters.

        // Example: When input "eaiuo":
        // Mapping is a->e, e->a, i->i, o->u, u->o
        // and "Hello World!" maps to "Hallu Wurld!"

        // Returns: a dictionary mapping a letter (string) to
        //          another letter (string).
        let mut transpose_dict: HashMap<char, char> = HashMap::new();
        let mut vowels_permutation_chars = vowels_permutation.chars();
        for c in VOWELS_LOWER.chars() {
            transpose_dict.insert(c, vowels_permutation_chars.next().unwrap());
        }
        let mut vowels_permutation_chars = vowels_permutation.chars();
        for c in VOWELS_UPPER.chars() {
            transpose_dict.insert(
                c,
                vowels_permutation_chars
                    .next()
                    .unwrap()
                    .to_ascii_uppercase(),
            );
        }
        for c in CONSONANTS_LOWER.chars() {
            transpose_dict.insert(c, c);
        }
        for c in CONSONANTS_UPPER.chars() {
            transpose_dict.insert(c, c);
        }
        transpose_dict
    }

    pub fn apply_transpose(&self, transpose_dict: HashMap<char, char>) -> String {
        // transpose_dict (dict): a transpose dictionary
        // Returns: an encrypted version of the message text, based
        //   on the dictionary
        let mut encrypted_message = String::new();
        for c in self.message_text.chars() {
            // account for the fact that the text might have other characters
            // that are not in the transpose dict
            encrypted_message.push(*transpose_dict.get(&c).unwrap_or(&c));
        }
        encrypted_message
    }
}

#[cfg(test)]
#[test]
fn test_sub_message() {
    let sub_message = SubMessage::new("Hello World!".to_string());
    assert_eq!(sub_message.get_message_text(), "Hello World!");
    assert_eq!(sub_message.get_valid_words().len(), 55901);
    let transpose_dict = sub_message.build_transpose_dict("eaiuo");
    assert_eq!(transpose_dict.len(), 52);
    assert_eq!(transpose_dict.get(&'a'), Some(&'e'));
    assert_eq!(transpose_dict.get(&'e'), Some(&'a'));
    assert_eq!(transpose_dict.get(&'i'), Some(&'i'));
    assert_eq!(transpose_dict.get(&'o'), Some(&'u'));
    assert_eq!(transpose_dict.get(&'u'), Some(&'o'));
    assert_eq!(transpose_dict.get(&'A'), Some(&'E'));
    assert_eq!(transpose_dict.get(&'E'), Some(&'A'));
    assert_eq!(transpose_dict.get(&'I'), Some(&'I'));
    assert_eq!(transpose_dict.get(&'O'), Some(&'U'));
    assert_eq!(transpose_dict.get(&'U'), Some(&'O'));
    assert_eq!(transpose_dict.get(&'b'), Some(&'b'));
    assert_eq!(transpose_dict.get(&'B'), Some(&'B'));
    assert_eq!(transpose_dict.get(&'c'), Some(&'c'));
    assert_eq!(transpose_dict.get(&'C'), Some(&'C'));
    assert_eq!(transpose_dict.get(&'d'), Some(&'d'));
    assert_eq!(transpose_dict.get(&'D'), Some(&'D'));

    // test apply_transpose
    let encrypted_message = sub_message.apply_transpose(transpose_dict);
    assert_eq!(encrypted_message, "Hallu Wurld!");
}

struct EncryptedSubMessage {
    message_text: String,
    valid_words: Vec<String>,
}

impl EncryptedSubMessage {
    pub fn new(text: String) -> Self {
        let valid_words = load_words("../words.txt".to_string());
        Self {
            message_text: text,
            valid_words,
        }
    }

    pub fn decrypt_message(&self) -> (String, String) {
        // Attempt to decrypt the encrypted message

        // Idea is to go through each permutation of the vowels and test it
        // on the encrypted message. For each permutation, check how many
        // words in the decrypted text are valid English words, and return
        // the decrypted message with the most English words.

        // If no good permutations are found (i.e. no permutations result in
        // at least 1 valid word), return the original string. If there are
        // multiple permutations that yield the maximum number of words, return any
        // one of them.

        // Returns: the best decrypted message

        let mut best_decrypted_message = String::new();
        let mut best_permutation = String::new();
        let mut best_num_valid_words = 0;

        for permutation in get_permutations(VOWELS_LOWER.to_string()) {
            let sub_message = SubMessage::new(self.message_text.clone());
            let transpose_dict = sub_message.build_transpose_dict(&permutation);
            let decrypted_message = sub_message.apply_transpose(transpose_dict);
            let num_valid_words = decrypted_message
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .filter(|s| is_word(&self.valid_words, s.to_string()))
                .count();

            if num_valid_words > best_num_valid_words {
                best_decrypted_message = decrypted_message;
                best_permutation = permutation;
                best_num_valid_words = num_valid_words;
            }
        }

        if best_num_valid_words == 0 {
            (self.message_text.clone(), best_permutation)
        } else {
            (best_decrypted_message, best_permutation)
        }
    }
}

#[cfg(test)]
#[test]
fn test_encrypted_sub_message() {
    let encrypted_sub_message = EncryptedSubMessage::new("Hallu Wurld!".to_string());
    let (decrypted_message, permutation) = encrypted_sub_message.decrypt_message();
    assert_eq!(decrypted_message, "Hello World!");
    assert_eq!(permutation, "eaiuo");
}
