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
    fn play_quiz(&self) {
        let mut question_num: i32 = 1;
        println!("Welcome to the quiz show!! There are {} total questions!", self.questions.len());
        
        for question in &self.questions {
            // Keeps player informed of their score every iteration
            println!("Your current score is: {}", self.score);
            println!("Question number {}:", question_num);

            // From questions vector
            println!("{}", question.text);
        }
    }
}

fn main() {
    let questions: Vec<Question> = vec!(
        Question {
            text: String::from("what is the capital of france???????"),
            options: ["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()],
            answer: String::from("a"),
        },
        Question {
            text: String::from("test"),
            options: ["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()],
            answer: String::from("a"),
        }
    );

    let new_quiz: Quiz = Quiz {
        questions: questions,
        score: 0,
    };

    new_quiz.play_quiz();

    println!("{}", new_quiz.score);
}