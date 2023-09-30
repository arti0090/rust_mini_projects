use std::io;
use rand::seq::SliceRandom;

pub struct Hanger {
    words: Vec<&'static str>,
    random_word: String,
    user_word: Vec<char>,
    lives: i8,
}

impl Hanger {
    pub fn new() -> Hanger {
        let words = vec!["car", "cat", "programming", "computer"];
        let user_word = Vec::new();
        let lives = 3;

        return Hanger {
            words,
            random_word: String::new(),
            user_word,
            lives,
        }
    }

    pub(crate) fn play(&mut self) {
        println!("Start game");

        let mut rng = rand::thread_rng();
        self.random_word = self.words.choose(&mut rng).unwrap().to_string();

        self.user_word = vec!['_'; self.random_word.len()];

        println!("The word you are looking for has {} characters", self.random_word.len());
        println!("You have {} lives left", self.lives);

        while !self.game_ended() && self.lives > 0 {
            let normal_string: String = self.user_word.iter().collect();
            println!("{}", normal_string);
            println!();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            self.has_letter_in_random_word(input.chars().next().unwrap());
        }

        if self.lives == 0 {
            println!("You lost :(");
        } else {
            println!("You won!");
        }
    }

    fn has_letter_in_random_word(&mut self, input_letter: char) {
        let mut letter_found = false;

        for (index, char_in_word) in self.random_word.chars().enumerate() {
            if char_in_word == input_letter {
                self.user_word[index] = input_letter;
                letter_found = true;
            }
        }

        if !letter_found {
            println!("There is no letter like this in this word :(");
            self.lives -= 1;
            println!("You have {} lives left.", self.lives);
        }
    }

    fn game_ended(&self) -> bool {
        self.user_word == self.random_word.chars().collect::<Vec<_>>()
    }
}