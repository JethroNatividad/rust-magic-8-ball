use rand::seq::SliceRandom;

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
