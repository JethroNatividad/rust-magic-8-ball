// A program that asks for a question, and outputs one of these randomly “Yes,” “No,” “Maybe,” or “Ask again later.”
// Inputs: Question
// Process: Select from "Yes", "No", "Maybe", "Ask again later." randomly
// Output: "Yes", "No", "Maybe", or "Ask again later."

fn get_random_answer(answers: Vector<&str>) -> &str {
    // Choose using rand
    // return answer
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_answer() {
        let answers: Vector<&str> = vec!["Yes", "No", "Maybe", "Ask again later."];
        assert!(answers.contains(get_random_answer(answers)));
    }
}
fn main() {
    //
}
