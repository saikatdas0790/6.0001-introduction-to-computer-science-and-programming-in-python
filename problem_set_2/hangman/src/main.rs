use hangman::{choose_word, hangman, load_words};

fn main() {
    let word_list = load_words();
    let chosen_word = choose_word(word_list);
    hangman(chosen_word);
}
