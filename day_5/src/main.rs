use input_reader;
use std::{collections::HashMap, str::from_utf8};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Crate {
    id: String,
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Stack {
    id: usize,
    crates: Vec<Crate>,
}

impl Stack {
    fn pop(&mut self) -> Option<Crate> {
        self.crates.pop()
    }

    fn push(&mut self, crat: Crate) {
        self.crates.push(crat)
    }

    fn pop_range(&mut self, range: usize) -> Vec<Crate> {
        let mut vec = vec![];
        for _ in 0..range {
            vec.push(self.crates.pop().unwrap());
        }
        vec
    }

    fn push_range(&mut self, crat: Vec<Crate>) {
        crat.into_iter().rev().for_each(|f| self.crates.push(f))
    }
}

#[derive(Debug)]
struct Pile {
    stack_to_id: HashMap<usize, Stack>,
}

impl Pile {
    fn perform_operation(&mut self, amount: usize, from: usize, to: usize) {
        let (_, mut from_stack) = self.stack_to_id.remove_entry(&from).unwrap();
        let (_, mut to_stack) = self.stack_to_id.remove_entry(&to).unwrap();

        for i in 0..amount {
            let popped_element = from_stack.pop();
            to_stack.push(popped_element.unwrap());
        }

        self.stack_to_id.insert(from, from_stack);
        self.stack_to_id.insert(to, to_stack);
    }

    fn perform_operation2(&mut self, amount: usize, from: usize, to: usize) {
        let (_, mut from_stack) = self.stack_to_id.remove_entry(&from).unwrap();
        let (_, mut to_stack) = self.stack_to_id.remove_entry(&to).unwrap();

        let popped_element = from_stack.pop_range(amount);
        to_stack.push_range(popped_element);

        self.stack_to_id.insert(from, from_stack);
        self.stack_to_id.insert(to, to_stack);
    }
}

fn main() {
    let input = prepare_input();
    let grouped_input = group_input(input);
    let mut pile = generate_pile(grouped_input.first().unwrap().to_vec()); //.max();

    for operation in grouped_input.last().unwrap().to_vec() {
        let (amount, from, to) = parse_operation(operation);
        pile.perform_operation(amount, from, to)
    }

    for stack in &pile.stack_to_id {
        println!("{:?}", stack);
    }

    let mut pile2 = generate_pile(grouped_input.first().unwrap().to_vec()); //.max();
    for operation in grouped_input.last().unwrap().to_vec() {
        let (amount, from, to) = parse_operation(operation);
        pile2.perform_operation2(amount, from, to)
    }

    for i in (0..pile2.stack_to_id.len()) {
        println!("{:?}", pile2.stack_to_id.get(&i));
    }
}

fn parse_operation(operation_input: String) -> (usize, usize, usize) {
    let ops: Vec<i32> = operation_input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    (
        (ops[0]) as usize,
        (ops[1] - 1) as usize,
        (ops[2] - 1) as usize,
    )
}

fn group_input(input: Vec<String>) -> Vec<Vec<String>> {
    let grouped_input = input.split(|x| x == "");
    let parsed_grouped_input = grouped_input.map(|x| Vec::from(x));
    parsed_grouped_input.collect()
}

fn generate_pile(grouped_input: Vec<String>) -> Pile {
    let stack_height = grouped_input.len();
    let stacks = get_stacks(&grouped_input[0..stack_height - 1]);
    let pile = create_pile(stacks);
    pile
}
// -> Vec<(usize, String)>
fn get_stacks(stacks_lines: &[String]) -> Vec<Vec<String>> {
    let t: Vec<Vec<String>> = stacks_lines
        .iter()
        .map(|x| split_string_into_crates(x.replace('[', " ").replace(']', " ")))
        .collect();
    let t = transpose2::<String>(t);
    let t = remove_empty_stacks(t);
    t.iter()
        .map(|x| {
            x.iter()
                .map(|z| z.to_string())
                .filter(|y| y != " ")
                .rev()
                .collect()
        })
        .collect()
}

fn remove_empty_stacks(stacks: Vec<Vec<String>>) -> Vec<Vec<String>> {
    stacks
        .into_iter()
        .filter(|x| vec_only_contains_empty_values(x))
        .collect()
}

fn vec_only_contains_empty_values(vec: &Vec<String>) -> bool {
    for val in vec {
        if val != " " {
            return true;
        }
    }
    return false;
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

// fn populate_pile(pile: Pile, stacks_lines: &[String]) -> Pile {
//     for line in stacks_lines {
//         let clean_line = line.replace('[', "").replace(']', "");
//         let crates = split_string_into_crates(clean_line);
//         for (i, id) in crates.into_iter().enumerate() {
//             println!("{:?} {:?}", clean_crate_string(&id), i);
//             let new_crate = Crate { id: id };
//             let stack = pile.stack_to_id.get(&i).unwrap();
//             stack.crates.push(new_crate);
//         }
//         println!("");
//     }
//     pile
// }

fn split_string_into_crates(line: String) -> Vec<String> {
    line.as_bytes()
        .chunks(2)
        .map(|x| clean_crate_string(from_utf8(x).unwrap().to_string()))
        .collect()
}

fn clean_crate_string(crate_string: String) -> String {
    let component = crate_string
        .chars()
        .reduce(|accum, item| if item != '\0' { item } else { accum });
    component.unwrap().to_string()
}

fn create_pile(stacks: Vec<Vec<String>>) -> Pile {
    let stack_tuples = stacks.iter().enumerate().map(|(i, x)| Stack {
        id: i,
        crates: x.iter().map(|y| Crate { id: y.to_string() }).collect(),
    });

    let pile = Pile {
        stack_to_id: HashMap::from_iter(stack_tuples.map(|x| (x.id, x))),
    };
    pile
}

fn parse_string_to_int(input_string: &str) -> i32 {
    input_string.parse::<i32>().unwrap()
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
