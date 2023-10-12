use crate::wordle::Score::{CorrectSpot, NotPresent, WrongSpot};

#[derive(Debug, PartialEq)]
pub(crate) enum Score {
    CorrectSpot,
    WrongSpot,
    NotPresent
}

pub(crate) fn score_wordle(wordle: &str, guess: &str) -> Vec<(char, Score)> {
    let wordle: Vec<char> = wordle.chars().collect();
    let guess: Vec<char> = guess.chars().collect();
    let mut scores:Vec<(char, Score)> = Vec::new();

    // first pass to get correct spots
    for n in 0..5 {
        let wordle_char = wordle.get(n).expect("Wordle length is incorrect!");
        let guess_char = guess.get(n).expect("Guess length is incorrect!");
        if wordle_char == guess_char {
            scores.push((*guess_char, CorrectSpot));
        } else {
            scores.push((*guess_char, NotPresent));
        }
    }

    // second pass to get wrong spots
    for n in 0..5 {
        let guess_char = guess.get(n).expect("Guess length is incorrect!");

        // skip letters already scored correctly
        let scored_letter = scores.get(n).expect("Scores length is incorrect");
        if scored_letter == &(*guess_char, CorrectSpot) {
            continue;
        }

        // skip letters that don't occur at all
        if !wordle.contains(guess_char) {
            continue;
        }

        // this letter occurs at least once, but just not in this spot
        let guess_occurrences = count_char_occurrences(&wordle, guess_char);
        let times_scored = scores.iter().filter(|&(c,score) | c == guess_char && (score == &CorrectSpot || score == &WrongSpot)).count();
        if guess_occurrences > times_scored {
            scores[n] = (*guess_char, WrongSpot);
        }

    }
    return scores;
}

pub(crate) fn is_solved(score: Vec<(char, Score)>) -> bool {
    for scored_letter in score {
        match scored_letter {
            (_, NotPresent) => return false,
            (_, WrongSpot) => return false,
            _ => continue
        }
    }
    return true;
}

fn count_char_occurrences(wordle: &Vec<char>, target_char: &char) -> usize {
    wordle.iter().filter(|&c| c == target_char).count()
}

#[cfg(test)]
mod tests {
    use Score::CorrectSpot;
    use crate::wordle::{Score, score_wordle};
    use crate::wordle::Score::{NotPresent, WrongSpot};

    #[test]
    fn test_score_wordle_all_correct() {
        // arrange
        let wordle = "image";
        let guess = "image";

        // act
        let wordle = score_wordle(wordle, guess);

        // assert
        assert_eq!(wordle.len(), 5);
        assert_eq!(wordle.get(0).unwrap(), &('i', CorrectSpot));
        assert_eq!(wordle.get(1).unwrap(), &('m', CorrectSpot));
        assert_eq!(wordle.get(2).unwrap(), &('a', CorrectSpot));
        assert_eq!(wordle.get(3).unwrap(), &('g', CorrectSpot));
        assert_eq!(wordle.get(4).unwrap(), &('e', CorrectSpot));
    }

    #[test]
    fn test_score_wordle_non_correct() {
        // arrange
        let wordle = "image";
        let guess = "souls";

        // act
        let wordle = score_wordle(wordle, guess);

        // assert
        assert_eq!(wordle.len(), 5);
        assert_eq!(wordle.get(0).unwrap(), &('s', NotPresent));
        assert_eq!(wordle.get(1).unwrap(), &('o', NotPresent));
        assert_eq!(wordle.get(2).unwrap(), &('u', NotPresent));
        assert_eq!(wordle.get(3).unwrap(), &('l', NotPresent));
        assert_eq!(wordle.get(4).unwrap(), &('s', NotPresent));
    }

    #[test]
    fn test_score_wordle_one_correct_spot() {
        // arrange
        let wordle = "slate";
        let guess = "sound";

        // act
        let wordle = score_wordle(wordle, guess);

        // assert
        assert_eq!(wordle.len(), 5);
        assert_eq!(wordle.get(0).unwrap(), &('s', CorrectSpot));
        assert_eq!(wordle.get(1).unwrap(), &('o', NotPresent));
        assert_eq!(wordle.get(2).unwrap(), &('u', NotPresent));
        assert_eq!(wordle.get(3).unwrap(), &('n', NotPresent));
        assert_eq!(wordle.get(4).unwrap(), &('d', NotPresent));
    }

    #[test]
    fn test_score_wordle_some_correct_some_wrong_spot() {
        // arrange
        let wordle = "slate";
        let guess = "stale";

        // act
        let wordle = score_wordle(wordle, guess);

        // assert
        assert_eq!(wordle.len(), 5);
        assert_eq!(wordle.get(0).unwrap(), &('s', CorrectSpot));
        assert_eq!(wordle.get(1).unwrap(), &('t', WrongSpot));
        assert_eq!(wordle.get(2).unwrap(), &('a', CorrectSpot));
        assert_eq!(wordle.get(3).unwrap(), &('l', WrongSpot));
        assert_eq!(wordle.get(4).unwrap(), &('e', CorrectSpot));
    }

    #[test]
    fn test_score_wordle_guessed_letter_twice_in_wordle() {
        // arrange
        let wordle = "truth";
        let guess = "tails";

        // act
        let wordle = score_wordle(wordle, guess);

        // assert
        assert_eq!(wordle.len(), 5);
        assert_eq!(wordle.get(0).unwrap(), &('t', CorrectSpot));
        assert_eq!(wordle.get(1).unwrap(), &('a', NotPresent));
        assert_eq!(wordle.get(2).unwrap(), &('i', NotPresent));
        assert_eq!(wordle.get(3).unwrap(), &('l', NotPresent));
        assert_eq!(wordle.get(4).unwrap(), &('s', NotPresent));
    }

    #[test]
    fn test_score_wordle_guessed_letter_twice_in_wordle_multiple() {
        // arrange
        let wordle = "truth";
        let guess = "title";

        // act
        let wordle = score_wordle(wordle, guess);

        // assert
        assert_eq!(wordle.len(), 5);
        assert_eq!(wordle.get(0).unwrap(), &('t', CorrectSpot));
        assert_eq!(wordle.get(1).unwrap(), &('i', NotPresent));
        assert_eq!(wordle.get(2).unwrap(), &('t', WrongSpot));
        assert_eq!(wordle.get(3).unwrap(), &('l', NotPresent));
        assert_eq!(wordle.get(4).unwrap(), &('e', NotPresent));
    }

    #[test]
    fn test_score_wordle_twice_guessed_single_letter() {
        // arrange
        let wordle = "truth";
        let guess = "tarry";

        // act
        let wordle = score_wordle(wordle, guess);

        // assert
        assert_eq!(wordle.len(), 5);
        assert_eq!(wordle.get(0).unwrap(), &('t', CorrectSpot));
        assert_eq!(wordle.get(1).unwrap(), &('a', NotPresent));
        assert_eq!(wordle.get(2).unwrap(), &('r', WrongSpot));
        assert_eq!(wordle.get(3).unwrap(), &('r', NotPresent));
        assert_eq!(wordle.get(4).unwrap(), &('y', NotPresent));
    }

    #[test]
    fn test_score_wordle_edge_case_1() {
        // arrange
        let wordle = "fence";
        let guess = "eeece";

        // act
        let wordle = score_wordle(wordle, guess);

        // assert
        assert_eq!(wordle.len(), 5);
        assert_eq!(wordle.get(0).unwrap(), &('e', NotPresent));
        assert_eq!(wordle.get(1).unwrap(), &('e', CorrectSpot));
        assert_eq!(wordle.get(2).unwrap(), &('e', NotPresent));
        assert_eq!(wordle.get(3).unwrap(), &('c', CorrectSpot));
        assert_eq!(wordle.get(4).unwrap(), &('e', CorrectSpot));
    }

    #[test]
    fn test_score_wordle_edge_case_2() {
        // arrange
        let wordle = "fence";
        let guess = "efece";

        // act
        let wordle = score_wordle(wordle, guess);

        // assert
        assert_eq!(wordle.len(), 5);
        assert_eq!(wordle.get(0).unwrap(), &('e', WrongSpot));
        assert_eq!(wordle.get(1).unwrap(), &('f', WrongSpot));
        assert_eq!(wordle.get(2).unwrap(), &('e', NotPresent));
        assert_eq!(wordle.get(3).unwrap(), &('c', CorrectSpot));
        assert_eq!(wordle.get(4).unwrap(), &('e', CorrectSpot));
    }

}
