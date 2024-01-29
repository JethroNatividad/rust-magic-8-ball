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

fn valid_question(question: &str) -> bool {
    // remove whitespace
    // check last char if equal to ?
    question.trim_end().ends_with("?")
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

    #[test]
    fn test_valid_question() {
        assert_eq!(valid_question("Will it rain today?"), true);
        assert_eq!(valid_question("Am I lucky today?"), true);
        assert_eq!(valid_question("Hello"), false);
        assert_eq!(valid_question("asfasff"), false);
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

fn ask_question() {
    loop {
        let question: String = get_input("What's your question? ");
        match valid_question(question) {
            true => break,
            false => println!("That's not a question, Please try again."),
        }
    }
}

fn main() {
    // ask a question.
    ask_question();
    // generate random answer
    let answer: &str = get_random_answer(vec!["Yes", "No", "Maybe", "Ask again later."]);
    // print random answer
    println!("{}", answer);
}
