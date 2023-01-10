use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Addition(char),
    Subtraction(char),
    Division(char),
    Multiplication(char),
}

pub struct MathProblem {
    pub first_number: u32,
    pub second_number: u32,
    pub operation: Operation,
    pub user_answer: Option<u32>,
    pub expected_answer: u32,
    pub user_correct: Option<bool>,
    pub ui_string: String,
}

impl MathProblem {
    pub fn new(difficulty: u32, allowed_operations: &Vec<Operation>) -> Self {
        let mut rng = thread_rng();
        let min = 1;
        let max = match difficulty {
            1 => 10,
            2 => 100,
            _ => 1000,
        };
        let first_number = rng.gen_range(min..=max);
        let second_number = rng.gen_range(min..=max);
        let rand_operation_index = rng.gen_range(0..allowed_operations.len());
        let operation = allowed_operations[rand_operation_index];
        let (expected_answer, ui_string) = match operation {
            Operation::Addition(ref operator) => (
                first_number + second_number,
                format!("{} {} {} = ?", &first_number, operator, &second_number),
            ),
            Operation::Subtraction(ref operator) => (
                first_number - second_number,
                format!("{} {} {} = ?", &first_number, operator, &second_number),
            ),
            Operation::Multiplication(ref operator) => (
                first_number * second_number,
                format!("{} {} {} = ?", &first_number, operator, &second_number),
            ),
            Operation::Division(ref operator) => (
                first_number / second_number,
                format!("{} {} {} = ?", &first_number, operator, &second_number),
            ),
        };

        MathProblem {
            first_number,
            second_number,
            operation,
            user_answer: None,
            expected_answer,
            user_correct: None,
            ui_string,
        }
    }
}
