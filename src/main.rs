use ansi_escapes::EraseLines;
use proconio::input;
use proconio::marker::Chars;

mod word;

const EXPECT_LENGTH: usize = 5;

fn main() {
    // todo: ランダムで回答の単語を出力させる
    let answer = word::Word::new(vec!['h', 'e', 'l', 'l', 'o']);
    let mut correct = false;
    let mut i = 0;
    loop {
        input! {
            chars: Chars,
        }
        if !is_valid_length(chars.len()) {
            println!("Enter a five-character word");
            continue;
        }

        let input = word::Word::new(chars);

        print!("{}", EraseLines(2));
        input.color_diff(&answer);
        if input.eq(&answer) {
            println!("Genius");
            correct = true;
            break;
        }
        if i == 4 {
            break;
        }
        i += 1;
    }

    if !correct {
        show_answer(&answer);
    }
}

fn show_answer(answer: &word::Word) {
    println!("{}", answer.text.iter().collect::<String>());
}

fn is_valid_length(len: usize) -> bool {
    len == EXPECT_LENGTH
}
