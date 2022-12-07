use input_reader;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = &prepare_input()[0];
    let start_marker = find_start_of_message_marker(input.to_string(), 4);
    println!("{:?}", start_marker);

    let start_marker = find_start_of_message_marker(input.to_string(), 14);
    println!("{:?}", start_marker);
}

fn find_start_of_message_marker(input: String, capacity: i32) -> i32 {
    let mut deque: VecDeque<char> = VecDeque::with_capacity(capacity as usize);
    let mut hashmap: HashMap<char, i32> = HashMap::new();

    let input_chars = input
        .chars()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<Vec<(char, usize)>>();

    for (cs, i) in input_chars {
        deque.push_back(cs);
        (*(hashmap.entry(cs).or_insert(0))) += 1;
        if deque.len() == capacity as usize + 1 {
            let popped_val = deque.pop_front().unwrap();
            hashmap.entry(popped_val).and_modify(|x| *x -= 1);
            if hashmap[&popped_val] == 0 {
                hashmap.remove_entry(&popped_val);
            }
        }

        if hashmap.values().sum::<i32>() == capacity && hashmap.len() == capacity as usize {
            return i as i32 + 1;
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
