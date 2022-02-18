use colored::*;

#[derive(Debug)]
pub struct Word {
    pub text: Vec<char>,
}

impl Word {
    pub fn new(chars: Vec<char>) -> Self {
        Self { text: chars }
    }
    pub fn color_diff(&self, answer: &Word) {
        for (i, char) in self.text.iter().enumerate() {
            match answer.text.get(i) {
                Some(answer_char) => {
                    if char == answer_char {
                        print!("{}", char.to_string().green());
                    } else if answer.text.contains(char) {
                        // todo: containerとしているため以下の場合適切な色付けがされない
                        // answer: world
                        // input: color
                        // この場合答えは o が１つなので、一致している o のみ色付けされてほしい
                        // ただ、color にあるどちらの o も緑と黄色で色付けされてしまうので、答えに o が2つ入っていると勘違いを生む
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

    pub fn eq(&self, other: &Word) -> bool {
        self.text == other.text
    }
}
