use input_reader;
fn main() {
    let input = prepare_input();

    let parse_input_part1: i32 = input.iter().map(|input| parse_input_part1(input)).sum();
    println!("{:?}", parse_input_part1);

    let parse_input_part2: i32 = input.iter().map(|input| parse_input_part2(input)).sum();
    println!("{:?}", parse_input_part2)
}

fn parse_input_part2(input: &String) -> i32 {
    let (p1, p2) = input.split_once(",").unwrap();
    let res = if pair_is_overlapping(p1.to_string(), p2.to_string()) {
        1
    } else {
        0
    };
    res
}

fn pair_is_overlapping(p1: String, p2: String) -> bool {
    let (p1_start, p1_end) = p1.split_once("-").unwrap();
    let (p2_start, p2_end) = p2.split_once("-").unwrap();

    let p1_start = parse_string_to_int(p1_start);
    let p1_end = parse_string_to_int(p1_end);
    let p2_start = parse_string_to_int(p2_start);
    let p2_end = parse_string_to_int(p2_end);

    if p1_start <= p2_start && p1_end >= p2_end {
        return true;
    }

    if p2_start <= p1_start && p2_end >= p1_end {
        return true;
    }

    if (p1_start >= p2_start && p1_start <= p2_end) || (p1_end <= p2_start && p1_end >= p2_end) {
        return true;
    }

    if (p2_start >= p1_start && p2_start <= p1_end) || (p2_end <= p1_start && p2_end >= p1_end) {
        return true;
    }

    false
}

fn parse_input_part1(input: &String) -> i32 {
    let (p1, p2) = input.split_once(",").unwrap();
    if pair_is_completely_overlapping(p1.to_string(), p2.to_string()) {
        1
    } else {
        0
    }
}

fn pair_is_completely_overlapping(p1: String, p2: String) -> bool {
    let (p1_start, p1_end) = p1.split_once("-").unwrap();
    let (p2_start, p2_end) = p2.split_once("-").unwrap();

    let p1_start = parse_string_to_int(p1_start);
    let p1_end = parse_string_to_int(p1_end);
    let p2_start = parse_string_to_int(p2_start);
    let p2_end = parse_string_to_int(p2_end);

    if p1_start <= p2_start && p1_end >= p2_end {
        return true;
    }

    if p2_start <= p1_start && p2_end >= p1_end {
        return true;
    }

    false
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
