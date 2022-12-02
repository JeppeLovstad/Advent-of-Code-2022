use input_reader;
use std::collections::HashMap;

fn main() {
    let input = prepare_input();
    let calculated_input: i32 = input
        .iter()
        .map(|x| calculate_score_for_round_part1(x))
        .sum();

    println!("{:?}", calculated_input);

    let calculated_input: i32 = input
        .iter()
        .map(|x| calculate_score_for_round_part2(x))
        .sum();
    println!("{:?}", calculated_input);
}

fn calculate_score_for_round_part2(round: &String) -> i32 {
    let winning_score: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let winning_move: HashMap<&str, &str> = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
    let tied_move: HashMap<&str, &str> = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    let lost_move: HashMap<&str, &str> = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);

    let (opponent_move, round_outcome) = round.split_once(" ").unwrap();

    let (my_move, victory) = {
        if round_outcome == "Z" {
            (winning_move[&opponent_move], 6)
        } else if round_outcome == "Y" {
            (tied_move[&opponent_move], 3)
        } else {
            (lost_move[&opponent_move], 0)
        }
    };

    victory + winning_score[&my_move]
}

fn calculate_score_for_round_part1(round: &String) -> i32 {
    let winning_score: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let winning_move: HashMap<&str, &str> = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
    let tied_move: HashMap<&str, &str> = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);

    let (opponent_move, my_move) = round.split_once(" ").unwrap();

    let victory: i32 = {
        if tied_move[&opponent_move].eq(my_move) {
            3
        } else if winning_move[&opponent_move].eq(my_move) {
            6
        } else {
            0
        }
    };

    victory + winning_score[&my_move]
}

fn prepare_input() -> Vec<String> {
    let name = env!("CARGO_PKG_NAME").to_string();
    let day_number = name.split("_").nth(1).unwrap().parse::<i32>().unwrap();

    input_reader::url_reader(2022, day_number, &name, false).unwrap();

    if let Ok(lines) = input_reader::read_lines(name) {
        lines
    } else {
        Vec::new()
    }
}
