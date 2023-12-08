#![allow(dead_code)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

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

struct Message {
    message_text: String,
    valid_words: Vec<String>,
}

impl Message {
    pub fn new(text: String) -> Self {
        Message {
            message_text: text,
            valid_words: load_words("../words.txt".to_string()),
        }
    }

    pub fn get_message_text(&self) -> &String {
        &self.message_text
    }

    pub fn get_valid_words(&self) -> &Vec<String> {
        &self.valid_words
    }

    pub fn build_shift_dict(&self, shift: i32) -> HashMap<char, char> {
        // Creates a dictionary that can be used to apply a cipher to a letter.
        // The dictionary maps every uppercase and lowercase letter to a
        // character shifted down the alphabet by the input shift. The dictionary
        // should have 52 keys of all the uppercase letters and all the lowercase
        // letters only. Letters unaffected by the cipher should remain
        // unaltered.
        // shift (integer): the amount by which to shift every letter of the
        // alphabet. 0 <= shift < 26
        // Returns: a dictionary mapping a letter (string) to
        // another letter (string).
        let mut shift_dict: HashMap<char, char> = HashMap::new();
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        for (i, c) in alphabet.chars().enumerate() {
            let shifted_index = (i as i32 + shift) % 26;
            let shifted_char = alphabet.chars().nth(shifted_index as usize).unwrap();
            shift_dict.insert(c, shifted_char);
            shift_dict.insert(
                c.to_uppercase().next().unwrap(),
                shifted_char.to_uppercase().next().unwrap(),
            );
        }
        shift_dict
    }
}

#[cfg(test)]
#[test]
fn test_build_shift_dict() {
    let message = Message::new("hello".to_string());
    let shift_dict = message.build_shift_dict(2);
    assert_eq!(shift_dict.get(&'a'), Some(&'c'));
    assert_eq!(shift_dict.get(&'A'), Some(&'C'));
    assert_eq!(shift_dict.get(&'z'), Some(&'b'));
    assert_eq!(shift_dict.get(&'Z'), Some(&'B'));
    assert_eq!(shift_dict.get(&' '), None);
    assert_eq!(shift_dict.get(&'!'), None);
}

struct PlaintextMessage {
    message_text: String,
    message: Message,
    valid_words: Vec<String>,
    shift: i32,
    encryption_dict: HashMap<char, char>,
    message_text_encrypted: String,
}

impl PlaintextMessage {
    pub fn new(text: String, shift: i32) -> Self {
        // Initializes a PlaintextMessage object
        // text (string): the message's text
        // shift (integer): the cipher's shift value
        // Returns: a PlaintextMessage object
        let message = Message::new(text);
        let encryption_dict = message.build_shift_dict(shift);
        let message_text_encrypted = message
            .get_message_text()
            .chars()
            .map(|c| encryption_dict.get(&c).unwrap_or(&c).to_owned())
            .collect();
        PlaintextMessage {
            message_text: message.get_message_text().to_owned(),
            valid_words: message.get_valid_words().to_owned(),
            message,
            shift,
            encryption_dict,
            message_text_encrypted,
        }
    }

    pub fn get_shift(&self) -> i32 {
        // Used to safely access self.shift outside of the class
        self.shift
    }

    pub fn get_encryption_dict(&self) -> &HashMap<char, char> {
        // Used to safely access a copy self.encryption_dict outside of the class
        &self.encryption_dict
    }

    pub fn get_message_text_encrypted(&self) -> &String {
        &self.message_text_encrypted
    }

    pub fn change_shift(&mut self, shift: i32) {
        // Changes self.shift of the PlaintextMessage and updates other
        // attributes determined by shift (ie. self.message_text_encrypted).
        // shift (integer): the new shift that should be associated with this
        // message.
        // Returns: nothing
        self.shift = shift;
        self.encryption_dict = self.message.build_shift_dict(shift);
        self.message_text_encrypted = self
            .message_text
            .chars()
            .map(|c| self.encryption_dict.get(&c).unwrap_or(&c).to_owned())
            .collect();
    }
}

#[cfg(test)]
#[test]
fn test_plaintext_message() {
    let mut message = PlaintextMessage::new("hello".to_string(), 2);
    assert_eq!(message.get_shift(), 2);
    assert_eq!(message.get_message_text_encrypted(), "jgnnq");
    message.change_shift(3);
    assert_eq!(message.get_shift(), 3);
    assert_eq!(message.get_message_text_encrypted(), "khoor");

    let mut message = PlaintextMessage::new("hello".to_string(), 0);
    assert_eq!(message.get_shift(), 0);
    assert_eq!(message.get_message_text_encrypted(), "hello");
    message.change_shift(1);
    assert_eq!(message.get_shift(), 1);
    assert_eq!(message.get_message_text_encrypted(), "ifmmp");

    let mut message = PlaintextMessage::new("hello".to_string(), 26);
    assert_eq!(message.get_shift(), 26);
    assert_eq!(message.get_message_text_encrypted(), "hello");
    message.change_shift(1);
    assert_eq!(message.get_shift(), 1);
    assert_eq!(message.get_message_text_encrypted(), "ifmmp");

    let mut message = PlaintextMessage::new("hello".to_string(), 27);
    assert_eq!(message.get_shift(), 27);
    assert_eq!(message.get_message_text_encrypted(), "ifmmp");
    message.change_shift(1);
    assert_eq!(message.get_shift(), 1);
    assert_eq!(message.get_message_text_encrypted(), "ifmmp");
}

struct CiphertextMessage {
    message_text: String,
    valid_words: Vec<String>,
}

impl CiphertextMessage {
    pub fn new(text: String) -> Self {
        // Initializes a CiphertextMessage object
        // text (string): the message's text
        // Returns: a CiphertextMessage object
        let message = Message::new(text);
        CiphertextMessage {
            message_text: message.get_message_text().to_owned(),
            valid_words: message.get_valid_words().to_owned(),
        }
    }

    pub fn decrypt_message(&self) -> (i32, String) {
        // Decrypt self.message_text by trying every possible shift value
        // and find the "best" one. We will define "best" as the shift that
        // creates the maximum number of real words when we use apply_shift(shift)
        // on the message text. If s is the original shift value used to encrypt
        // the message, then we would expect 26 - s to be the best shift value
        // for decrypting it.
        // Note: if multiple shifts are equally good such that they all create
        // the maximum number of you may choose any of those shifts (and their
        // corresponding decrypted messages) to return
        // Returns: a tuple of the best shift value used to decrypt the message
        // and the decrypted message text using that shift value
        let mut best_shift = 0;
        let mut best_shift_count = 0;
        let mut best_shift_message = String::new();
        for shift in 0..26 {
            let message = PlaintextMessage::new(self.message_text.to_owned(), shift);
            let message_text_decrypted = message.get_message_text_encrypted();
            let message_words: Vec<&str> = message_text_decrypted.split_whitespace().collect();
            let mut valid_word_count = 0;
            for word in message_words {
                if is_word(&self.valid_words, word.to_string()) {
                    valid_word_count += 1;
                }
            }
            if valid_word_count > best_shift_count {
                best_shift = shift;
                best_shift_count = valid_word_count;
                best_shift_message = message_text_decrypted.to_owned();
            }
        }
        (best_shift, best_shift_message)
    }
}

#[cfg(test)]
#[test]
fn test_ciphertext_message() {
    let message = CiphertextMessage::new("hello".to_string());
    let (shift, message_text_decrypted) = message.decrypt_message();
    assert_eq!(shift, 0);
    assert_eq!(message_text_decrypted, "hello");

    let message = CiphertextMessage::new("ifmmp".to_string());
    let (shift, message_text_decrypted) = message.decrypt_message();
    assert_eq!(shift, 25);
    assert_eq!(message_text_decrypted, "hello");

    let message = CiphertextMessage::new("jgnnq".to_string());
    let (shift, message_text_decrypted) = message.decrypt_message();
    assert_eq!(shift, 24);
    assert_eq!(message_text_decrypted, "hello");

    let message = CiphertextMessage::new("khoor".to_string());
    let (shift, message_text_decrypted) = message.decrypt_message();
    assert_eq!(shift, 23);
    assert_eq!(message_text_decrypted, "hello");
}
