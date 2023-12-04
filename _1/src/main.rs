use std::fs;
use std::env;
use phf::phf_map;


fn collect_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    return args;
}

static DIGITS: phf::Map<&str, char> = phf_map! { 
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
};

fn find_next_digit<I>(chars: I, string_digits: &phf::Map<&str, char>, reversed: bool) -> char 
where
    I: IntoIterator<Item = char>
{
    let mut substr = String::new();
    for c in chars {
        if c.is_numeric() {
            return c;
        }

        let substr_idx = if reversed == false {substr.len()} else {0};
        substr.insert(substr_idx, c);
        for (digit_str, digit) in string_digits {
            if substr.contains(digit_str) {
                return *digit;
            }
        }
    }

    panic!("Invalid input data!");
}

fn parse_line(line: &str, string_digits: &phf::Map<&str, char>) -> i32 {
    let mut number = String::new();
    number.push(find_next_digit(line.chars().into_iter(), string_digits, false));
    number.push(find_next_digit(line.chars().rev().into_iter(), string_digits, true));
    return number.parse::<i32>()
        .expect("Could not parse to string");
}

fn main() {
    let args = collect_args();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let mut part_1_result = 0;
    let mut part_2_result = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        part_1_result += parse_line(line, &phf_map!{});
        part_2_result += parse_line(line, &DIGITS); 
    }

    println!("part_1: {}", part_1_result);
    println!("part_2: {}", part_2_result);
    assert_eq!(part_1_result, 54644);
    assert_eq!(part_2_result, 53348);
}

