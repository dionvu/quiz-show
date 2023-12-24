use std::io;
use colored::Colorize;

struct Question {
    text: String,
    options: [String; 4],
    answer: String,
}

struct Quiz {
    questions: Vec<Question>,
    score: i32,
}

impl Quiz {
    fn ask_question(question: &Question) -> bool {
        println!("{}", question.text);

        println!("The possible options are: {}, {}, {}, or {}.",
            question.options[0],
            question.options[1],
            question.options[2],
            question.options[3],
            
        );

        let mut input: String = String::new();

        // Handles error then checks if answer is correct
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input: &str = input.trim(); // trims the whitespace from input

                if input == question.answer {
                    println!("{}", "That is the correct answer!".green());
                    return true;
                }
                else {
                    println!("{}", "I'm sorry that is the wrong answer".red());
                    return false;
                }
            }
            Err(_) => {
                eprintln!("Error reading input.");
                return false;
            }
        }
    }

    fn play_quiz(&mut self) {
        let mut question_num: i32 = 1;
        println!("Welcome to the quiz show!! There are {} total questions!", self.questions.len());
        
        for question in &self.questions {
            // Keeps player informed of their score every iteration
            println!("Your current score is: {}", self.score);
            println!("Question number {}:", question_num);
            
            question_num += 1;
            
            // Checks if question is correct
            let correct: bool = Quiz::ask_question(question);

            if correct { self.score += 1 };
        }

        println!("Congratulations! Your final score is {}, thanks for playing!", self.score);
    }
}

fn main() {
    let questions: Vec<Question> = vec!(
        Question {
            text: String::from("Which is the longest river in the world?"),
            options: ["Nile River".to_string(), "Amazon River".to_string(), "Yangtze River".to_string(), "Mississippi River".to_string()],
            answer: String::from("Nile River"),
        },
        Question {
            text: String::from("test"),
            options: ["Fe".to_string(), "Ag".to_string(), "Au".to_string(), "Hg".to_string()],
            answer: String::from("Au"),
        },
        Question {
            text: String::from("What is the setting of 'To Kill a Mockingbird'?"),
            options: ["Maycomb, Alabama".to_string(), "London, England".to_string(), 
                "New York City, New York".to_string(), "Los Angeles, California".to_string()],
            answer: String::from("Maycomb, Alabama"),
        },
        Question {
            text: String::from("Which art movement is characterized by distorted shapes and vivid colors, often conveying emotion?"),
            options: ["Impressionism".to_string(), "Cubism".to_string(), "Abstract Expressionism".to_string(), "Surrealism".to_string()],
            answer: String::from("Expressionism"),
        },
        Question {
            text: String::from("In the anime 'Attack on Titan,' what is the name of the elite group of soldiers using omni-directional mobility gear to fight Titans?"),
            options: ["Survey Corps".to_string(), "Garrison Regiment".to_string(), "Military Police Brigade".to_string(), "104th Training Corps".to_string()],
            answer: String::from("Survey Corps"),
        },
        Question {
            text: String::from("Which part of the human brain is responsible for regulating vital functions such as breathing, heart rate, and blood pressure?"),
            options: ["Cerebellum".to_string(), "Hypothalamus".to_string(), "Medulla Oblongata".to_string(), "Amygdala".to_string()],
            answer: String::from("Medulla Oblongata"),
        }
    );

    let mut new_quiz: Quiz = Quiz {
        questions: questions,
        score: 0,
    };

    new_quiz.play_quiz();
}