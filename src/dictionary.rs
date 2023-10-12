use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;

pub(crate) fn select_wordle() -> String {
    let lines = get_lines("src/files/dictionary-wordles.txt");
    let random_index = rand::thread_rng().gen_range(0..lines.len());
    return lines.get(random_index).expect("Selection not in range").to_string();
}

pub(crate) fn valid_guess(guess: &String) -> bool {
    if get_lines("src/files/dictionary-wordles.txt").contains(guess) {
        return true;
    } else {
        return get_lines("src/files/dictionary-candidates.txt").contains(guess);
    }
}

fn get_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Could not find file!");
    let reader = BufReader::new(file);
    return reader.lines().filter_map(|result| result.ok()).collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use crate::dictionary::{select_wordle, valid_guess};

    #[test]
    fn test_select_wordle() {
        // arrange
        // act
        let wordle = select_wordle();
        // assert
        assert_eq!(wordle.len(), 5);
    }

    #[test]
    fn test_valid_guess() {
        // arrange
        let guess = "audio".to_string();

        // act
        let result = valid_guess(&guess);

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_valid_guess_candidates() {
        // arrange
        let guess = "abbes".to_string();

        // act
        let result = valid_guess(&guess);

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn test_valid_guess_invalid() {
        // arrange
        let guess = "xoxox".to_string();

        // act
        let result = valid_guess(&guess);

        // assert
        assert_eq!(result, false);
    }

}