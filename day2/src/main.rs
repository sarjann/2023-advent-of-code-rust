use std::collections::HashMap;
use utils::load;


fn part1() {
    let mut hash_map: HashMap<&str, i32> = HashMap::new();
    hash_map.insert("red", 12);
    hash_map.insert("green", 13);
    hash_map.insert("blue", 14);
    let hash_map = hash_map;

    let data = load("input.txt");

    let total_score: i32 = data.lines().map(|line| game_score(&line, &hash_map)).sum();
    println!("Total Score 1: {}", total_score);
}

fn part2() {
    let data = load("input.txt");

    let total_score: i32 = data.lines().map(|line| power_min_cubes(&line)).sum();
    println!("Total Score 2: {}", total_score);
}

fn set_needed_cubes<'a>(step: &'a str, hash_map: &mut HashMap<&'a str, i32>){
    let actions: Vec<&str> = step.split(",").collect();
    for action in actions {
        let split_action: Vec<&str> = action.split(" ").collect();
        let count: i32 = split_action[1].parse::<i32>().unwrap();
        let colour: &str = split_action[2];
        let bag_count: i32 = 
            match hash_map.get(colour) {
                Some(x) => *x,
                _ => 0,
            };
        if count > bag_count {
            hash_map.insert(colour, count);
        }
    }
}

fn power_min_cubes(line: &str) -> i32 {
    let mut hash_map: Box::<HashMap<&str, i32>> = Box::new(HashMap::new());
    let split_line: Vec<&str> = line.split(":").collect();
    
    let steps: &str = split_line[1];
    let split_steps: Vec<&str> = steps.split(";").collect();
    for step in split_steps {
        set_needed_cubes(step, &mut hash_map);
    }
 
    // Not set to 1 incase the game takes nothing out.
    let mut score = 0;
    for (_, count) in hash_map.iter() {
        if score == 0 {
            score = *count
        } else {
            score *= *count;
        }
    }
    score
}

fn is_valid_step(step: &str, hash_map: &HashMap<&str, i32>) -> bool {
    let actions: Vec<&str> = step.split(",").collect();
    for action in actions {
        let split_action: Vec<&str> = action.split(" ").collect();
        let count: i32 = split_action[1].parse::<i32>().unwrap();
        let colour: &str = split_action[2];
        let bag_count_option = hash_map.get(colour);
        let bag_count: i32 = 
            match bag_count_option {
                Some(x) => *x,
                _ => 0,
            };

        if bag_count < count {
            return false;
        }
    }
    true
}

fn game_score(line: &str, hash_map: &HashMap<&str, i32>) -> i32 {
    let split_line: Vec<&str> = line.split(":").collect();

    let score: i32 = split_line[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();

    let steps: &str = split_line[1];
    let split_steps: Vec<&str> = steps.split(";").collect();
    for step in split_steps {
        let step_valid = is_valid_step(step, hash_map);
        match step_valid {
            true => {},
            false => return 0,
        }
    }
    score
}

fn main() {
    part1();
    part2();
}
