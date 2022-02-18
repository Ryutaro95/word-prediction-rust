use proconio::input;
use proconio::marker::Chars;
use ansi_escapes::{EraseLines};
use colored::*;

#[derive(Debug)]
struct Word(Vec<char>);

impl Word {
    fn print_diff(&self, answer: &Word) {
        for (i, char) in self.0.iter().enumerate() {
            let maybe_correct_char = answer.0.get(i);
            match maybe_correct_char {
                Some(correct_char) => {
                    if char == correct_char {
                        print!("{}", char.to_string().green());
                    } else if answer.0.contains(char) {
                        print!("{}", char.to_string().yellow());
                    } else {
                        print!("{}", char);
                    }
                }
                None => {}
            }
        }
        println!();
    }

    fn eq(&self, other: &Word) -> bool {
        self.0 == other.0
    }
}

fn main() {
    let answer = Word(['w', 'o', 'r', 'l', 'd'].to_vec());
    let mut correct = false;

    for _i in 0..6 {
        input! {
            chars: Chars,
        }
        let input = Word(chars);

        print!("{}", EraseLines(2));
        input.print_diff(&answer);
        if input.eq(&answer) {
            println!("Genius");
            correct = true;
            break;
        }
        // println!("{:?}", input);
    }

    if !correct {
        println!("{}", answer.0.into_iter().collect::<String>());
    }

    // print!("{}", EraseLines(2));

}
