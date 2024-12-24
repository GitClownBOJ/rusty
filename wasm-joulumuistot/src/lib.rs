use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub struct Question {
    text: String,
    category: String,
}

#[wasm_bindgen]
impl Question {
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn category(&self) -> String {
        self.category.clone()
    }
}

#[wasm_bindgen]
pub struct Game {
    questions: Vec<Question>,
    used_questions: Vec<String>,
    rng: rand::rngs::ThreadRng,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let questions = vec![
            Question {
                text: String::from("Mikä on varhaisin joulumuistosi?"),
                category: String::from("Muistot"),
            },
            Question {
                text: String::from("Mikä joululaulu tuo sinulle parhaimmat muistot? Miksi?"),
                category: String::from("Musiikki"),
            },
            // Add all other questions here...
        ];

        Game {
            questions,
            used_questions: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn next_question(&mut self) -> Option<Question> {
        if self.used_questions.len() >= self.questions.len() {
            self.used_questions.clear();
        }

        let available_questions: Vec<&Question> = self.questions
            .iter()
            .filter(|q| !self.used_questions.contains(&q.text))
            .collect();

        if let Some(question) = available_questions.get(
            self.rng.gen_range(0..available_questions.len())
        ) {
            self.used_questions.push(question.text.clone());
            Some(Question {
                text: question.text.clone(),
                category: question.category.clone(),
            })
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.used_questions.clear();
    }
}