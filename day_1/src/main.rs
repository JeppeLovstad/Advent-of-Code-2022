use input_reader;
use std::cmp::Reverse;

#[derive(Debug, Clone)]
struct Elf {
    total_calories: i32,
}

fn main() {
    let input = prepare_input();
    let grouped_input = group_input(input);
    let elves: Vec<Elf> = get_elves(grouped_input);

    let max = elves.iter().map(|x| x.total_calories).max();

    println!("{:?}", max);

    let mut sorted_elves = elves.clone();
    sorted_elves.sort_by_key(|x| Reverse(x.total_calories));
    let sum: i32 = sorted_elves[0..=2].iter().map(|x| x.total_calories).sum();
    println!("{:?}", sum);
}

fn group_input(input: Vec<String>) -> Vec<Vec<i32>> {
    let grouped_input = input.split(|x| x == "");
    let parsed_grouped_input =
        grouped_input.map(|x| x.into_iter().map(|y| parse_string_to_int(y)).collect());
    parsed_grouped_input.collect()
}

fn parse_string_to_int(input_string: &String) -> i32 {
    input_string.parse::<i32>().unwrap()
}

fn get_elves(grouped_elves: Vec<Vec<i32>>) -> Vec<Elf> {
    grouped_elves.into_iter().map(|x| get_elf(x)).collect()
}

fn get_elf(calories: Vec<i32>) -> Elf {
    Elf {
        total_calories: calories.iter().sum(),
    }
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
