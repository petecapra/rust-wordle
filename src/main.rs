use std::io;
use std::io::BufRead;

mod wordle;
mod dictionary;

fn main() {
    loop {
        println!("Starting new Wordle...");
        let wordle = dictionary::select_wordle();
        let mut attempts = 0;
        let mut solved = false;
        while attempts < 6 && !solved {
            attempts += 1;
            let mut valid_guess = false;
            let mut guess: String = "".to_string();
            while !valid_guess {
                println!("Enter guess #{attempts}: ");
                guess = read_line();
                valid_guess = dictionary::valid_guess(&guess);
                if !valid_guess {
                    println!("That is not a valid word");
                }
            }
            let score = wordle::score_wordle(&*wordle, &*guess);
            println!("Score: {:?}", score);
            solved = wordle::is_solved(score);
            if solved {
                println!("Congratulations! You solved it in {attempts} attempts.");
            }
        }
        println!("The wordle was: {wordle}.\n\n");
    }
}

pub fn read_line() -> String {
    let mut name = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut name).unwrap();
    trim_newline(&mut name);
    name
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}