use input_reader;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = &prepare_input()[0];
    let start_marker = find_start_of_packet_marker(input.to_string());
    println!("{:?}", start_marker);

    let start_marker = find_start_of_message_marker(input.to_string());
    println!("{:?}", start_marker);
}

fn find_start_of_message_marker(input: String) -> i32 {
    let mut deque: VecDeque<char> = VecDeque::with_capacity(14);
    let mut hashmap: HashMap<char, i32> = HashMap::new();

    let input_chars = input
        .chars()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<Vec<(char, usize)>>();

    for (cs, i) in input_chars {
        deque.push_back(cs);
        (*(hashmap.entry(cs).or_insert(0))) += 1;
        if deque.len() == deque.capacity() {
            let popped_val = deque.pop_front().unwrap();
            hashmap.entry(popped_val).and_modify(|x| *x -= 1);
            if hashmap[&popped_val] == 0 {
                hashmap.remove_entry(&popped_val);
            }
        }

        if hashmap.values().sum::<i32>() == 14 && hashmap.len() == 14 {
            return i as i32 + 1;
        }
    }
    -1
}

fn find_start_of_packet_marker(input: String) -> i32 {
    let input_chars: Vec<char> = input.chars().collect();

    for (i, cs) in input_chars.windows(4).enumerate() {
        let (first, second, third, fourth) = (cs[0], cs[1], cs[2], cs[3]);

        if first == second
            || first == third
            || first == fourth
            || second == third
            || second == fourth
            || third == fourth
        {
            continue;
        } else {
            return i as i32 + 4;
        }
    }
    -1
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
