use utils::load;

fn parse_data(data: String) -> Vec<Vec<i64>> {
    let mut data = data;
    let inputs: Vec<Vec<i64>> = data
        .lines()
        .map(|line| {
            let split: Vec<i64> = line
                .split(" ")
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect();
            split
        })
        .collect::<Vec<Vec<i64>>>();
    inputs
}

fn all_zero(nums: &Vec<i64>) -> bool {
    for num in nums {
        if *num != 0 {
            return false;
        }
    }
    true
}

fn get_score_part_1(input: &Vec<i64>) -> i64 {
    let mut head_idx = 0;
    let mut state: Vec<Vec<i64>> = vec![input.to_vec()];

    // Generate Initial
    while !all_zero(&state[head_idx]) {
        let mut new_head: Vec<i64> = vec![];
        let mut prev_val = state[head_idx][0];
        for val in state[head_idx].iter().skip(1) {
            new_head.push(val - prev_val);
            prev_val = *val;
        }
        state.push(new_head);
        head_idx += 1;
    }

    // Calculate new values
    let mut prev_new_val: i64 = 0;
    for idx in (0..state.len()).rev() {
        let current_head = &state[idx];
        let length = current_head.len();
        prev_new_val = current_head[length - 1] + prev_new_val;
    }
    let score = prev_new_val;
    score
}

fn part1(inputs: &Vec<Vec<i64>>) -> i64 {
    let total: i64 = inputs
        .iter()
        .map(|nums| -> i64 { get_score_part_1(nums) })
        .sum::<i64>();
    total
}

fn main() {
    let data = load("input.txt");
    let inputs = parse_data(data);
    let part_1_result = part1(&inputs);
    println!("Result for Part 1 is: {}", part_1_result)
}
