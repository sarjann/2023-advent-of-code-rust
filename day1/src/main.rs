use utils;


fn parse_line(line: &str) -> u32 {
    let mut first_char: Option<char> = None;
    let mut last_char: Option<char> = None;
    let mut count: usize = 0;
    for i in line.chars() {
        match i {
            '0'..='9' => {
                last_char = Some(i);
                if first_char == None {
                    first_char = Some(i);
                }
            }
            'a'..='z' => {
                if let Some(int_char) = get_int_char(&line, &count) {
                    last_char = Some(int_char);
                    if first_char == None {
                        first_char = Some(int_char);
                    }
                }
            }
            _ => {}
        }
        count += 1;
    }

    let num_string = format!("{}{}", first_char.unwrap(), last_char.unwrap());
    let num: u32 = num_string.parse().unwrap();
    num
}

fn get_int_char(line: &str, count: &usize) -> Option<char> {
    let mut cv: Vec<char> = vec![];

    for ch in line[*count..].chars() {
        match ch {
            'a'..='z' => {
                cv.push(ch);
                if let Some(int_char) = get_match(&cv) {
                    return Some(int_char);
                }},
            _ => break
        }
    }
    return None;
}

fn get_match(char_vec: &Vec<char>) -> Option<char> {
    let char_string: String = char_vec.into_iter().collect();
    let char_str: &str = &char_string[..];
    return match char_str {
        "zero" => Some('0'),
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None
    }
}

fn main() {
    let data = utils::load("input.txt");
    let res = data.lines().map(|line| {
        parse_line(line)
    });
    let sum_res: u32 = res.sum();
    println!("{}", sum_res);
}
