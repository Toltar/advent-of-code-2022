use std::fmt;
use std::fs::read_to_string;
struct InvalidInputError;
impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input")
    }
}

#[derive(Eq, PartialEq)]
enum Plays {
    Rock,
    Paper,
    Scissors,
}

impl Plays {
    fn value(&self) -> i32 {
        match *self {
            Plays::Rock => 1,
            Plays::Paper => 2,
            Plays::Scissors => 3,
        }
    }

    fn to_str(&self) -> &str {
        match *self {
            Plays::Paper => "Paper",
            Plays::Rock => "Rock",
            Plays::Scissors => "Scissors",
        }
    }
}

impl Clone for Plays {
    fn clone(&self) -> Plays {
        match self {
            Plays::Rock => Plays::Rock,
            Plays::Paper => Plays::Paper,
            Plays::Scissors => Plays::Scissors,
        }
    }
}

enum OpponentsPlay {
    A,
    B,
    C,
}

impl OpponentsPlay {
    fn value(&self) -> Plays {
        match *self {
            OpponentsPlay::A => Plays::Rock,
            OpponentsPlay::B => Plays::Paper,
            OpponentsPlay::C => Plays::Scissors,
        }
    }

    fn from_str(input: &str) -> Option<OpponentsPlay> {
        match input {
            "A" => Some(OpponentsPlay::A),
            "B" => Some(OpponentsPlay::B),
            "C" => Some(OpponentsPlay::C),
            _ => None,
        }
    }
}

enum PlayersPlay {
    X,
    Y,
    Z,
}

impl PlayersPlay {
    fn value(&self) -> Plays {
        match *self {
            PlayersPlay::X => Plays::Rock,
            PlayersPlay::Y => Plays::Paper,
            PlayersPlay::Z => Plays::Scissors,
        }
    }

    fn from_str(input: &str) -> Option<PlayersPlay> {
        match input {
            "X" => Some(PlayersPlay::X),
            "Y" => Some(PlayersPlay::Y),
            "Z" => Some(PlayersPlay::Z),
            _ => None,
        }
    }
}

enum GameResult {
    Loss,
    Win,
    Draw,
}

impl GameResult {
    fn value(&self) -> i32 {
        match *self {
            GameResult::Loss => 0,
            GameResult::Win => 6,
            GameResult::Draw => 3,
        }
    }
}

fn split_line_and_convert(line: &str) -> (OpponentsPlay, PlayersPlay) {
    let round: Vec<_> = line.split(" ").collect();
    let opponents_play = OpponentsPlay::from_str(round[0]).unwrap();
    let players_play = PlayersPlay::from_str(round[1]).unwrap();
    return (opponents_play, players_play);
}

fn determine_game_result_and_players_play(
    opponents_play: OpponentsPlay,
    players_play: PlayersPlay,
) -> (GameResult, Plays) {
    let opponents_move = opponents_play.value();
    let players_move = players_play.value();
    if opponents_move.value() == players_move.value() {
        return (GameResult::Draw, players_move);
    }

    // Win or Loss
    if (opponents_move == Plays::Rock && players_move == Plays::Paper)
        || (opponents_move == Plays::Paper && players_move == Plays::Scissors)
        || (opponents_move == Plays::Scissors && players_move == Plays::Rock)
    {
        return (GameResult::Win, players_move);
    } else {
        return (GameResult::Loss, players_move);
    }
}

fn determine_round_score(round_play: (OpponentsPlay, PlayersPlay)) -> i32 {
    let game_result = determine_game_result_and_players_play(round_play.0, round_play.1);
    return game_result.0.value() + game_result.1.value();
}

fn part_one() {
    let file_contents = read_to_string("./input.txt").unwrap();
    let lines = file_contents.lines();
    let mut total_score = 0;
    for line in lines {
        println!("Parsing round: {}", line);
        let round_play = split_line_and_convert(line);
        println!("Determining round score...");
        let round_score = determine_round_score(round_play);

        total_score += round_score;

        println!(
            "Round score is {}! And total is {}",
            round_score, total_score
        );
    }

    println!("Total score is {}", total_score);
}

enum RoundOutcome {
    Win,
    Loss,
    Draw,
}

impl RoundOutcome {
    fn value(&self) -> i32 {
        match *self {
            RoundOutcome::Loss => 0,
            RoundOutcome::Win => 6,
            RoundOutcome::Draw => 3,
        }
    }

    fn from_str(input: &str) -> Option<RoundOutcome> {
        match input {
            "X" => Some(RoundOutcome::Loss),
            "Y" => Some(RoundOutcome::Draw),
            "Z" => Some(RoundOutcome::Win),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match *self {
            RoundOutcome::Draw => "Draw",
            RoundOutcome::Loss => "Loss",
            RoundOutcome::Win => "Win",
        }
    }
}

impl Clone for RoundOutcome {
    fn clone(&self) -> RoundOutcome {
        match self {
            RoundOutcome::Draw => RoundOutcome::Draw,
            RoundOutcome::Loss => RoundOutcome::Loss,
            RoundOutcome::Win => RoundOutcome::Win,
        }
    }
}

fn determine_players_play(round_outcome: RoundOutcome, opponents_play: Plays) -> Plays {
    let play = match round_outcome {
        RoundOutcome::Draw => opponents_play.clone(),
        RoundOutcome::Loss => match opponents_play {
            Plays::Paper => Plays::Rock,
            Plays::Rock => Plays::Scissors,
            Plays::Scissors => Plays::Paper,
        },
        RoundOutcome::Win => match opponents_play {
            Plays::Paper => Plays::Scissors,
            Plays::Rock => Plays::Paper,
            Plays::Scissors => Plays::Rock,
        },
    };
    return play;
}

fn part_two() {
    let file_contents = read_to_string("./input.txt").unwrap();
    let lines = file_contents.lines();
    let mut total_score = 0;
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let round_outcome = RoundOutcome::from_str(split[1]).unwrap();
        let round_outcome_as_str = round_outcome.to_str();

        let opponents_play = OpponentsPlay::from_str(split[0]).unwrap().value();
        let opponents_play_as_str = opponents_play.to_str();

        let players_play = determine_players_play(round_outcome.clone(), opponents_play.clone());

        let round_score = players_play.value() + round_outcome.value();
        total_score += round_score;
        println!(
            "Round Score: {}, Total Score: {}, Opponents Play {}, Players Play {}, Round Outcome: {}",
            round_score,
            total_score,
            opponents_play_as_str,
            players_play.to_str(),
            round_outcome_as_str
        );
    }

    println!("Total score for part 2 is {}", total_score);
}

fn main() {
    part_one();
    part_two();
}
