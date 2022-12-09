use rand::Rng;
use std::{env, path::PathBuf};

pub fn load_words() -> Vec<String> {
    println!("Loading word list from file...");

    // Open words.txt file as string
    let contents = std::fs::read_to_string(
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../words.txt"),
    )
    .expect("Something went wrong reading the words.txt file");

    println!("{} words loaded.", contents.split(" ").count());

    let word_list: Vec<String> = contents.split(" ").map(|s| s.to_string()).collect();

    word_list
}

pub fn choose_word(word_list: Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..word_list.len());

    word_list[random_index].clone()
}

pub fn is_word_guessed(secret_word: &str, letters_guessed: &Vec<char>) -> bool {
    secret_word.chars().all(|c| letters_guessed.contains(&c))
}

pub fn get_guessed_word(secret_word: &str, letters_guessed: &Vec<char>) -> String {
    secret_word
        .chars()
        .map(|c| {
            if letters_guessed.contains(&c) {
                c.to_string()
            } else {
                r"_ ".to_string()
            }
        })
        .collect()
}

pub fn get_available_letters(letters_guessed: &Vec<char>) -> String {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .filter(|c| !letters_guessed.contains(c))
        .collect()
}

pub fn hangman(secret_word: String) {
    println!("Welcome to the game, Hangman!");

    let number_of_guesses = 6;

    println!(
        "I am thinking of a word that is {} letters long.",
        secret_word.len()
    );
    println!("-------------");

    let mut letters_guessed: Vec<char> = vec![];
    let mut number_of_guesses_left = number_of_guesses;
    let mut number_of_warnings_left = 3;

    // while number_of_guesses_left > 0 {
    //     println!("You have {} guesses left.", number_of_guesses_left);
    //     println!(
    //         "Available letters: {}",
    //         get_available_letters(&letters_guessed)
    //     );

    //     let mut guess = String::new();

    //     std::io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess = guess.trim().to_lowercase();

    //     if guess.len() != 1 {
    //         if number_of_warnings_left > 0 {
    //             number_of_warnings_left -= 1;
    //             println!(
    //                 "Oops! That is not a valid letter. You have {} warnings left: {}",
    //                 number_of_warnings_left,
    //                 get_guessed_word(&secret_word, &letters_guessed)
    //             );
    //         } else {
    //             number_of_guesses_left -= 1;
    //             println!("Oops! That is not a valid letter. You have no warnings left so you lose one guess: {}", get_guessed_word(&secret_word, &letters_guessed));
    //         }
    //     } else {
    //         let guess_char = guess.chars().next().unwrap();

    //         if letters_guessed.contains(&guess_char) {
    //             if number_of_warnings_left > 0 {
    //                 number_of_warnings_left -= 1;
    //                 println!(
    //                     "Oops! You've already guessed that letter. You have {} warnings left: {}",
    //                     number_of_warnings_left,
    //                     get_guessed_word(&secret_word, &letters_guessed)
    //                 );
    //             } else {
    //                 number_of_guesses_left -= 1;
    //                 println!("Oops! You've already guessed that letter. You have no warnings left so you lose one guess: {}", get_guessed_word(&secret_word, &letters_guessed));
    //             }
    //         } else {
    //             letters_guessed.push(guess_char);

    //             if secret_word.contains(guess_char) {
    //                 println!(
    //                     "Good guess: {}",
    //                     get_guessed_word(&secret_word, &letters_guessed)
    //                 );
    //             } else {
    //                 println!(
    //                     "Oops! That letter is not in my word: {}",
    //                     get_guessed_word(&secret_word, &letters_guessed)
    //                 );

    //                 if "aeiou".contains(guess_char) {
    //                     number_of_guesses_left -= 2;
    //                 } else {
    //                     number_of_guesses_left -= 1;
    //                 }
    //             }
    //         }
    //     }

    //     println!("-------------");

    //     if is_word_guessed(&secret_word, &letters_guessed) {
    //         println!("Congratulations, you won!");
    //         println!(
    //             "Your total score for this game is: {}",
    //             number_of_guesses_left * secret_word.len()
    //         );
    //         break;
    //     }
    // }
    // vowels lose 2 guesses, consonants lose 1 guess

    while number_of_guesses_left > 0 {
        println!("You have {} guesses left.", number_of_guesses_left);
        println!(
            "Available letters: {}",
            get_available_letters(&letters_guessed)
        );

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_lowercase();

        if guess.len() != 1 {
            if number_of_warnings_left > 0 {
                number_of_warnings_left -= 1;
                println!(
                    "Oops! That is not a valid letter. You have {} warnings left: {}",
                    number_of_warnings_left,
                    get_guessed_word(&secret_word, &letters_guessed)
                );
            } else {
                number_of_guesses_left -= 1;
                println!("Oops! That is not a valid letter. You have no warnings left so you lose one guess: {}", get_guessed_word(&secret_word, &letters_guessed));
            }
        } else {
            let guess_char = guess.chars().next().unwrap();

            if letters_guessed.contains(&guess_char) {
                if number_of_warnings_left > 0 {
                    number_of_warnings_left -= 1;
                    println!(
                        "Oops! You've already guessed that letter. You have {} warnings left: {}",
                        number_of_warnings_left,
                        get_guessed_word(&secret_word, &letters_guessed)
                    );
                } else {
                    number_of_guesses_left -= 1;
                    println!("Oops! You've already guessed that letter. You have no warnings left so you lose one guess: {}", get_guessed_word(&secret_word, &letters_guessed));
                }
            } else {
                letters_guessed.push(guess_char);

                if secret_word.contains(guess_char) {
                    println!(
                        "Good guess: {}",
                        get_guessed_word(&secret_word, &letters_guessed)
                    );
                } else {
                    println!(
                        "Oops! That letter is not in my word: {}",
                        get_guessed_word(&secret_word, &letters_guessed)
                    );

                    if "aeiou".contains(guess_char) {
                        number_of_guesses_left -= 2;
                    } else {
                        number_of_guesses_left -= 1;
                    }
                }
            }
        }

        println!("-------------");

        if is_word_guessed(&secret_word, &letters_guessed) {
            println!("Congratulations, you won!");
            println!(
                "Your total score for this game is: {}",
                number_of_guesses_left * secret_word.len()
            );
            break;
        }

        if number_of_guesses_left == 0 {
            println!(
                "Sorry, you ran out of guesses. The word was {}.",
                secret_word
            );
        }
    }
}

#[allow(unused)]
fn match_with_gaps(my_word: &str, other_word: &str) -> bool {
    let mut my_word_chars = my_word.chars();
    let mut other_word_chars = other_word.chars();

    loop {
        let my_word_char = my_word_chars.next();
        let other_word_char = other_word_chars.next();

        match (my_word_char, other_word_char) {
            (Some('_'), Some(_)) => {
                my_word_chars.next();
                continue;
            }
            (Some(my_char), Some(other_char)) => {
                if my_char != other_char {
                    return false;
                }
            }
            (None, None) => return true,
            _ => return false,
        }
    }
}

#[allow(unused)]
fn show_possible_matches(my_word: &str) -> Vec<String> {
    let word_list = load_words();

    let mut possible_matches = vec![];

    for word in word_list {
        if match_with_gaps(my_word, &word) {
            possible_matches.push(word);
        }
    }

    if possible_matches.len() > 0 {
        println!("Possible word matches are:");
        println!("{}", possible_matches.join(" "));
    } else {
        println!("No matches found.");
    }

    possible_matches
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_words() {
        assert_eq!(load_words().len(), 55900);
    }

    #[test]
    fn test_choose_word() {
        let word_list = load_words();
        let word = choose_word(word_list);

        assert!(word.len() > 0);
    }

    #[test]
    fn test_is_word_guessed() {
        let secret_word = "apple";
        let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];

        assert_eq!(is_word_guessed(secret_word, &letters_guessed), false);

        let letters_guessed = vec!['a', 'p', 'l', 'e'];

        assert_eq!(is_word_guessed(secret_word, &letters_guessed), true);
    }

    #[test]
    fn test_get_guessed_word() {
        let secret_word = "apple";
        let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];

        assert_eq!(get_guessed_word(secret_word, &letters_guessed), "_ pp_ e");
    }

    #[test]
    fn test_get_available_letters() {
        let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];

        assert_eq!(
            get_available_letters(&letters_guessed),
            "abcdfghjlmnoqtuvwxyz"
        );
    }

    #[test]
    fn test_match_with_gaps() {
        assert_eq!(match_with_gaps("te_ t", "tact"), false);
        assert_eq!(match_with_gaps("a_ _ le", "banana"), false);
        assert_eq!(match_with_gaps("a_ _ le", "apple"), true);
        assert_eq!(match_with_gaps("a_ ple", "apple"), true);
    }

    #[test]
    fn test_show_possible_matches() {
        assert_eq!(
            show_possible_matches("t_ _ t"),
            vec![
                "tact", "tart", "taut", "teat", "tent", "test", "text", "that", "tilt", "tint",
                "toot", "tort", "tout", "trot", "tuft", "twit",
            ]
        );
        assert_eq!(show_possible_matches("abbbb_ "), Vec::<String>::new());
        assert_eq!(
            show_possible_matches("a_ pl_ "),
            vec!["ample", "amply", "apple", "apply"]
        );
    }
}
