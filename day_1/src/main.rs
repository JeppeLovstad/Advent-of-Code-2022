use std::cmp::Reverse;

use input_reader;

#[derive(Debug, Clone)]
struct Elf {
    total_calories: i32,
    calories: Vec<i32>
}

fn main() {
    let input = prepare_input();

    let elves: Vec<Elf> = Vec::new();
    for line in input {

        if line == "" {


            continue;
        }

        let cal:i32 = line.parse::<i32>().unwrap();
        elf.total_calories += cal;
        elf.calories.push(cal);
    }
    println!("{:?}", elf);
    elves.push(elf);

    let max = elves.iter().map(|x| x.total_calories).max();

    println!("{:?}", max);

    elves.sort_by_key(|x| Reverse(x.total_calories));
    let sum: i32 = elves[0..=2].iter().map(|x| x.total_calories).sum();
    println!("{:?}", sum);
}

fn group_input(input:Vec<String>) -> Vec<Vec<I32>>{

}

fn get_elf(calories:&Vec<i32>) -> Elf {
    
    Elf{
        total_calories: calories.iter().sum(),
        calories: calories.to_vec(),
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
