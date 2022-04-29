/*create to generate random numbers*/
extern crate rand;
use rand::Rng;

/*imports from rust standard librart to read files */
use std::fs::File;
use std::io::prelude::*;

/*Reading user input  */
use std::io;

#[derive(Debug)]
pub struct WordLetter {
    letter: char,
    is_revealed: bool,
}

impl WordLetter {
    //reading data in the file an returning a string
    fn read_content_from_file() -> String {
        let mut file_data = String::new();
        let mut file = File::open("words.txt").expect("failed to open file ");
        file.read_to_string(&mut file_data)
            .expect("failed to read content");

        // converting file data to single word
        fn get_words_from_file_content(file_data: &mut String) -> String {
            let words: Vec<&str> = file_data.split(',').collect();
            let random_index = rand::thread_rng().gen_range(0..words.len());
            return String::from(words[random_index]);
        }
        return get_words_from_file_content(&mut file_data);
    }

    /*this a function to crete a word with a character struct */
    fn create_word_char(word: &String) -> Vec<Self> {
        let mut letters: Vec<Self> = Vec::new();

        for c in word.chars() {
            letters.push(Self {
                letter: c,
                is_revealed: false,
            })
        }
        letters
    }

    /*this functiond displays the progress of the letters based on the Vec<WordLetter> //
    example = _ _ j _ n _
    */
    fn display_progress(letters: &mut Vec<Self>) -> String {
        let mut showing_word = String::from("");
        for l in letters {
            showing_word.push(' ');
            if l.is_revealed {
                showing_word.push(l.letter);
            } else {
                showing_word.push('-')
            }
        }
        showing_word
    }

    //getting user input
    fn get_char_from_user() -> char {
        let mut user_char = String::new();
        io::stdin().read_line(&mut user_char).expect("!");
        user_char.chars().next().expect("!")
    }
}

fn main() {
    //reading the word from the file
    let generated_word = WordLetter::read_content_from_file();
    //generating characters from the file word
    let mut word_char = WordLetter::create_word_char(&generated_word);
    //lengeth of number of  turns available
    let mut turns_availble: u32 = word_char.len() as u32;

    loop {
        if turns_availble == 0 {
            println!("Exhaused Turns!!!!");
        } else {
            println!("Turns available {}", turns_availble);
        }

        let display_word = WordLetter::display_progress(&mut word_char);
        //checks if all characters have been revealed
        let is_valid = word_char
            .iter()
            .zip(word_char.iter())
            .all(|(c, _)| c.is_revealed);

        if is_valid {
            println!("Congrats! you nailed it ");
            break;
        }
        let mut at_least_one_char_revealed = true;
        if turns_availble >= 1 {
            println!("Please enter a character ");
            println!("progress : {}", display_word);
            let user_char = WordLetter::get_char_from_user();

            for l in word_char.iter_mut() {
                if l.letter == user_char {
                    l.is_revealed = true;
                    at_least_one_char_revealed = false;
                }
            }
            if user_char == '!' {
                break;
            }
        }

        if turns_availble < 1 {
            println!("Sorry failed to figure out the word ");
            break;
        }
        if at_least_one_char_revealed {
            turns_availble -= 1;
        }
    }

    println!("The word is {}", generated_word);
}
