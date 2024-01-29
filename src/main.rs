use rand::seq::SliceRandom;
use std::io;
use std::io::Write;

// A program that asks for a question, and outputs one of these randomly “Yes,” “No,” “Maybe,” or “Ask again later.”
// Inputs: Question
// Process: Select from "Yes", "No", "Maybe", "Ask again later." randomly
// Output: "Yes", "No", "Maybe", or "Ask again later."

fn get_random_answer(answers: Vec<&str>) -> &str {
    // Choose using rand
    let result: Option<&&str> = answers.choose(&mut rand::thread_rng());
    // return answer
    match result {
        Some(answer) => return *answer,
        None => return "No answer!",
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn valid_question(question: &str) -> bool {
    // remove whitespace
    // check last char if equal to ?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_answer() {
        let answers: Vec<&str> = vec!["Yes", "No", "Maybe", "Ask again later."];
        let answer: &str = get_random_answer(answers.clone());
        assert!(answers.contains(&answer));
    }
}
fn main() {
    // ask a question.
    // generate random answer
    // print random answer
}
