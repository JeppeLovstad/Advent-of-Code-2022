use std::collections::HashSet;

use input_reader;
fn main() {
    let input = prepare_input();
    let scored_input = input.iter().map(|x| convert_rucksack_to_score(x));

    println!("{:?}", scored_input.sum::<u32>());

    let part_2_sum = group_input_and_sum(input);
    println!("{:?}", part_2_sum);
}

fn group_input_and_sum(input: Vec<String>) -> u32 {
    let groups = input.chunks_exact(3);
    let calculated_groups = groups.map(|x| calculate_group(x));

    calculated_groups.sum()
}

fn calculate_group(group: &[String]) -> u32 {
    let (r1, r2, r3) = get_each_rucksack(group);
    let r1_hash: HashSet<char> = HashSet::from_iter(r1.chars());
    let r2_hash: HashSet<char> = HashSet::from_iter(r2.chars());
    let r3_hash: HashSet<char> = HashSet::from_iter(r3.chars());

    let component = r1_hash.iter().reduce(|accum, item| {
        if r1_hash.contains(&item) && r2_hash.contains(&item) && r3_hash.contains(&item) {
            item
        } else {
            accum
        }
    });
    get_itemtype_priority_score(*component.unwrap())
}

fn get_each_rucksack(group: &[String]) -> (String, String, String) {
    (
        group[0].to_string(),
        group[1].to_string(),
        group[2].to_string(),
    )
}

fn convert_rucksack_to_score(rucksack: &String) -> u32 {
    let shared_component = get_shared_component(rucksack.split_at(rucksack.len() / 2));
    get_itemtype_priority_score(shared_component)
}

fn get_shared_component((compartment_one, compartment_two): (&str, &str)) -> char {
    let all_compartment_one: HashSet<char> = HashSet::from_iter(compartment_one.chars());

    let component = compartment_two.chars().reduce(|accum, item| {
        if all_compartment_one.contains(&item) {
            item
        } else {
            accum
        }
    });

    component.unwrap()
}

fn get_itemtype_priority_score(itemtype: char) -> u32 {
    let res = if itemtype.is_ascii_lowercase() {
        itemtype as u32 - 96
    } else {
        itemtype as u32 - 64 + 26
    };
    res
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
